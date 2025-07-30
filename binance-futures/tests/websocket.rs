#[cfg(test)]
mod futures_websocket_api_integration_test {
    use binance_common::{
        enums::{WebSocketCommand, WebSocketType},
        error::BinanceError,
        futures::{
            endpoint::host::WebSocketHost,
            model::{
                params::websocket::{WebSocketParams, WebSocketSymbol},
                response::websocket::WebSocketResponse,
            },
        },
    };
    use binance_futures::websocket::WebSocketConnection;
    use tokio::sync::watch::Receiver;

    #[tokio::test]
    async fn test_websocket_market() {
        let stream = WebSocketParams::new(WebSocketHost::CombinedStreams).kline_candlesticks(
            WebSocketSymbol::BtcUsdt,
            binance_common::enums::Interval::Minutes5,
        );

        let (mut controller, mut websocket) =
            WebSocketConnection::new_market(WebSocketType::MultiStream);

        let websocket_handler = tokio::spawn(async move { websocket.run().await });

        controller
            .send_command(WebSocketCommand::Connect(stream.route))
            .await
            .unwrap();

        let buffer = fill_buffer(controller.watch().await).await;

        controller
            .send_command(WebSocketCommand::Close)
            .await
            .unwrap();

        let websocket_result = websocket_handler.await;

        assert_eq!(buffer.len(), 15);
        assert!(websocket_result.is_ok());
    }

    async fn fill_buffer(
        mut kline_stream: Receiver<Result<WebSocketResponse, BinanceError>>,
    ) -> Vec<WebSocketResponse> {
        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut count = 15;

            while kline_stream.changed().await.is_ok() {
                if count == 0 {
                    break;
                }

                let kline = (*kline_stream.borrow()).as_ref().unwrap().clone();
                println!("{:?}", kline);

                buffer.push(kline);
                count -= 1;
            }

            buffer
        })
        .await
        .unwrap()
    }
}
