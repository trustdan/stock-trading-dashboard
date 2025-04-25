use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TradeStatus {
    Planned,
    Open,
    Closed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub id: Option<i64>,
    pub analysis_id: i64,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub status: TradeStatus,
    pub entry_time: Option<DateTime<Utc>>,
    pub exit_time: Option<DateTime<Utc>>,
    pub entry_price: Option<f64>,
    pub exit_price: Option<f64>,
    pub quantity: u32,
    pub profit_loss: Option<f64>,
    pub percent_return: Option<f64>,
    pub notes: Option<String>,
}

impl Trade {
    pub fn new(symbol: &str, analysis_id: i64) -> Self {
        Self {
            id: None,
            analysis_id,
            timestamp: Utc::now(),
            symbol: symbol.to_string(),
            status: TradeStatus::Planned,
            entry_time: None,
            exit_time: None,
            entry_price: None,
            exit_price: None,
            quantity: 0,
            profit_loss: None,
            percent_return: None,
            notes: None,
        }
    }

    pub fn enter_trade(&mut self, entry_time: DateTime<Utc>, entry_price: f64, quantity: u32) {
        self.entry_time = Some(entry_time);
        self.entry_price = Some(entry_price);
        self.quantity = quantity;
        self.status = TradeStatus::Open;
    }

    pub fn exit_trade(&mut self, exit_time: DateTime<Utc>, exit_price: f64) {
        self.exit_time = Some(exit_time);
        self.exit_price = Some(exit_price);
        self.status = TradeStatus::Closed;
        
        if let Some(entry_price) = self.entry_price {
            self.profit_loss = Some((exit_price - entry_price) * self.quantity as f64);
            self.percent_return = Some((exit_price - entry_price) / entry_price * 100.0);
        }
    }

    pub fn cancel_trade(&mut self, reason: &str) {
        self.status = TradeStatus::Cancelled;
        self.notes = Some(reason.to_string());
    }
} 