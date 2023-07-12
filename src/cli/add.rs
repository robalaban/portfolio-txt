use crate::models::Stock;
use std::io;
use clap::ArgMatches;

pub fn add(matches: &ArgMatches) -> io::Result<()> {
    let mut stocks = Stock::load_stocks()?;

    let ticker = matches.value_of("ticker").unwrap().to_string();
    let purchased_date = matches.value_of("purchased_date").unwrap().to_string();
    let purchase_price: f64 = matches.value_of("purchase_price").unwrap().parse().unwrap();
    let quantity: i32 = matches.value_of("quantity").unwrap().parse().unwrap();
    let dividend: f64 = matches.value_of("dividend").unwrap().parse().unwrap();
    let dividend_pay_date = matches.value_of("dividend_pay_date").map(String::from);

    let new_stock = Stock {
        ticker,
        purchased_date,
        purchase_price,
        quantity,
        dividend,
        dividend_pay_date,
    };

    stocks.push(new_stock);

    Stock::save_stocks(&stocks)
}
