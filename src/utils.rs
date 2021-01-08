use crate::core::CoinData;
use console::{self, style, StyledObject, Term};
use prettytable::{Cell, Row, Table};
use url::{ParseError, Url};

pub fn clear_console() {
    let term = Term::stdout();
    term.clear_screen();
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

fn get_coin_color(name: String) -> StyledObject<String> {
    let result = match name.as_ref() {
        "Bitcoin" => style(name).yellow(),
        "Litecoin" => style(name).blue(),
        "Ethereum" => style(name).magenta(),
        _ => style(name).yellow(),
    };
    return result.to_owned();
}

pub fn pretty_print(data: Vec<CoinData>) -> Result<(), ()> {
    clear_console();
    let mut table = Table::new();
    for item in data {
        table.add_row(Row::new(vec![
            Cell::new(&get_coin_color(item.name).to_string()),
            Cell::new(
                &item
                    .market_data
                    .current_price
                    .get("usd")
                    .unwrap()
                    .to_string(),
            ),
            Cell::new(&format!(
                "24h Price Change: {:.4}%",
                item.market_data
                    .price_change_percentage_24h_in_currency
                    .get("usd")
                    .unwrap()
                    .to_string()
            )),
        ]));
    }

    table.printstd();

    return Ok(());
}
