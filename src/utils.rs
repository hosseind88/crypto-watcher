use url::{ParseError, Url};
use console::style;

use crate::core::CoinData;

pub fn clear_console() {
    // clear console
    print!("{}[2J", 27 as char);
}

pub fn parse_url(url: &str) -> Result<Url, ParseError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) if error == ParseError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "http://", url);
            Url::parse(url_with_base.as_str())
        }
        Err(error) => Err(error),
    }
}

pub fn pretty_print(data: &CoinData) {
    let result = format!("bitcoin price now: {}", data.market_data.current_price.get("usd").unwrap());
    println!("{}", style(result).cyan());
}