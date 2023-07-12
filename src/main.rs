mod cli;
mod models;

use clap::{Arg, App, SubCommand};
use cli::{add, view};

fn main() {
    let app_cli = App::new("Stock Tracker")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Tracks stock purchases and dividends")
        .subcommand(SubCommand::with_name("add")
            .about("Adds a new stock")
            .arg(Arg::with_name("ticker").short('t').long("ticker").required(true).takes_value(true))
            .arg(Arg::with_name("purchased_date").short('d').long("date").required(true).takes_value(true))
            .arg(Arg::with_name("purchase_price").short('p').long("price").required(true).takes_value(true))
            .arg(Arg::with_name("quantity").short('q').long("quantity").required(true).takes_value(true))
            .arg(Arg::with_name("dividend").short('i').long("dividend").required(true).takes_value(true))
            .arg(Arg::with_name("dividend_pay_date").short('y').long("divpaydate").required(false).takes_value(true)))
        .subcommand(SubCommand::with_name("view")
            .about("Displays all stocks"))
        .get_matches();

    match app_cli.subcommand() {
        Some(("add", add_matches)) => {
            if let Err(e) = add(add_matches) {
                eprintln!("Error adding stock: {}", e);
            }
        },
        Some(("view", _)) => {
            if let Err(e) = view() {
                eprintln!("Error viewing stocks: {}", e);
            }
        },
        _ => {
            eprintln!("Invalid command. Use add to add a new stock and view to display all stocks.");
        },
    }
}
