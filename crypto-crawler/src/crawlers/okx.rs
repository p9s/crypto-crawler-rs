use super::utils::fetch_symbols_retry;
use crate::{crawlers::utils::create_conversion_thread, msg::Message};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;
use std::sync::mpsc::Sender;

const EXCHANGE_NAME: &str = "okx";

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    let tx = create_conversion_thread(
        EXCHANGE_NAME.to_string(),
        MessageType::FundingRate,
        market_type,
        tx,
    );

    let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        fetch_symbols_retry(EXCHANGE_NAME, market_type)
    } else {
        symbols.unwrap().to_vec()
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!("funding-rate:{}", symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_client = OkxWSClient::new(tx, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("OKX {} does NOT have funding rates", market_type),
    }
}

pub(crate) fn crawl_open_interest(
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    let tx = create_conversion_thread(
        EXCHANGE_NAME.to_string(),
        MessageType::OpenInterest,
        market_type,
        tx,
    );

    let symbols = if let Some(symbols) = symbols {
        if symbols.is_empty() {
            fetch_symbols_retry(EXCHANGE_NAME, market_type)
        } else {
            symbols.to_vec()
        }
    } else {
        fetch_symbols_retry(EXCHANGE_NAME, market_type)
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!("open-interest:{}", symbol))
        .collect();

    if market_type != MarketType::Spot {
        let ws_client = OkxWSClient::new(tx, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
    } else {
        panic!("spot does NOT have open interest");
    }
}