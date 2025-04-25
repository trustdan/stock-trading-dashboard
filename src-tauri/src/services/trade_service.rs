use rusqlite::{Connection, params};
use serde_json::to_string;
use std::error::Error;
use chrono::Utc;

use crate::models::trade::{Trade, TradeStatus};

pub fn save_trade(conn: &Connection, trade: &mut Trade) -> Result<i64, Box<dyn Error>> {
    trade.timestamp = Utc::now();
    
    let status_json = to_string(&trade.status)?;
    
    conn.execute(
        "INSERT INTO trades 
        (analysis_id, timestamp, symbol, status, entry_time, exit_time, entry_price, exit_price, 
        quantity, profit_loss, percent_return, notes) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            trade.analysis_id,
            trade.timestamp.to_rfc3339(),
            trade.symbol,
            status_json,
            trade.entry_time.map(|dt| dt.to_rfc3339()),
            trade.exit_time.map(|dt| dt.to_rfc3339()),
            trade.entry_price,
            trade.exit_price,
            trade.quantity,
            trade.profit_loss,
            trade.percent_return,
            trade.notes,
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    trade.id = Some(id);
    
    Ok(id)
}

pub fn update_trade(conn: &Connection, trade: &Trade) -> Result<(), Box<dyn Error>> {
    let status_json = to_string(&trade.status)?;
    
    conn.execute(
        "UPDATE trades 
        SET status = ?1, entry_time = ?2, exit_time = ?3, entry_price = ?4, exit_price = ?5,
        quantity = ?6, profit_loss = ?7, percent_return = ?8, notes = ?9
        WHERE id = ?10",
        params![
            status_json,
            trade.entry_time.map(|dt| dt.to_rfc3339()),
            trade.exit_time.map(|dt| dt.to_rfc3339()),
            trade.entry_price,
            trade.exit_price,
            trade.quantity,
            trade.profit_loss,
            trade.percent_return,
            trade.notes,
            trade.id,
        ],
    )?;
    
    Ok(())
}

pub fn get_trade(conn: &Connection, id: i64) -> Result<Trade, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, analysis_id, timestamp, symbol, status, entry_time, exit_time, entry_price, 
            exit_price, quantity, profit_loss, percent_return, notes
        FROM trades 
        WHERE id = ?1"
    )?;
    
    let trade = stmt.query_row(params![id], |row| {
        let timestamp_str: String = row.get(2)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let status_json: String = row.get(4)?;
        let status: TradeStatus = serde_json::from_str(&status_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?;
            
        let entry_time: Option<String> = row.get(5)?;
        let entry_time = entry_time.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))
                .unwrap()
                .with_timezone(&Utc)
        });
        
        let exit_time: Option<String> = row.get(6)?;
        let exit_time = exit_time.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, Box::new(e)))
                .unwrap()
                .with_timezone(&Utc)
        });
        
        Ok(Trade {
            id: Some(row.get(0)?),
            analysis_id: row.get(1)?,
            timestamp,
            symbol: row.get(3)?,
            status,
            entry_time,
            exit_time,
            entry_price: row.get(7)?,
            exit_price: row.get(8)?,
            quantity: row.get(9)?,
            profit_loss: row.get(10)?,
            percent_return: row.get(11)?,
            notes: row.get(12)?,
        })
    })?;
    
    Ok(trade)
}

pub fn get_trades_by_analysis(conn: &Connection, analysis_id: i64) -> Result<Vec<Trade>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, analysis_id, timestamp, symbol, status, entry_time, exit_time, entry_price, 
            exit_price, quantity, profit_loss, percent_return, notes
        FROM trades 
        WHERE analysis_id = ?1
        ORDER BY timestamp DESC"
    )?;
    
    let trades_iter = stmt.query_map(params![analysis_id], |row| {
        let timestamp_str: String = row.get(2)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let status_json: String = row.get(4)?;
        let status: TradeStatus = serde_json::from_str(&status_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?;
            
        let entry_time: Option<String> = row.get(5)?;
        let entry_time = entry_time.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))
                .unwrap()
                .with_timezone(&Utc)
        });
        
        let exit_time: Option<String> = row.get(6)?;
        let exit_time = exit_time.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, Box::new(e)))
                .unwrap()
                .with_timezone(&Utc)
        });
        
        Ok(Trade {
            id: Some(row.get(0)?),
            analysis_id: row.get(1)?,
            timestamp,
            symbol: row.get(3)?,
            status,
            entry_time,
            exit_time,
            entry_price: row.get(7)?,
            exit_price: row.get(8)?,
            quantity: row.get(9)?,
            profit_loss: row.get(10)?,
            percent_return: row.get(11)?,
            notes: row.get(12)?,
        })
    })?;
    
    let mut trades = Vec::new();
    for trade in trades_iter {
        trades.push(trade?);
    }
    
    Ok(trades)
}

pub fn get_recent_trades(conn: &Connection, limit: i64) -> Result<Vec<Trade>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, analysis_id, timestamp, symbol, status, entry_time, exit_time, entry_price, 
            exit_price, quantity, profit_loss, percent_return, notes
        FROM trades 
        ORDER BY timestamp DESC 
        LIMIT ?1"
    )?;
    
    let trades_iter = stmt.query_map(params![limit], |row| {
        let timestamp_str: String = row.get(2)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let status_json: String = row.get(4)?;
        let status: TradeStatus = serde_json::from_str(&status_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?;
            
        let entry_time: Option<String> = row.get(5)?;
        let entry_time = entry_time.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))
                .unwrap()
                .with_timezone(&Utc)
        });
        
        let exit_time: Option<String> = row.get(6)?;
        let exit_time = exit_time.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, Box::new(e)))
                .unwrap()
                .with_timezone(&Utc)
        });
        
        Ok(Trade {
            id: Some(row.get(0)?),
            analysis_id: row.get(1)?,
            timestamp,
            symbol: row.get(3)?,
            status,
            entry_time,
            exit_time,
            entry_price: row.get(7)?,
            exit_price: row.get(8)?,
            quantity: row.get(9)?,
            profit_loss: row.get(10)?,
            percent_return: row.get(11)?,
            notes: row.get(12)?,
        })
    })?;
    
    let mut trades = Vec::new();
    for trade in trades_iter {
        trades.push(trade?);
    }
    
    Ok(trades)
} 