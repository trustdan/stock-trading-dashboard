use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PsychologicalState {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub gain_loss_yesterday: f64,
    pub emotional_state: i32,    // -3 to +3
    pub fomo: i32,               // -3 to +3
    pub market_bias: i32,        // -3 to +3
    pub hunger: i32,             // 0 to +3
    pub headache_pain: i32,      // 0 to +3
    pub extra_factors: HashMap<String, i32>,
    pub total_risk_score: f64,
}

impl PsychologicalState {
    pub fn new() -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            gain_loss_yesterday: 0.0,
            emotional_state: 0,
            fomo: 0,
            market_bias: 0,
            hunger: 0,
            headache_pain: 0,
            extra_factors: HashMap::new(),
            total_risk_score: 0.0,
        }
    }

    pub fn calculate_total_risk(&self) -> f64 {
        let base = self.emotional_state + self.fomo + self.market_bias;
        let physical = self.hunger + self.headache_pain;
        let pnl_effect = (self.gain_loss_yesterday / 100.0).clamp(-1.0, 1.0) * 3.0;
        let extras: i32 = self.extra_factors.values().sum();
        (base as f64 + physical as f64 + pnl_effect + extras as f64) / 3.0
    }

    pub fn update_risk_score(&mut self) {
        self.total_risk_score = self.calculate_total_risk();
    }
}

impl Default for PsychologicalState {
    fn default() -> Self {
        Self::new()
    }
} 