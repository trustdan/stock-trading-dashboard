use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarketTrend {
    Uptrend,
    Downtrend,
    Sideways,
    Uncertain,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChartPattern {
    HighBase,
    LowBase,
    AscendingTriangle,
    DescendingTriangle,
    Cup,
    HeadAndShoulders,
    InverseHeadAndShoulders,
    DoubleTop,
    DoubleBottom,
    Consolidation,
    BreakoutPullback,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockRating {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub security_name: Option<String>,
    pub sector: String,
    pub market_sentiment: i32,     // -3 to +3
    pub sector_sentiment: i32,     // -3 to +3
    pub security_sentiment: i32,   // -3 to +3
    pub bull_bear: i8,             // Bull = +1, Bear = -1
    pub confidence: u8,            // 0 to 100%
    pub market_trend: MarketTrend,
    pub chart_pattern: ChartPattern,
    pub strategy: String,
    pub overall_score: i32,        // Computed
    pub notes: Option<String>,
}

impl StockRating {
    pub fn new(symbol: &str, sector: &str) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            symbol: symbol.to_string(),
            security_name: None,
            sector: sector.to_string(),
            market_sentiment: 0,
            sector_sentiment: 0,
            security_sentiment: 0,
            bull_bear: 1,
            confidence: 50,
            market_trend: MarketTrend::Uncertain,
            chart_pattern: ChartPattern::Other("None".to_string()),
            strategy: String::new(),
            overall_score: 0,
            notes: None,
        }
    }

    pub fn calculate_overall_score(&self) -> i32 {
        let bull_bear_factor = self.bull_bear as i32;
        let confidence_factor = (self.confidence as f32 / 100.0) * 3.0;
        let sentiment_sum = self.market_sentiment + self.sector_sentiment + self.security_sentiment;
        
        bull_bear_factor * (sentiment_sum + confidence_factor as i32)
    }

    pub fn update_overall_score(&mut self) {
        self.overall_score = self.calculate_overall_score();
    }
} 