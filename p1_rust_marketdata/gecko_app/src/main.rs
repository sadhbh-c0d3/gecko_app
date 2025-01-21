use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use parking_lot::RwLock;
use lazy_static::lazy_static;
use std::sync::Arc; 
use std::fmt::{Debug, Formatter}; 

struct GeckoMarket(String);

impl Debug for GeckoMarket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { 
        write!(f, "GeckoMarket({})", self.0)
    }
}

impl Serialize for GeckoMarket {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> 
    where
        S: Serializer,
    {
        let read_guard = APPLICATION.read();
        let mappings = &read_guard.adapter.read().market_mappings;
        if let Some(original) = mappings.reverse.get(&self.0) {
            serializer.serialize_str(original) 
        } else {
            serializer.serialize_str(&self.0) 
        }
    }
}

impl<'de> Deserialize<'de> for GeckoMarket {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> 
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let read_guard = APPLICATION.read();
        let mappings = &read_guard.adapter.read().market_mappings;
        Ok(GeckoMarket(mappings.forward.get(&s).unwrap_or(&s).to_string()))
    }
}

struct GeckoPrice(f64);

impl Debug for GeckoPrice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { 
        write!(f, "GeckoPrice({})", self.0)
    }
}

impl Serialize for GeckoPrice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for GeckoPrice {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> 
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<f64>()
            .map(GeckoPrice)
            .map_err(serde::de::Error::custom)
    }
}

struct GeckoQuantity(f64); 

impl Debug for GeckoQuantity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { 
        write!(f, "GeckoQuantity({})", self.0)
    }
}

impl Serialize for GeckoQuantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for GeckoQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> 
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<f64>()
            .map(GeckoQuantity)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
enum GeckoSide {
    Ask,
    Bid,
}

impl Serialize for GeckoSide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GeckoSide::Ask => serializer.serialize_str("ask"),
            GeckoSide::Bid => serializer.serialize_str("bid"),
        }
    }
}

impl<'de> Deserialize<'de> for GeckoSide {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ask" => Ok(GeckoSide::Ask),
            "bid" => Ok(GeckoSide::Bid),
            _ => Err(serde::de::Error::custom("Invalid GeckoSide")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GeckoData {
    market: GeckoMarket,
    price: GeckoPrice,
    quantity: GeckoQuantity, 
    side: GeckoSide,
}

// Define a struct to hold the market mappings
struct MarketMappings {
    forward: HashMap<String, String>,
    reverse: HashMap<String, String>,
}

// Rename Config to GeckoConfig
struct GeckoConfig {
    db_host: String,
    db_port: u16,
    markets: Vec<String>, 
}

// Simulate a database connection (replace with actual DB connection logic)
#[derive(Clone, Debug)] // Implement Clone for DbConnection
struct DbConnection {}

// Adapter struct to hold configuration and database connection
struct GeckoAdapter {
    db_connection: DbConnection, 
    market_mappings: MarketMappings, 
}

// Introduce GeckoApplication struct
struct GeckoApplication {
    adapter: Arc<RwLock<GeckoAdapter>>,
}

impl GeckoApplication {
    fn new(config: GeckoConfig) -> Self {
        let db_connection = DbConnection {}; 
        GeckoApplication {
            adapter: Arc::new(RwLock::new(GeckoAdapter {
                db_connection: db_connection.clone(), 
                market_mappings: load_market_mappings(&config, &db_connection), 
            })),
        }
    }
}

fn load_market_mappings(config: &GeckoConfig, db_connection: &DbConnection) -> MarketMappings {
    // This function would typically interact with a database 
    // to fetch the market mappings using db_connection and config.
    // Here, we simulate this with hardcoded values based on config.markets

    let mut forward = HashMap::new();
    let mut reverse = HashMap::new();

    for market in &config.markets {
        let normalized = format!("{}/BTC", market); 
        forward.insert(market.clone(), normalized.clone());
        reverse.insert(normalized, market.clone());
    }

    MarketMappings { forward, reverse }
}

lazy_static! {
    static ref APPLICATION: Arc<RwLock<GeckoApplication>> = Arc::new(RwLock::new(GeckoApplication::new(
        GeckoConfig {
            db_host: "localhost".to_string(), 
            db_port: 5432, 
            markets: vec!["ARBTC".to_string(), "ETHBTC".to_string()], // Example markets
        },
    )));
}

#[test]
fn test_marketdata() -> Result<(), serde_json::Error> {
    let json_str = r#"
        {
            "market": "ARBTC",
            "price": "0.0012",
            "quantity": "10.5",
            "side": "bid" 
        }
    "#;

    let data: GeckoData = serde_json::from_str(json_str)?;

    assert_eq!(data.market.0, "ARBTC/BTC"); 
    assert!((data.price.0 - 0.0012).abs() < f64::EPSILON); 
    assert!((data.quantity.0 - 10.5).abs() < f64::EPSILON); 
    assert!(match data.side { GeckoSide::Bid => true, _ => false });

    Ok(())
}

fn main() {
    println!("Hello!"); 
}