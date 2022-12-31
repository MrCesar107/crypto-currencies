fn main() {
    let mut coin: String = String::new();
    let _ = std::io::stdin().read_line(&mut coin).expect("An error ocurred.");
    let result_currency = get_currency(&coin);
    match result_currency {
        Ok(price) => println!("The price of {} is {}", coin, price),
        Err(error) => println!("An error ocurred: {}", error),
    }
}

fn get_currency(coin: &str) -> Result<String, ureq::Error> {
    // Request for the current price of the coin
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{}?localization=false", coin))
        .call()?
        .into_string()?;
    Ok(body)
}
