fn main() {
    let mut coin: String = String::new();
    let result = std::io::stdin().read_line(&mut coin);
    match result {
        Ok(_) => {
            let currency = get_currency_price(&coin);
            println!("El precio es: {}", currency);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn get_currency_price(_coin: &str) -> String {
    String::from("Test")
}
