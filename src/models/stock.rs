use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stock {
    pub ticker: String,
    pub purchased_date: String,
    pub purchase_price: f64,
    pub quantity: i32,
    pub dividend: Option<f64>,
    pub dividend_pay_date: Option<String>,
}

impl Stock {
    pub fn load_stocks() -> std::io::Result<Vec<Stock>> {
        let path = Path::new("stocks.json");

        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(&path)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let stocks: Vec<Stock> = serde_json::from_str(&data)?;
        Ok(stocks)
    }

    pub fn save_stocks(stocks: &Vec<Stock>) -> std::io::Result<()> {
        let data = serde_json::to_string(stocks)?;

        let path = Path::new("stocks.json");
        let mut file = File::create(&path)?;

        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
