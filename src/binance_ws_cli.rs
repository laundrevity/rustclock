use futures_util::stream::StreamExt;
use serde_json::Value;
use tokio::net::TcpStream;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade")?;

    // Connect to WebSocket API
    // let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();
    let request = http::Request::builder()
        .uri(url.as_str())
        .header("Sec-WebSocket-Protocol", "rust-websocket")
        .body(())
        .unwrap();
    let addr = url.socket_addrs(|| None)?.pop().ok_or("No address found")?;
    let tcp_stream = TcpStream::connect(addr).await?;
    let tls_connector = tokio_native_tls::TlsConnector::from(native_tls::TlsConnector::new()?);
    let tls_stream = tls_connector
        .connect(url.domain().unwrap(), tcp_stream)
        .await?;
    let (ws_stream, _) = tokio_tungstenite::client_async(request, tls_stream).await?;

    // Read and process message from the websocket
    let (_write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Rcvd: {}", msg);
                    let text = msg.into_text().unwrap();
                    let parsed_data: serde_json::Result<Value> = serde_json::from_str(&text);

                    if let Ok(data) = parsed_data {
                        if let Some(price) = data.get("p") {
                            let price_str = price.as_str().unwrap().to_string();
                            println!("Price: {}", price_str);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}
