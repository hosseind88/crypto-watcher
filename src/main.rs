use clap::clap_app;
use cryptowatcher::{
    core::update_data,
    utils::{clear_console, parse_url},
};
use failure::Fallible;
use futures::executor::block_on;
use std::{thread, time::Duration};

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
    (about: "A cryptocurrency price tracker written in Rust.")
    (@arg tokens: +takes_value "you can add aditional tokens, f.e ripple,stellar")
    )
    .get_matches_safe();

    let mut tokens = [
        "bitcoin".to_owned(),
        "ethereum".to_owned(),
        "litecoin".to_owned(),
    ]
    .to_vec();
    let base_url = "https://api.coingecko.com/api/v3/coins/";
    if let Some(tkns) = args.unwrap().value_of("tokens") {
        for split in tkns.split(",") {
            if split != "" {
                tokens.push(split.to_owned());
            }
        }
    }

    let mut urls = Vec::new();
    for item in tokens {
        urls.push(parse_url(&format!("{}{}", base_url, item)).unwrap());
    }

    clear_console();

    println!("Welcome to cryptowatcher 👋\nplease wait a little to initial data be fetched\nthanks for your patience");

    let update = Box::new(move || {
        block_on(update_data(&urls)).unwrap();
        thread::sleep(Duration::from_millis(10000));
    });

    let scheduler = thread::spawn(move || loop {
        let thread = thread::spawn(update.to_owned());
        thread.join().expect("Thread panicked");
    });

    scheduler.join().expect("Scheduler panicked");

    return Ok(());
}
