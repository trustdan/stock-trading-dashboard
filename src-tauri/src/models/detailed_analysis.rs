use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::stock_rating::{MarketTrend, ChartPattern};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailedAnalysis {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub bull_bear: i8,             // Bull = +1, Bear = -1
    pub confidence: u8,            // 0 to 100%
    pub market_trend: MarketTrend,
    pub chart_pattern: ChartPattern,
    pub strategy: String,
    pub overall_score: i32,
    pub market_sentiment: i32,     // -3 to +3
    pub sector_sentiment: i32,     // -3 to +3
    pub sector: String,
    pub security: String,
    pub bought: bool,
    pub entry_reason: String,
    pub time: DateTime<Utc>,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub target_price: f64,
    pub short_leg: Option<String>,
    pub long_leg: Option<String>,
    pub debit_credit: f64,
    pub quantity: u32,
    pub risk_max: f64,
    pub reward: f64,
    pub max_gain: Option<f64>,
    pub percent_profit: Option<f64>,
    pub delta: Option<f64>,
    pub theta: Option<f64>,
    pub gamma: Option<f64>,
    pub vega: Option<f64>,
    pub alerts: Vec<String>,
    pub exit_reason: Option<String>,
    pub skip_reason: Option<String>,
}

impl DetailedAnalysis {
    pub fn new(security: &str, sector: &str) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            bull_bear: 1,
            confidence: 50,
            market_trend: MarketTrend::Uncertain,
            chart_pattern: ChartPattern::Other("None".to_string()),
            strategy: String::new(),
            overall_score: 0,
            market_sentiment: 0,
            sector_sentiment: 0,
            sector: sector.to_string(),
            security: security.to_string(),
            bought: false,
            entry_reason: String::new(),
            time: Utc::now(),
            entry_price: 0.0,
            stop_loss: 0.0,
            target_price: 0.0,
            short_leg: None,
            long_leg: None,
            debit_credit: 0.0,
            quantity: 0,
            risk_max: 0.0,
            reward: 0.0,
            max_gain: None,
            percent_profit: None,
            delta: None,
            theta: None,
            gamma: None,
            vega: None,
            alerts: Vec::new(),
            exit_reason: None,
            skip_reason: None,
        }
    }

    pub fn calculate_risk_reward(&mut self) {
        if self.bought {
            if self.entry_price > 0.0 && self.stop_loss > 0.0 {
                self.risk_max = (self.entry_price - self.stop_loss).abs() * self.quantity as f64;
                
                if self.target_price > 0.0 {
                    self.reward = (self.target_price - self.entry_price).abs() * self.quantity as f64;
                }
            }
        } else {
            // For options or other instruments with fixed debit/credit
            self.risk_max = self.debit_credit.abs() * self.quantity as f64;
        }
    }

    pub fn update_profit(&mut self, exit_price: f64, _exit_time: DateTime<Utc>) {
        if self.bought {
            self.max_gain = Some((exit_price - self.entry_price) * self.quantity as f64);
            if self.entry_price > 0.0 {
                self.percent_profit = Some((exit_price - self.entry_price) / self.entry_price * 100.0);
            }
        } else {
            // For options or other instruments
            self.max_gain = Some(exit_price * self.quantity as f64);
            if self.debit_credit != 0.0 {
                self.percent_profit = Some((exit_price - self.debit_credit) / self.debit_credit.abs() * 100.0);
            }
        }
    }
} 