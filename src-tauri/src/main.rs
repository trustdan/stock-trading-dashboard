#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod services;

use std::sync::Mutex;
use tauri::State;
use tauri::Manager;

use crate::models::{PsychologicalState, StockRating, Trade};
use crate::services::{psychological_service, stock_rating_service, trade_service};

struct AppState {
    db: Mutex<Option<rusqlite::Connection>>,
}

#[tauri::command]
fn save_psychological_state(app_state: State<AppState>, state: PsychologicalState) -> Result<i64, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut state_copy = state;
    psychological_service::save_psychological_state(conn, &mut state_copy)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_psychological_state(app_state: State<AppState>, id: i64) -> Result<PsychologicalState, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    psychological_service::get_psychological_state(conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recent_psychological_states(app_state: State<AppState>, limit: i64) -> Result<Vec<PsychologicalState>, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    psychological_service::get_recent_psychological_states(conn, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_stock_rating(app_state: State<AppState>, rating: StockRating) -> Result<i64, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut rating_copy = rating;
    stock_rating_service::save_stock_rating(conn, &mut rating_copy)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_stock_rating(app_state: State<AppState>, id: i64) -> Result<StockRating, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    stock_rating_service::get_stock_rating(conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_stock_ratings_by_symbol(app_state: State<AppState>, symbol: String) -> Result<Vec<StockRating>, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    stock_rating_service::get_stock_ratings_by_symbol(conn, &symbol)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recent_stock_ratings(app_state: State<AppState>, limit: i64) -> Result<Vec<StockRating>, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    stock_rating_service::get_recent_stock_ratings(conn, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_trade(app_state: State<AppState>, trade: Trade) -> Result<i64, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let mut trade_copy = trade;
    trade_service::save_trade(conn, &mut trade_copy)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_trade(app_state: State<AppState>, trade: Trade) -> Result<(), String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    trade_service::update_trade(conn, &trade)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_trade(app_state: State<AppState>, id: i64) -> Result<Trade, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    trade_service::get_trade(conn, id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_trades_by_analysis(app_state: State<AppState>, analysis_id: i64) -> Result<Vec<Trade>, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    trade_service::get_trades_by_analysis(conn, analysis_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recent_trades(app_state: State<AppState>, limit: i64) -> Result<Vec<Trade>, String> {
    let db_guard = app_state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Database not initialized")?;
    
    trade_service::get_recent_trades(conn, limit)
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            save_psychological_state,
            get_psychological_state,
            get_recent_psychological_states,
            save_stock_rating,
            get_stock_rating,
            get_stock_ratings_by_symbol,
            get_recent_stock_ratings,
            save_trade,
            update_trade,
            get_trade,
            get_trades_by_analysis,
            get_recent_trades
        ])
        .setup(|app| {
            // Initialize database connection
            let app_state: State<AppState> = app.state();
            let mut db_guard = app_state.db.lock().unwrap();
            let db_path = app.path_resolver()
                .app_data_dir()
                .unwrap()
                .join("stock_dashboard.db");
            
            // Ensure directory exists
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            *db_guard = Some(rusqlite::Connection::open(db_path)?);
            
            // Initialize database schema
            if let Some(conn) = &*db_guard {
                services::db::initialize_database(conn)?;
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 