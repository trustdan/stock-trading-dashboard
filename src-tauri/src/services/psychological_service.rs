use rusqlite::{Connection, params};
use serde_json::to_string;
use std::error::Error;
use chrono::Utc;

use crate::models::PsychologicalState;

pub fn save_psychological_state(conn: &Connection, state: &mut PsychologicalState) -> Result<i64, Box<dyn Error>> {
    state.update_risk_score();
    state.timestamp = Utc::now();
    
    let extra_factors_json = to_string(&state.extra_factors)?;
    
    conn.execute(
        "INSERT INTO psychological_states 
        (timestamp, gain_loss_yesterday, emotional_state, fomo, market_bias, hunger, headache_pain, extra_factors, total_risk_score) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            state.timestamp.to_rfc3339(),
            state.gain_loss_yesterday,
            state.emotional_state,
            state.fomo,
            state.market_bias,
            state.hunger,
            state.headache_pain,
            extra_factors_json,
            state.total_risk_score
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    state.id = Some(id);
    
    Ok(id)
}

pub fn get_psychological_state(conn: &Connection, id: i64) -> Result<PsychologicalState, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, timestamp, gain_loss_yesterday, emotional_state, fomo, market_bias, 
            hunger, headache_pain, extra_factors, total_risk_score 
        FROM psychological_states 
        WHERE id = ?1"
    )?;
    
    let state = stmt.query_row(params![id], |row| {
        let timestamp_str: String = row.get(1)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let extra_factors_json: String = row.get(8)?;
        let extra_factors = serde_json::from_str(&extra_factors_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))?;
        
        Ok(PsychologicalState {
            id: Some(row.get(0)?),
            timestamp,
            gain_loss_yesterday: row.get(2)?,
            emotional_state: row.get(3)?,
            fomo: row.get(4)?,
            market_bias: row.get(5)?,
            hunger: row.get(6)?,
            headache_pain: row.get(7)?,
            extra_factors,
            total_risk_score: row.get(9)?,
        })
    })?;
    
    Ok(state)
}

pub fn get_recent_psychological_states(conn: &Connection, limit: i64) -> Result<Vec<PsychologicalState>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, timestamp, gain_loss_yesterday, emotional_state, fomo, market_bias, 
            hunger, headache_pain, extra_factors, total_risk_score 
        FROM psychological_states 
        ORDER BY timestamp DESC 
        LIMIT ?1"
    )?;
    
    let states_iter = stmt.query_map(params![limit], |row| {
        let timestamp_str: String = row.get(1)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let extra_factors_json: String = row.get(8)?;
        let extra_factors = serde_json::from_str(&extra_factors_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))?;
        
        Ok(PsychologicalState {
            id: Some(row.get(0)?),
            timestamp,
            gain_loss_yesterday: row.get(2)?,
            emotional_state: row.get(3)?,
            fomo: row.get(4)?,
            market_bias: row.get(5)?,
            hunger: row.get(6)?,
            headache_pain: row.get(7)?,
            extra_factors,
            total_risk_score: row.get(9)?,
        })
    })?;
    
    let mut states = Vec::new();
    for state in states_iter {
        states.push(state?);
    }
    
    Ok(states)
} 