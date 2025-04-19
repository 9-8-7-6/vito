/// Represents basic metadata for a stock/security in a specific market.
///
/// This struct is used to normalize stock listing data retrieved from different APIs
/// (e.g., Taiwan Stock Exchange, US markets) into a consistent format.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// ISO country code (e.g., "TW", "US")
    pub country: String,

    /// Ticker symbol of the stock (e.g., "2330", "AAPL")
    pub ticker_symbol: String,

    /// Official company name associated with the ticker
    pub company_name: String,
}
