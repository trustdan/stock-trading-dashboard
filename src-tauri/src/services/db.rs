use rusqlite::Connection;
use std::error::Error;

pub fn initialize_database(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS psychological_states (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            gain_loss_yesterday REAL NOT NULL,
            emotional_state INTEGER NOT NULL,
            fomo INTEGER NOT NULL,
            market_bias INTEGER NOT NULL,
            hunger INTEGER NOT NULL,
            headache_pain INTEGER NOT NULL,
            extra_factors TEXT NOT NULL,
            total_risk_score REAL NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS stock_ratings (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            symbol TEXT NOT NULL,
            security_name TEXT,
            sector TEXT NOT NULL,
            market_sentiment INTEGER NOT NULL,
            sector_sentiment INTEGER NOT NULL,
            security_sentiment INTEGER NOT NULL,
            bull_bear INTEGER NOT NULL,
            confidence INTEGER NOT NULL,
            market_trend TEXT NOT NULL,
            chart_pattern TEXT NOT NULL,
            strategy TEXT NOT NULL,
            overall_score INTEGER NOT NULL,
            notes TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS detailed_analyses (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            bull_bear INTEGER NOT NULL,
            confidence INTEGER NOT NULL,
            market_trend TEXT NOT NULL,
            chart_pattern TEXT NOT NULL,
            strategy TEXT NOT NULL,
            overall_score INTEGER NOT NULL,
            market_sentiment INTEGER NOT NULL,
            sector_sentiment INTEGER NOT NULL,
            sector TEXT NOT NULL,
            security TEXT NOT NULL,
            bought BOOLEAN NOT NULL,
            entry_reason TEXT NOT NULL,
            time TEXT NOT NULL,
            entry_price REAL NOT NULL,
            stop_loss REAL NOT NULL,
            target_price REAL NOT NULL,
            short_leg TEXT,
            long_leg TEXT,
            debit_credit REAL NOT NULL,
            quantity INTEGER NOT NULL,
            risk_max REAL NOT NULL,
            reward REAL NOT NULL,
            max_gain REAL,
            percent_profit REAL,
            delta REAL,
            theta REAL,
            gamma REAL,
            vega REAL,
            alerts TEXT NOT NULL,
            exit_reason TEXT,
            skip_reason TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS trades (
            id INTEGER PRIMARY KEY,
            analysis_id INTEGER NOT NULL,
            timestamp TEXT NOT NULL,
            symbol TEXT NOT NULL,
            status TEXT NOT NULL,
            entry_time TEXT,
            exit_time TEXT,
            entry_price REAL,
            exit_price REAL,
            quantity INTEGER NOT NULL,
            profit_loss REAL,
            percent_return REAL,
            notes TEXT,
            FOREIGN KEY (analysis_id) REFERENCES detailed_analyses (id)
        )",
        [],
    )?;

    Ok(())
} 