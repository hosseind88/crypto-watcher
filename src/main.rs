use clap::{clap_app, crate_version};
use console::style;
use cryptowatcher::{core::update_data, utils::{clear_console, parse_url}};
use failure::{format_err, Fallible};
use regex::Regex;
use std::{thread, time::Duration};
use cryptowatcher::core::{ get_data };
use futures::executor::block_on;
use tokio;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            //process::exit(1);
        }
    }
}

fn run() -> Fallible<()> {
    let args = clap_app!(cryptowatcher =>
    (author: "Hossein Dindar <hosseind2017@gmail.com>")
    (about: "A downloader written in Rust.")
    (@arg tokens: +takes_value "tokens")
    )
    .get_matches_safe();

    let mut tokens: Vec<&str> = ["bitcoin", "litecoin"].to_vec();

    if let Some(tkns) = args.unwrap().value_of("tokens") {
        let seperator = Regex::new(r"\[|\]").expect("Invalid regex");
        let splits: Vec<_> = seperator.split(tkns).into_iter().collect();
        for split in splits {
            if split != "" {
                tokens.push(split);
            }
        }
    }

    println!("This is {} neat", style("quite").cyan());

    fn update() -> Result<(), ()> {
        let url = parse_url("https://api.coingecko.com/api/v3/coins/bitcoin").unwrap();
        block_on(update_data(&url));
        thread::sleep(Duration::from_millis(1000));

        return Ok(());
    }

    let scheduler = thread::spawn(|| loop {
        let thread = thread::spawn(update);
        thread.join().expect("Thread panicked");
    });

    scheduler.join().expect("Scheduler panicked");

    return Ok(());
}
