use crate::simple_types::*;

use anyhow::Result;
pub trait ICurrencyLookup {
    fn find_currency(&self, currency_code: CurrencyCode) -> Result<CurrencyDetails>;
}
