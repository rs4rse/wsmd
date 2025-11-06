use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use serde_json::json;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:9002").await.unwrap();
    println!("✅ Server running on ws://127.0.0.1:9002");

    loop {
        let (stream, _) = server.accept().await.unwrap();
        tokio::spawn(async move {
            let ws = accept_async(stream)
                .await
                .expect("not able to make a ws stream from tcp stream, why??");
            let (mut tx, _) = ws.split();

            loop {
                let value: f32 = rand::rng().random_range(0.0..100.0);
                let msg = json!({ "value": value }).to_string();
                dbg!(&msg);

                tx.send(tokio_tungstenite::tungstenite::Message::Text(msg.into()))
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
    }
}
