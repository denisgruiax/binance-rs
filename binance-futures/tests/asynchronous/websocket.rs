#[cfg(test)]
mod futures_websocket_api_integration_test {
    use binance_common::{
        enums::WebSocketType,
        error::BinanceError,
        futures::{
            endpoint::host::WebSocketHost,
            model::{
                params::websocket::{WebSocketParams, WebSocketSymbol},
                response::websocket::WebSocketResponse,
            },
        },
    };
    use binance_core::websocket::{
        futures::market::handler::WebSocketMarketHandler, handler::WebSocketHandler,
    };

    use tokio::sync::watch::Receiver;

    #[tokio::test]
    async fn test_websocket_market() {
        let stream = WebSocketParams::new(WebSocketHost::CombinedStreamsMarket).kline_candlesticks(
            WebSocketSymbol::BtcUsdt,
            binance_common::enums::Interval::Minutes5,
        );

        let mut websocket_handler = WebSocketMarketHandler::new(WebSocketType::MultiStream);

        websocket_handler.start(stream.route).await.unwrap();

        let buffer = fill_buffer(websocket_handler.watch().await.unwrap()).await;

        websocket_handler.stop().await.unwrap();

        assert_eq!(buffer.len(), 15);
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
