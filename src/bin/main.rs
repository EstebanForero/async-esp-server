#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_hal::clock::CpuClock;
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;

#[panic_handler]
fn panic(err: &core::panic::PanicInfo) -> ! {
    println!("{err:?}");
    loop {}
}

extern crate alloc;

use async_esp_server as lib;
use esp_wifi::EspWifiController;
use lib::peripheral_tasks::*;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    println!("Embassy initialized");

    let timer1 = TimerGroup::new(peripherals.TIMG0);

    let rng = Rng::new(peripherals.RNG);
    let esp_wifi_ctrl = &*lib::mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng, peripherals.RADIO_CLK).unwrap()
    );

    let stack = lib::wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner).await;

    let web_app = lib::web::WebApp::default();

    for id in 0..lib::web::WEB_TASK_POOL_SIZE {
        spawner.must_spawn(lib::web::web_task(
            id,
            stack,
            web_app.router,
            web_app.config,
        ));
    }

    println!("Web server started");

    //spawner.must_spawn(test_load());

    spawner.must_spawn(sensor_reader_task(
        peripherals.GPIO15,
        peripherals.ADC1,
        peripherals.GPIO34,
        peripherals.GPIO19,
    ));
    spawner.must_spawn(display_task(
        peripherals.I2C0.into(),
        peripherals.GPIO18,
        peripherals.GPIO23,
    ));
    spawner.must_spawn(alarms_task(
        peripherals.GPIO12,
        peripherals.GPIO13,
        peripherals.GPIO14,
        peripherals.GPIO27
    ));
}
