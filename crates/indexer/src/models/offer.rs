//! Offer model for SDEX offers

use chrono::{DateTime, Utc};

use super::{asset::Asset, horizon::HorizonOffer};
use crate::error::{IndexerError, Result};

/// Normalized offer from SDEX
#[derive(Debug, Clone)]
pub struct Offer {
    pub id: u64,
    pub seller: String,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: String,
    pub price_n: i32,
    pub price_d: i32,
    pub price: String,
    pub last_modified_ledger: u64,
    pub last_modified_time: Option<DateTime<Utc>>,
}

impl TryFrom<HorizonOffer> for Offer {
    type Error = IndexerError;

    fn try_from(horizon_offer: HorizonOffer) -> Result<Self> {
        let id = horizon_offer
            .id
            .parse::<u64>()
            .map_err(|_| IndexerError::StellarApi(format!("Invalid offer ID: {}", horizon_offer.id)))?;

        // Parse assets using the client's parse_asset method
        // We'll need to pass the client or make parse_asset a standalone function
        // For now, let's create a helper function
        let selling = parse_asset_from_value(&horizon_offer.selling)?;
        let buying = parse_asset_from_value(&horizon_offer.buying)?;

        let price_n = horizon_offer
            .price_r
            .as_ref()
            .map(|r| r.n as i32)
            .unwrap_or(0);
        let price_d = horizon_offer
            .price_r
            .as_ref()
            .map(|r| r.d as i32)
            .unwrap_or(1);

        Ok(Offer {
            id,
            seller: horizon_offer.seller,
            selling,
            buying,
            amount: horizon_offer.amount,
            price_n,
            price_d,
            price: horizon_offer.price,
            last_modified_ledger: horizon_offer.last_modified_ledger as u64,
            last_modified_time: None, // Horizon doesn't provide this directly
        })
    }
}

fn parse_asset_from_value(v: &serde_json::Value) -> Result<Asset> {
    let asset_type = v
        .get("asset_type")
        .and_then(|x| x.as_str())
        .ok_or_else(|| IndexerError::StellarApi("missing asset_type".to_string()))?;

    match asset_type {
        "native" => Ok(Asset::Native),
        "credit_alphanum4" => Ok(Asset::CreditAlphanum4 {
            asset_code: v
                .get("asset_code")
                .and_then(|x| x.as_str())
                .ok_or_else(|| IndexerError::StellarApi("missing asset_code".to_string()))?
                .to_string(),
            asset_issuer: v
                .get("asset_issuer")
                .and_then(|x| x.as_str())
                .ok_or_else(|| IndexerError::StellarApi("missing asset_issuer".to_string()))?
                .to_string(),
        }),
        "credit_alphanum12" => Ok(Asset::CreditAlphanum12 {
            asset_code: v
                .get("asset_code")
                .and_then(|x| x.as_str())
                .ok_or_else(|| IndexerError::StellarApi("missing asset_code".to_string()))?
                .to_string(),
            asset_issuer: v
                .get("asset_issuer")
                .and_then(|x| x.as_str())
                .ok_or_else(|| IndexerError::StellarApi("missing asset_issuer".to_string()))?
                .to_string(),
        }),
        other => Err(IndexerError::StellarApi(format!("unknown asset_type: {other}"))),
    }
}
