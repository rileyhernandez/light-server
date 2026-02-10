use rumqttc::{MqttOptions, AsyncClient, QoS, Event, Packet};
use tokio::task;
use std::time::Duration;
use std::error::Error;

async fn announce_nodes(client: &AsyncClient) -> Result<(), Box<dyn Error>> {
    for i in 0..3 {
        let topic = format!("stat/node-{i}/power");
        client.publish(topic, QoS::AtLeastOnce, true, "OFF").await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new("mcu-simulator", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("cmd/+/power", QoS::AtLeastOnce).await?;

    announce_nodes(&client).await?;

    task::spawn(async move {
        loop {
            let notification = eventloop.poll().await;
            match notification {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    let topic = publish.topic.to_string();
                    let payload = publish.payload.to_vec();
                    println!("Received command on topic: {topic}");
        
                    let response_topic = topic.replace("cmd", "stat");
                    let response_payload = payload;
        
                    if let Err(e) = client.publish(response_topic.clone(), QoS::AtLeastOnce, false, response_payload).await {
                        eprintln!("Error publishing response: {e:?}");
                    }
                    println!("Published status to topic: {response_topic}");
                }
                Err(e) => {
                    eprintln!("Error in event loop: {e:?}");
                    break;
                }
                _ => {}
            }
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
