use exitfailure::ExitFailure;
use crate::models::company_quote::CompanyQuote;

mod models;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let api_key = "YOUR API KEY".to_string();
    let symbol: String = "AAPL".to_string();

    let res = CompanyQuote::get(&symbol, &api_key).await?;

    println!("{}'s current stock price: {}", symbol, res.c);
    Ok(())
}