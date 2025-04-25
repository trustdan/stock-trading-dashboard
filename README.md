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

## Building for Production

### Icon Requirements

Before building for production, you need appropriate icon files in the `src-tauri/icons/` directory:

- **Windows**: Requires `icon.ico` file (256x256px recommended)
- **macOS**: Requires `icon.icns` file
- **Cross-platform**: Include multiple formats in your `tauri.conf.json`:
  ```json
  "bundle": {
    "active": true,
    "icon": [
      "icons/32x32.png",  
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
  ```

### Building on Windows

```powershell
npm run tauri build
```

Installers will be created in:
- MSI installer: `src-tauri/target/release/bundle/msi/`
- EXE installer: `src-tauri/target/release/bundle/nsis/`

### Building on macOS

```bash
npm run tauri build
```

DMG installer will be created in:
- `src-tauri/target/release/bundle/dmg/`

### Cross-Platform Building

Building for platforms other than your host OS requires additional setup:
- Windows → Mac: Not directly possible without a Mac or CI service
- Mac → Windows: Requires Wine and additional tooling

Many developers use CI/CD services like GitHub Actions to build for all platforms automatically.

## Database

The application uses SQLite for data persistence with the bundled feature, meaning:
- No separate SQLite installation is required
- SQLite is compiled directly into the application
- The database file is automatically created in the application data directory:

**Database Locations:**
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

## Troubleshooting

### TypeScript in Svelte
- If using TypeScript with Svelte, ensure proper setup or use `.js` files directly
- TypeScript type annotations in Svelte files may cause errors without proper configuration

### Building Issues
- Icon files are required for production builds
- Windows builds specifically need an `.ico` file
- macOS builds specifically need an `.icns` file

## License

MIT 