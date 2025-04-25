use rusqlite::{Connection, params};
use serde_json::to_string;
use std::error::Error;
use chrono::Utc;

use crate::models::stock_rating::{StockRating, MarketTrend, ChartPattern};

pub fn save_stock_rating(conn: &Connection, rating: &mut StockRating) -> Result<i64, Box<dyn Error>> {
    rating.update_overall_score();
    rating.timestamp = Utc::now();
    
    let market_trend_json = to_string(&rating.market_trend)?;
    let chart_pattern_json = to_string(&rating.chart_pattern)?;
    
    conn.execute(
        "INSERT INTO stock_ratings 
        (timestamp, symbol, security_name, sector, market_sentiment, sector_sentiment, security_sentiment, 
        bull_bear, confidence, market_trend, chart_pattern, strategy, overall_score, notes) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            rating.timestamp.to_rfc3339(),
            rating.symbol,
            rating.security_name,
            rating.sector,
            rating.market_sentiment,
            rating.sector_sentiment,
            rating.security_sentiment,
            rating.bull_bear,
            rating.confidence,
            market_trend_json,
            chart_pattern_json,
            rating.strategy,
            rating.overall_score,
            rating.notes,
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    rating.id = Some(id);
    
    Ok(id)
}

pub fn get_stock_rating(conn: &Connection, id: i64) -> Result<StockRating, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, timestamp, symbol, security_name, sector, market_sentiment, sector_sentiment, 
            security_sentiment, bull_bear, confidence, market_trend, chart_pattern, 
            strategy, overall_score, notes
        FROM stock_ratings 
        WHERE id = ?1"
    )?;
    
    let rating = stmt.query_row(params![id], |row| {
        let timestamp_str: String = row.get(1)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let market_trend_json: String = row.get(10)?;
        let market_trend: MarketTrend = serde_json::from_str(&market_trend_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?;
            
        let chart_pattern_json: String = row.get(11)?;
        let chart_pattern: ChartPattern = serde_json::from_str(&chart_pattern_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(11, rusqlite::types::Type::Text, Box::new(e)))?;
        
        Ok(StockRating {
            id: Some(row.get(0)?),
            timestamp,
            symbol: row.get(2)?,
            security_name: row.get(3)?,
            sector: row.get(4)?,
            market_sentiment: row.get(5)?,
            sector_sentiment: row.get(6)?,
            security_sentiment: row.get(7)?,
            bull_bear: row.get(8)?,
            confidence: row.get(9)?,
            market_trend,
            chart_pattern,
            strategy: row.get(12)?,
            overall_score: row.get(13)?,
            notes: row.get(14)?,
        })
    })?;
    
    Ok(rating)
}

pub fn get_stock_ratings_by_symbol(conn: &Connection, symbol: &str) -> Result<Vec<StockRating>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, timestamp, symbol, security_name, sector, market_sentiment, sector_sentiment, 
            security_sentiment, bull_bear, confidence, market_trend, chart_pattern, 
            strategy, overall_score, notes
        FROM stock_ratings 
        WHERE symbol = ?1
        ORDER BY timestamp DESC"
    )?;
    
    let ratings_iter = stmt.query_map(params![symbol], |row| {
        let timestamp_str: String = row.get(1)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let market_trend_json: String = row.get(10)?;
        let market_trend: MarketTrend = serde_json::from_str(&market_trend_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?;
            
        let chart_pattern_json: String = row.get(11)?;
        let chart_pattern: ChartPattern = serde_json::from_str(&chart_pattern_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(11, rusqlite::types::Type::Text, Box::new(e)))?;
        
        Ok(StockRating {
            id: Some(row.get(0)?),
            timestamp,
            symbol: row.get(2)?,
            security_name: row.get(3)?,
            sector: row.get(4)?,
            market_sentiment: row.get(5)?,
            sector_sentiment: row.get(6)?,
            security_sentiment: row.get(7)?,
            bull_bear: row.get(8)?,
            confidence: row.get(9)?,
            market_trend,
            chart_pattern,
            strategy: row.get(12)?,
            overall_score: row.get(13)?,
            notes: row.get(14)?,
        })
    })?;
    
    let mut ratings = Vec::new();
    for rating in ratings_iter {
        ratings.push(rating?);
    }
    
    Ok(ratings)
}

pub fn get_recent_stock_ratings(conn: &Connection, limit: i64) -> Result<Vec<StockRating>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, timestamp, symbol, security_name, sector, market_sentiment, sector_sentiment, 
            security_sentiment, bull_bear, confidence, market_trend, chart_pattern, 
            strategy, overall_score, notes
        FROM stock_ratings 
        ORDER BY timestamp DESC 
        LIMIT ?1"
    )?;
    
    let ratings_iter = stmt.query_map(params![limit], |row| {
        let timestamp_str: String = row.get(1)?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);
        
        let market_trend_json: String = row.get(10)?;
        let market_trend: MarketTrend = serde_json::from_str(&market_trend_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?;
            
        let chart_pattern_json: String = row.get(11)?;
        let chart_pattern: ChartPattern = serde_json::from_str(&chart_pattern_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(11, rusqlite::types::Type::Text, Box::new(e)))?;
        
        Ok(StockRating {
            id: Some(row.get(0)?),
            timestamp,
            symbol: row.get(2)?,
            security_name: row.get(3)?,
            sector: row.get(4)?,
            market_sentiment: row.get(5)?,
            sector_sentiment: row.get(6)?,
            security_sentiment: row.get(7)?,
            bull_bear: row.get(8)?,
            confidence: row.get(9)?,
            market_trend,
            chart_pattern,
            strategy: row.get(12)?,
            overall_score: row.get(13)?,
            notes: row.get(14)?,
        })
    })?;
    
    let mut ratings = Vec::new();
    for rating in ratings_iter {
        ratings.push(rating?);
    }
    
    Ok(ratings)
} 