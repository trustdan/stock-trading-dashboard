# Stock Trading Dashboard

A cross-platform native stock trading dashboard application for managing risk metrics, rating stocks, and tracking trades.

## Features

- **Risk Management Dashboard**: Track psychological and emotional risk metrics
- **Stock-Rating Dashboard**: Rate market, sectors, and individual securities
- **Live Calendar & Trade Tracker**: Manage trade positions and review historical trades

## Technology Stack

- **Core Language**: Rust (business logic, data models)
- **Framework**: Tauri (cross-platform native wrapper)
- **Frontend**: Svelte (embedded web UI in Tauri)
- **Persistence**: SQLite (primary), JSON files (fallback)
- **Charting**: Chart.js for opportunity visualizations

## Prerequisites

- [Node.js](https://nodejs.org/) (v14 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

## Getting Started

1. Clone the repository:
   ```
   git clone <repository-url>
   cd stock-trading-dashboard
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Run in development mode:
   ```
   npm run tauri dev
   ```

4. Build for production:
   ```
   npm run tauri build
   ```

## Database

The application uses SQLite for data persistence. The database file is automatically created in the application data directory:

- Windows: `%APPDATA%\stock-trading-dashboard\stock_dashboard.db`
- macOS: `~/Library/Application Support/stock-trading-dashboard/stock_dashboard.db`
- Linux: `~/.config/stock-trading-dashboard/stock_dashboard.db`

## Project Structure

- `/src-tauri`: Rust backend code
  - `/src/models`: Data structures
  - `/src/services`: Business logic and database operations
- `/src`: Svelte frontend code
  - `/components`: UI components
  - `/lib`: Utilities and helpers
  - `/pages`: Page components

## License

MIT 