use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:9002";
    println!("plug emoji here: Connecting to {url}");

    let (ws, _) = connect_async(url).await.unwrap();
    println!("✅ Connected!");

    let (_, mut reader) = ws.split();

    // while let Some(message) = reader.next().await {
    //     if let Ok(msg) = message {
    //         if let Ok(text) = msg.into_text() {
    //             println!("📥 Received: {text}");
    //         }
    //     }
    // }

    #[allow(clippy::collapsible_if)]
    while let Some(message) = reader.next().await {
        if let Ok(msg) = message {
            if let Ok(text) = msg.into_text() {
                println!("📥 Received: {text}");
            }
        }
    }
}
