use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;

/// Scrape the “data-buy” rate for each currency code from the “参考汇率” dropdown.
pub async fn currency_scraper(
    codes: &[String],
) -> Result<HashMap<String, Decimal>, Box<dyn Error + Send + Sync>> {
    // 1) fetch the page
    let url = "https://www.cathaybk.com.tw/cathaybk/personal/product/deposit/currency-billboard/";
    let body = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&body);

    // 2) find all <select data-exchange-currency>
    let select_sel = Selector::parse("select[data-exchange-currency]").unwrap();
    let option_sel = Selector::parse("option").unwrap();

    let all_selects: Vec<_> = document.select(&select_sel).collect();

    // try to pick the second one (output) if exists, else fallback to first
    let select_el = all_selects.get(1).or_else(|| all_selects.get(0));
    if select_el.is_none() {
        eprintln!("[currency_scraper] ERROR: no select[data-exchange-currency] found");
    }

    let mut map = HashMap::new();
    if let Some(sel) = select_el {
        for opt in sel.select(&option_sel) {
            let code = opt.value().attr("value").unwrap_or_default();
            let buy_str = opt.value().attr("data-buy").unwrap_or_default();
            if codes.iter().any(|c| c == &code) {
                if let Ok(rate) = Decimal::from_str(buy_str) {
                    map.insert(code.to_string(), rate);
                }
            }
        }
    }

    Ok(map)
}
