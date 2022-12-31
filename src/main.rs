use serde::{Deserialize, Serialize};

fn main() {
    let mut coin: String = String::new();
    println!("Enter the coin you want to know the price of: ");
    let _ = std::io::stdin().read_line(&mut coin).expect("An error ocurred.");
    let result_currency = get_currency(&coin);
    match result_currency {
        Ok(price) => println!("The price of {} is ${}", coin, price),
        Err(error) => println!("An error ocurred: {}", error),
    }
}

fn get_currency(coin: &str) -> Result<String, ureq::Error> {
    // Request for the current price of the coin
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{}?localization=false", coin))
        .call()?
        .into_string()?;
    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    Ok(coin_data.market_data.current_price.usd.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
    mxn: f32,
}
