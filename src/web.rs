use embassy_net::Stack;
use embassy_time::Duration;
use esp_alloc as _;
use esp_println::println;
use picoserve::{
    extract,
    response::{json, File},
    routing::{self, get, post},
    AppBuilder, AppRouter, Router,
};
use serde::Serialize;

use crate::app::{self, SensorValues, ValueHistoryArray, APP_STATE};

pub struct Application;

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("iot_dashboard/dist/index.html"))),
            )
            .route(
                "/index.css",
                routing::get_service(File::css(include_str!("iot_dashboard/dist/index.css"))),
            )
            .route(
                "/index.js",
                routing::get_service(File::javascript(include_str!(
                    "iot_dashboard/dist/index.js"
                ))),
            )
            .route(
                "/config",
                post(|extract::Json::<app::Config>(config)| async move {
                    println!("{:#?}", config);
                })
                .get(|| async {
                    let state = APP_STATE.lock().await;

                    let config = state.config.clone();

                    let json_value: json::Json<app::Config> = picoserve::extract::Json(config);

                    json_value
                }),
            )
            .route(
                "/values",
                get(|| async {
                    let mut state = APP_STATE.lock().await;

                    let vals = state.value_history.current_values();

                    #[derive(Serialize)]
                    struct SensorValuesInfo {
                        sensor_values: SensorValues,
                        has_changed: bool,
                    }

                    let json_value: json::Json<SensorValuesInfo> =
                        picoserve::extract::Json(SensorValuesInfo {
                            sensor_values: vals,
                            has_changed: state.value_history.new_change(),
                        });

                    json_value
                }),
            )
            .route(
                "/values/history",
                get(|| async {
                    let state = APP_STATE.lock().await;

                    let vals = state.value_history.get_current_values_history();
                    let json_value: json::Json<ValueHistoryArray> = picoserve::extract::Json(vals);

                    json_value
                }),
            )
    }
}

pub const WEB_TASK_POOL_SIZE: usize = 2;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        router,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

pub struct WebApp {
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());

        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        Self { router, config }
    }
}
