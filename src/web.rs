use embassy_net::Stack;
use embassy_time::Duration;
use esp_alloc as _;
use heapless::String;
use picoserve::{
    extract,
    response::{json, File},
    routing::{self, get, post},
    AppBuilder, AppRouter, Router,
};

use crate::{
    app::{self, SensorValues, CONFIG, VALUE_HISTORY},
    cors_layer::CorsLayer,
    peripheral_tasks::{RISK_SIGNAL, SENSOR_VALS_SIGNAL},
};

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
                    let mut state = CONFIG.lock().await;

                    *state = config
                })
                .get(|| async {
                    let config = CONFIG.lock().await.clone();

                    let json_value: json::Json<app::Config> = picoserve::extract::Json(config);

                    json_value
                }),
            )
            .route(
                "/values",
                get(|| async {
                    let mut value_history = VALUE_HISTORY.lock().await;

                    let vals_info = SensorValuesInfo {
                        sensor_values: value_history.current_values(),
                        has_changed: value_history.new_change(),
                    };

                    drop(value_history);

                    vals_info.to_string()
                }),
            )
            .route(
                "/values/now",
                get(|| async {
                    let real_time_values = SENSOR_VALS_SIGNAL.wait().await;

                    real_time_values.to_string()
                }),
            )
            .route(
                "/values/history",
                get(|| async {
                    let value_history = VALUE_HISTORY.lock().await.get_current_values_history();

                    value_history.to_string()
                }),
            )
        //.layer(CorsLayer)
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

struct SensorValuesInfo {
    sensor_values: SensorValues,
    has_changed: bool,
}

impl SensorValuesInfo {
    fn to_string(self) -> String<14> {
        let sensor_values_string = self.sensor_values.to_string();
        let mut sensor_info_string = String::new();

        sensor_info_string
            .push_str(sensor_values_string.as_str())
            .unwrap();

        sensor_info_string.push(' ').unwrap();

        if self.has_changed {
            sensor_info_string.push('1').unwrap();
        } else {
            sensor_info_string.push('0').unwrap();
        }

        sensor_info_string
    }
}
