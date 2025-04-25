<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let trades = [];
  let filteredTrades = [];
  let selectedDate = new Date();
  let selectedTrade = null;
  let searchQuery = '';
  
  let detailedView = false;
  let weekViewDates = [];
  
  function generateWeekDates(date) {
    const day = date.getDay(); // 0 = Sunday, 6 = Saturday
    const diff = date.getDate() - day + (day === 0 ? -6 : 1); // Adjust when day is Sunday
    
    const monday = new Date(date.setDate(diff));
    const weekDates = [];
    
    for (let i = 0; i < 5; i++) { // Monday to Friday
      const newDate = new Date(monday);
      newDate.setDate(monday.getDate() + i);
      weekDates.push(newDate);
    }
    
    return weekDates;
  }
  
  function formatDate(date) {
    return date.toLocaleDateString('en-US', { 
      weekday: 'short',
      month: 'short', 
      day: 'numeric'
    });
  }
  
  function getTradeCountForDate(date) {
    const dateString = date.toDateString();
    return trades.filter(trade => {
      const tradeDate = new Date(trade.timestamp);
      return tradeDate.toDateString() === dateString;
    }).length;
  }
  
  function selectDate(date) {
    selectedDate = date;
    filteredTrades = trades.filter(trade => {
      const tradeDate = new Date(trade.timestamp);
      return tradeDate.toDateString() === date.toDateString();
    });
  }
  
  function previousWeek() {
    const prevWeek = new Date(weekViewDates[0]);
    prevWeek.setDate(prevWeek.getDate() - 7);
    weekViewDates = generateWeekDates(prevWeek);
  }
  
  function nextWeek() {
    const nextWeek = new Date(weekViewDates[0]);
    nextWeek.setDate(nextWeek.getDate() + 7);
    weekViewDates = generateWeekDates(nextWeek);
  }
  
  function goToCurrentWeek() {
    weekViewDates = generateWeekDates(new Date());
  }
  
  function filterTrades() {
    if (!searchQuery) {
      loadAllTrades();
      return;
    }
    
    filteredTrades = trades.filter(trade => 
      trade.symbol.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (trade.notes && trade.notes.toLowerCase().includes(searchQuery.toLowerCase()))
    );
  }
  
  function viewTradeDetails(trade) {
    selectedTrade = trade;
    detailedView = true;
  }
  
  function closeDetailedView() {
    detailedView = false;
    selectedTrade = null;
  }
  
  async function loadAllTrades() {
    try {
      trades = await invoke('get_recent_trades', { limit: 100 });
      filteredTrades = [...trades];
    } catch (error) {
      console.error("Failed to load trades:", error);
    }
  }
  
  // Calculate profit/loss summary
  function calculateSummary() {
    if (!trades.length) return { totalTrades: 0, winRate: 0, profit: 0 };
    
    const completedTrades = trades.filter(t => t.status === 'Closed');
    const winningTrades = completedTrades.filter(t => t.profit_loss > 0);
    
    const totalProfit = completedTrades.reduce((sum, t) => sum + (t.profit_loss || 0), 0);
    const winRate = completedTrades.length ? (winningTrades.length / completedTrades.length * 100) : 0;
    
    return {
      totalTrades: completedTrades.length,
      winRate: winRate.toFixed(1),
      profit: totalProfit.toFixed(2)
    };
  }
  
  onMount(() => {
    weekViewDates = generateWeekDates(new Date());
    loadAllTrades();
  });
</script>

<div class="trade-calendar">
  {#if detailedView && selectedTrade}
    <div class="detailed-trade-view">
      <button class="back-btn" on:click={closeDetailedView}>← Back to Calendar</button>
      
      <div class="trade-details">
        <h2>{selectedTrade.symbol} Trade Details</h2>
        
        <div class="details-grid">
          <div class="detail-card">
            <h3>Trade Information</h3>
            <div class="detail-item">
              <span class="label">Symbol:</span>
              <span class="value">{selectedTrade.symbol}</span>
            </div>
            <div class="detail-item">
              <span class="label">Status:</span>
              <span class="value status-badge" class:status-open={selectedTrade.status === 'Open'} 
                class:status-closed={selectedTrade.status === 'Closed'}
                class:status-cancelled={selectedTrade.status === 'Cancelled'}>
                {selectedTrade.status}
              </span>
            </div>
            <div class="detail-item">
              <span class="label">Date:</span>
              <span class="value">{new Date(selectedTrade.timestamp).toLocaleString()}</span>
            </div>
            <div class="detail-item">
              <span class="label">Quantity:</span>
              <span class="value">{selectedTrade.quantity}</span>
            </div>
          </div>
          
          <div class="detail-card">
            <h3>Entry & Exit</h3>
            <div class="detail-item">
              <span class="label">Entry Price:</span>
              <span class="value">${selectedTrade.entry_price || 'N/A'}</span>
            </div>
            <div class="detail-item">
              <span class="label">Entry Time:</span>
              <span class="value">{selectedTrade.entry_time ? new Date(selectedTrade.entry_time).toLocaleString() : 'N/A'}</span>
            </div>
            <div class="detail-item">
              <span class="label">Exit Price:</span>
              <span class="value">${selectedTrade.exit_price || 'N/A'}</span>
            </div>
            <div class="detail-item">
              <span class="label">Exit Time:</span>
              <span class="value">{selectedTrade.exit_time ? new Date(selectedTrade.exit_time).toLocaleString() : 'N/A'}</span>
            </div>
          </div>
          
          <div class="detail-card">
            <h3>Performance</h3>
            <div class="detail-item">
              <span class="label">Profit/Loss:</span>
              <span class="value" class:positive={selectedTrade.profit_loss > 0} 
                class:negative={selectedTrade.profit_loss < 0}>
                ${selectedTrade.profit_loss?.toFixed(2) || 'N/A'}
              </span>
            </div>
            <div class="detail-item">
              <span class="label">Return %:</span>
              <span class="value" class:positive={selectedTrade.percent_return > 0} 
                class:negative={selectedTrade.percent_return < 0}>
                {selectedTrade.percent_return?.toFixed(2) || 'N/A'}%
              </span>
            </div>
          </div>
          
          <div class="detail-card notes-card">
            <h3>Notes</h3>
            <p class="notes">{selectedTrade.notes || 'No notes for this trade.'}</p>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="calendar-container">
      <div class="calendar-header">
        <h2>Trade Calendar</h2>
        <div class="calendar-controls">
          <button on:click={previousWeek}>◀</button>
          <button on:click={goToCurrentWeek}>Today</button>
          <button on:click={nextWeek}>▶</button>
        </div>
      </div>
      
      <div class="week-view">
        {#each weekViewDates as date}
          <div class="day-column" class:selected={date.toDateString() === selectedDate.toDateString()}
              on:click={() => selectDate(date)}>
            <div class="day-header">
              <span class="day-name">{formatDate(date)}</span>
              <span class="trade-count">{getTradeCountForDate(date)} trades</span>
            </div>
            <div class="day-content">
              {#each trades.filter(t => new Date(t.timestamp).toDateString() === date.toDateString()) as trade}
                <div class="trade-card" on:click|stopPropagation={() => viewTradeDetails(trade)}>
                  <div class="trade-symbol">{trade.symbol}</div>
                  <div class="trade-status" class:status-open={trade.status === 'Open'} 
                    class:status-closed={trade.status === 'Closed'}
                    class:status-cancelled={trade.status === 'Cancelled'}>
                    {trade.status}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
      
      <div class="trades-list">
        <div class="trades-header">
          <h3>{filteredTrades.length} Trades {searchQuery ? 'Matching "' + searchQuery + '"' : ''}</h3>
          <div class="search-container">
            <input 
              type="text" 
              placeholder="Search by symbol or notes..." 
              bind:value={searchQuery} 
              on:input={filterTrades}
            />
          </div>
        </div>
        
        <div class="summary-stats">
          {#if true}
            {@const summary = calculateSummary()}
            <div class="stat">
              <span class="stat-value">{summary.totalTrades}</span>
              <span class="stat-label">Total Trades</span>
            </div>
            <div class="stat">
              <span class="stat-value">{summary.winRate}%</span>
              <span class="stat-label">Win Rate</span>
            </div>
            <div class="stat">
              <span class="stat-value" class:positive={parseFloat(summary.profit) > 0} 
                class:negative={parseFloat(summary.profit) < 0}>
                ${summary.profit}
              </span>
              <span class="stat-label">Total P/L</span>
            </div>
          {/if}
        </div>
        
        <table class="trades-table">
          <thead>
            <tr>
              <th>Symbol</th>
              <th>Date</th>
              <th>Status</th>
              <th>P/L</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredTrades as trade}
              <tr>
                <td>{trade.symbol}</td>
                <td>{new Date(trade.timestamp).toLocaleDateString()}</td>
                <td>
                  <span class="status-badge" class:status-open={trade.status === 'Open'} 
                    class:status-closed={trade.status === 'Closed'}
                    class:status-cancelled={trade.status === 'Cancelled'}>
                    {trade.status}
                  </span>
                </td>
                <td class:positive={trade.profit_loss > 0} 
                    class:negative={trade.profit_loss < 0}>
                  ${trade.profit_loss?.toFixed(2) || '0.00'}
                </td>
                <td>
                  <button class="view-btn" on:click={() => viewTradeDetails(trade)}>View</button>
                </td>
              </tr>
            {/each}
            
            {#if filteredTrades.length === 0}
              <tr>
                <td colspan="5" class="no-data">No trades found</td>
              </tr>
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

<style>
  .trade-calendar {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  h2, h3 {
    color: #2c3e50;
    margin-top: 0;
  }
  
  h2 {
    border-bottom: 2px solid #eee;
    padding-bottom: 0.5rem;
    margin-bottom: 1.5rem;
  }
  
  .calendar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .calendar-controls {
    display: flex;
    gap: 0.5rem;
  }
  
  .calendar-controls button {
    background-color: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 0.3rem 0.6rem;
    cursor: pointer;
  }
  
  .calendar-controls button:hover {
    background-color: #e5e5e5;
  }
  
  .week-view {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 1rem;
    margin-bottom: 2rem;
    min-height: 250px;
  }
  
  .day-column {
    border: 1px solid #eee;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .day-column.selected {
    border-color: #3498db;
    box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
  }
  
  .day-column:hover {
    background-color: #f9f9f9;
  }
  
  .day-header {
    padding: 0.5rem;
    border-bottom: 1px solid #eee;
    background-color: #f8f9fa;
    border-radius: 6px 6px 0 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .day-name {
    font-weight: bold;
    color: #555;
  }
  
  .trade-count {
    font-size: 0.7rem;
    color: #777;
    background-color: #e1f5fe;
    padding: 0.1rem 0.3rem;
    border-radius: 4px;
  }
  
  .day-content {
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-height: 100px;
  }
  
  .trade-card {
    background-color: #f1f8ff;
    border-left: 3px solid #2980b9;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .trade-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  }
  
  .trade-symbol {
    font-weight: bold;
  }
  
  .trade-status {
    font-size: 0.7rem;
    margin-top: 0.3rem;
  }
  
  .status-open {
    color: #2980b9;
  }
  
  .status-closed {
    color: #27ae60;
  }
  
  .status-cancelled {
    color: #e74c3c;
  }
  
  .trades-list {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  
  .trades-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .search-container input {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    width: 300px;
  }
  
  .summary-stats {
    display: flex;
    justify-content: space-around;
    margin-bottom: 1.5rem;
    background-color: #f8f9fa;
    padding: 1rem;
    border-radius: 6px;
  }
  
  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  
  .stat-value {
    font-size: 1.5rem;
    font-weight: bold;
  }
  
  .stat-label {
    font-size: 0.8rem;
    color: #777;
    margin-top: 0.3rem;
  }
  
  .trades-table {
    width: 100%;
    border-collapse: collapse;
  }
  
  .trades-table th, .trades-table td {
    padding: 0.8rem;
    text-align: left;
    border-bottom: 1px solid #eee;
  }
  
  .trades-table th {
    background-color: #f8f9fa;
    font-weight: bold;
    color: #555;
  }
  
  .status-badge {
    display: inline-block;
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: bold;
  }
  
  .status-badge.status-open {
    background-color: #e1f5fe;
  }
  
  .status-badge.status-closed {
    background-color: #e8f5e9;
  }
  
  .status-badge.status-cancelled {
    background-color: #ffebee;
  }
  
  .view-btn {
    background-color: #27ae60;
    color: white;
    border: none;
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .no-data {
    text-align: center;
    color: #777;
    padding: 2rem 0;
  }
  
  .positive {
    color: #27ae60;
  }
  
  .negative {
    color: #e74c3c;
  }
  
  /* Detailed view styles */
  .detailed-trade-view {
    width: 100%;
  }
  
  .back-btn {
    background: none;
    border: none;
    color: #3498db;
    cursor: pointer;
    padding: 0;
    font-size: 1rem;
    margin-bottom: 1rem;
    display: flex;
    align-items: center;
  }
  
  .details-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
  }
  
  .detail-card {
    background-color: #f8f9fa;
    border-radius: 6px;
    padding: 1rem;
  }
  
  .detail-card h3 {
    margin-top: 0;
    margin-bottom: 1rem;
    font-size: 1.1rem;
    color: #2c3e50;
    border-bottom: 1px solid #eee;
    padding-bottom: 0.5rem;
  }
  
  .detail-item {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.8rem;
  }
  
  .label {
    color: #555;
    font-weight: 500;
  }
  
  .value {
    font-weight: bold;
  }
  
  .notes-card {
    grid-column: span 2;
  }
  
  .notes {
    white-space: pre-line;
    color: #555;
    line-height: 1.5;
  }
</style> 