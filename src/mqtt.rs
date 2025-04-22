use crate::{
    app::{Config, CONFIG},
    peripheral_tasks::{RISK_SIGNAL, SENSOR_VALS_SIGNAL},
};
use core::net::Ipv4Addr;
use embassy_futures::select::{select3, Either3};
use embassy_net::{tcp::TcpSocket, Stack};
use embassy_time::Duration;
use esp_println::println;
use rust_mqtt::{
    client::{client::MqttClient, client_config::ClientConfig},
    packet::v5::publish_packet::QualityOfService,
    utils::rng_generator::CountingRng,
};

#[embassy_executor::task]
pub async fn mqtt_task(stack: Stack<'static>) {
    loop {
        let mut rng = CountingRng(20000);

        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];
        let mut mqtt_recv_buffer = [0; 80];
        let mut mqtt_write_buffer = [0; 80];

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        //192.168.0.117
        let address = Ipv4Addr::new(192, 168, 0, 117);
        if let Err(e) = socket.connect((address, 1883)).await {
            println!("Failed to connect to MQTT broker: {:?}", e);
            embassy_time::Timer::after(Duration::from_secs(5)).await;
            continue;
        }

        // Explicitly annotate the type
        let mut config: ClientConfig<'_, 5, CountingRng> =
            ClientConfig::new(rust_mqtt::client::client_config::MqttVersion::MQTTv5, rng);
        config.add_client_id("mydevice-client");
        config.max_packet_size = 256;
        config.add_max_subscribe_qos(QualityOfService::QoS1);

        let mut client = MqttClient::new(
            socket,
            &mut mqtt_write_buffer,
            80,
            &mut mqtt_recv_buffer,
            80,
            config,
        );

        if let Err(e) = client.connect_to_broker().await {
            println!("Failed to connect to MQTT broker: {:?}", e);
            embassy_time::Timer::after(Duration::from_secs(5)).await;
            continue;
        }

        if let Err(e) = client.subscribe_to_topic("config/set").await {
            println!("Failed to subscribe: {:?}", e);
            continue;
        }

        loop {
            match select3(
                SENSOR_VALS_SIGNAL.wait(),
                RISK_SIGNAL.wait(),
                client.receive_message(),
            )
            .await
            {
                Either3::First(sensor_values) => {
                    let bytes = sensor_values.to_bytes();
                    if let Err(e) = client
                        .send_message("sensors", &bytes, QualityOfService::QoS1, true)
                        .await
                    {
                        println!("Failed to send sensor values: {:?}", e);
                        break;
                    }
                }
                Either3::Second(risk) => {
                    let risk_byte = risk.to_byte();
                    if let Err(e) = client
                        .send_message("risk", &[risk_byte], QualityOfService::QoS1, true)
                        .await
                    {
                        println!("Failed to send risk: {:?}", e);
                        break;
                    }
                }
                Either3::Third(Ok((topic, payload))) => {
                    if topic == "config/set" {
                        if payload.len() == 6 {
                            let new_config = Config::from_bytes(payload.try_into().unwrap());
                            *CONFIG.lock().await = new_config.clone();
                            let bytes = new_config.to_bytes();
                            if let Err(e) = client
                                .send_message(
                                    "mydevice/config",
                                    &bytes,
                                    QualityOfService::QoS1,
                                    true,
                                )
                                .await
                            {
                                println!("Config update publish failed: {:?}", e);
                                break;
                            }
                        } else {
                            println!("Invalid config payload length");
                        }
                    }
                }
                Either3::Third(Err(e)) => {
                    println!("MQTT receive error: {:?}", e);
                    break;
                }
            }
        }

        embassy_time::Timer::after(Duration::from_secs(5)).await;
    }
}

// Example usage in your main function
/*
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let stack = // ... initialize your network stack here, likely using mk_static!
    spawner.spawn(mqtt_task(stack)).ok();
}
*/
