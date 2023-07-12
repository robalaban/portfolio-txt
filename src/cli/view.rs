use crate::models::Stock;

pub fn view() -> Result<(), Box<dyn std::error::Error>> {
    let stocks = Stock::load_stocks()?;

    for stock in stocks {
        println!("{:?}", stock);
    }

    Ok(())
}
