use serde::Deserialize;
use reqwest::Client;
use std::env;


#[derive(Deserialize, Debug)]
struct ExchangeATicker {
    price: String,
}

#[derive(Deserialize, Debug)]
struct ExchangeBTicker {
    last: String,
}

async fn fetch_price_exchange_a(client: &Client) -> Result<f64, Box<dyn std::error::Error>> {
    let resp = client
        .get("https://api.binance.com/api/v3/exchangeInfo?symbol=USDBTC")
        .send()
        .await?
        .json::<ExchangeATicker>()
        .await?;
    Ok(resp.price.parse()?)
}

async fn fetch_price_exchange_b(client: &Client) -> Result<f64, Box<dyn std::error::Error>> {
    let resp = client
        .get("https://api.exchangeB.com/v1/ticker/BTCUSD")
        .send()
        .await?
        .json::<ExchangeBTicker>()
        .await?;
    Ok(resp.last.parse()?)
}

async fn check_arbitrage(price_a: f64, price_b: f64) {
    let threshold = 100.0;

    // Here i could use match case
    if (price_a - price_b).abs() > threshold {
        if price_a > price_b {
            println!("Buy on Exchange B and sell on Exchange A");
            // execute_trade("buy", "ExchangeB", price_b).await;
            // execute_trade("sell", "ExchangeA", price_a).await;
        } else {
            println!("Buy on Exchange A and sell on Exchange B");
            // execute_trade("buy", "ExchangeA", price_a).await;
            // execute_trade("sell", "ExchangeB", price_b).await;
        }
    } else {
        println!("No arbitrage opportunity detected.");
    }
}

async fn execute_trade(
    side: &str,
    exchange: &str,
    price: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // To be implemented
    println!("Executing {} on {} at price {}", side, exchange, price);

    Ok(())
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let exchange_a_api_key = env::var("EXCHANGE_A_API_KEY").unwrap();
    let exchange_b_api_key = env::var("EXCHANGE_B_API_KEY").unwrap();

    loop {
        let price_a = fetch_price_exchange_a(&client).await.unwrap_or(0.0);
        let price_b = fetch_price_exchange_b(&client).await.unwrap_or(0.0);

        if let Err(e) = check_arbitrage(price_a, price_b).await {
            eprintln!("Error checking arbitrage: {}", e);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

