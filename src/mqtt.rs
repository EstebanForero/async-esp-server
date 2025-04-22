// Example MQTT task (assuming additional dependencies and signals are defined elsewhere)

#[embassy_executor::task]
pub async fn mqtt_task(mut client: MqttClient) {
    use embassy_futures::select::{select3, Either3};
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
                    .send_message("mydevice/sensors", &bytes, QoS::AtLeastOnce, true)
                    .await
                {
                    println!("Failed to send sensor values: {:?}", e);
                }
            }
            Either3::Second(risk) => {
                let risk_byte = risk.to_byte();
                if let Err(e) = client
                    .send_message("mydevice/risk", &[risk_byte], QoS::AtLeastOnce, true)
                    .await
                {
                    println!("Failed to send risk: {:?}", e);
                }
            }
            Either3::Third(Ok(msg)) => {
                if msg.topic == "mydevice/config/set" {
                    if msg.payload.len() == 6 {
                        let new_config = Config::from_bytes(msg.payload.try_into().unwrap());
                        *CONFIG.lock().await = new_config.clone();
                        let bytes = new_config.to_bytes();
                        if let Err(e) = client
                            .send_message("mydevice/config", &bytes, QoS::AtLeastOnce, true)
                            .await
                        {
                            println!("Config update publish failed: {:?}", e);
                        }
                    } else {
                        println!("Invalid config payload length");
                    }
                }
            }
            Either3::Third(Err(e)) => {
                println!("MQTT receive error: {:?}", e);
            }
        }
    }
}
