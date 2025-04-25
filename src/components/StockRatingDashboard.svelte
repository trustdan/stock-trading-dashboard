<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { Chart, registerables } from 'chart.js';
  
  // Register Chart.js components
  Chart.register(...registerables);
  
  let sectors = [
    'Basic Materials', 'Communication Services', 'Consumer Cyclical',
    'Consumer Defensive', 'Energy', 'Financial', 'Healthcare',
    'Industrials', 'Real Estate', 'Technology', 'Utilities'
  ];
  
  let selectedSectors = [];
  let selectedSector = '';
  let sectorRationale = '';
  
  let symbol = '';
  let securityName = '';
  let marketSentiment = 0;
  let sectorSentiment = 0;
  let securitySentiment = 0;
  let bullBear = 1;  // 1 = Bull, -1 = Bear
  let confidence = 50;  // 0-100%
  let marketTrend = 'Uncertain';
  let chartPattern = 'None';
  let strategy = '';
  let notes = '';
  let overallScore = 0;
  
  let recentRatings = [];
  let marketTrends = ['Uptrend', 'Downtrend', 'Sideways', 'Uncertain'];
  let chartPatterns = [
    'HighBase', 'LowBase', 'AscendingTriangle', 'DescendingTriangle',
    'Cup', 'HeadAndShoulders', 'InverseHeadAndShoulders', 'DoubleTop',
    'DoubleBottom', 'Consolidation', 'BreakoutPullback', 'None'
  ];

  function addSector() {
    if (selectedSector && !selectedSectors.find(s => s.name === selectedSector)) {
      selectedSectors = [...selectedSectors, {
        name: selectedSector,
        rationale: sectorRationale
      }];
      selectedSector = '';
      sectorRationale = '';
    }
  }

  function removeSector(sectorToRemove) {
    selectedSectors = selectedSectors.filter(s => s.name !== sectorToRemove);
  }

  function calculateScore() {
    const sentimentSum = marketSentiment + sectorSentiment + securitySentiment;
    const confidenceFactor = (confidence / 100) * 3;
    overallScore = bullBear * (sentimentSum + confidenceFactor);
    
    // Save the rating
    saveRating();
  }
  
  async function saveRating() {
    if (!symbol) {
      alert('Security symbol is required');
      return;
    }
    
    try {
      await invoke('save_stock_rating', {
        rating: {
          symbol,
          security_name: securityName,
          sector: selectedSectors[0]?.name || '',
          market_sentiment: marketSentiment,
          sector_sentiment: sectorSentiment,
          security_sentiment: securitySentiment,
          bull_bear: bullBear,
          confidence,
          market_trend: marketTrend,
          chart_pattern: chartPattern === 'None' ? { Other: 'None' } : chartPattern,
          strategy,
          overall_score: overallScore,
          notes
        }
      });
      
      // Refresh the list
      loadRecentRatings();
      
    } catch (error) {
      console.error("Failed to save stock rating:", error);
    }
  }
  
  async function loadRecentRatings() {
    try {
      recentRatings = await invoke('get_recent_stock_ratings', { limit: 10 });
    } catch (error) {
      console.error("Failed to load recent ratings:", error);
    }
  }
  
  function createOpportunityChart(ratings) {
    // This would be implemented with Chart.js
    // For demo purposes we'll just show a placeholder
    console.log("Would create chart with ratings:", ratings);
  }
  
  function viewRatingDetails(ratingId) {
    // Navigate to detailed view
    console.log("View details for rating:", ratingId);
  }
  
  onMount(() => {
    loadRecentRatings();
  });
</script>

<div class="stock-rating-dashboard">
  <div class="dashboard-columns">
    <div class="rating-inputs">
      <h2>Stock Rating</h2>

      <div class="sector-selection">
        <h3>Sector Selection</h3>
        <div class="sector-picker">
          <select bind:value={selectedSector}>
            <option value="">Select a sector</option>
            {#each sectors as sector}
              <option value={sector}>{sector}</option>
            {/each}
          </select>
          <input 
            type="text" 
            placeholder="Rationale (optional)" 
            bind:value={sectorRationale}
          />
          <button on:click={addSector}>Add</button>
        </div>

        <div class="selected-sectors">
          {#each selectedSectors as sector}
            <div class="sector-tag">
              <span>{sector.name}</span>
              {#if sector.rationale}
                <span class="rationale"> - {sector.rationale}</span>
              {/if}
              <button on:click={() => removeSector(sector.name)}>×</button>
            </div>
          {/each}
        </div>
      </div>

      <div class="rating-section">
        <h3>Security Information</h3>
        <div class="input-group">
          <label for="symbol">Symbol</label>
          <input type="text" id="symbol" bind:value={symbol} placeholder="e.g. AAPL" />
        </div>
        <div class="input-group">
          <label for="security-name">Security Name (Optional)</label>
          <input type="text" id="security-name" bind:value={securityName} placeholder="e.g. Apple Inc" />
        </div>
      </div>

      <div class="rating-section">
        <h3>Market Sentiment</h3>
        <div class="sentiment-container">
          <div class="sentiment-slider">
            <label for="market-sentiment">Market ({marketSentiment})</label>
            <input 
              type="range" 
              id="market-sentiment" 
              bind:value={marketSentiment} 
              min="-3" 
              max="3" 
              step="1"
            />
            <div class="slider-labels">
              <span>Bearish</span>
              <span>Bullish</span>
            </div>
          </div>

          <div class="sentiment-slider">
            <label for="sector-sentiment">Sector ({sectorSentiment})</label>
            <input 
              type="range" 
              id="sector-sentiment" 
              bind:value={sectorSentiment} 
              min="-3" 
              max="3" 
              step="1"
            />
            <div class="slider-labels">
              <span>Bearish</span>
              <span>Bullish</span>
            </div>
          </div>

          <div class="sentiment-slider">
            <label for="security-sentiment">Security ({securitySentiment})</label>
            <input 
              type="range" 
              id="security-sentiment" 
              bind:value={securitySentiment} 
              min="-3" 
              max="3" 
              step="1"
            />
            <div class="slider-labels">
              <span>Bearish</span>
              <span>Bullish</span>
            </div>
          </div>
        </div>
      </div>

      <div class="rating-section">
        <h3>Analysis</h3>
        <div class="input-group bull-bear">
          <label>Bull/Bear</label>
          <div class="toggle-container">
            <button 
              class:active={bullBear === -1} 
              on:click={() => bullBear = -1}
            >Bear</button>
            <button 
              class:active={bullBear === 1} 
              on:click={() => bullBear = 1}
            >Bull</button>
          </div>
        </div>

        <div class="input-group">
          <label for="confidence">Confidence ({confidence}%)</label>
          <input 
            type="range" 
            id="confidence" 
            bind:value={confidence} 
            min="0" 
            max="100" 
            step="1"
          />
        </div>

        <div class="input-group">
          <label for="market-trend">Market Trend</label>
          <select id="market-trend" bind:value={marketTrend}>
            {#each marketTrends as trend}
              <option value={trend}>{trend}</option>
            {/each}
          </select>
        </div>

        <div class="input-group">
          <label for="chart-pattern">Chart Pattern</label>
          <select id="chart-pattern" bind:value={chartPattern}>
            {#each chartPatterns as pattern}
              <option value={pattern}>{pattern}</option>
            {/each}
          </select>
        </div>

        <div class="input-group">
          <label for="strategy">Strategy</label>
          <input type="text" id="strategy" bind:value={strategy} placeholder="e.g. Breakout, Pullback" />
        </div>

        <div class="input-group">
          <label for="notes">Notes</label>
          <textarea id="notes" bind:value={notes} rows="3"></textarea>
        </div>

        <div class="score-container">
          <div class="overall-score">
            <span>Overall Score</span>
            <span class="score-value" class:positive={overallScore > 0} class:negative={overallScore < 0}>{overallScore}</span>
          </div>
          <button class="calc-btn" on:click={calculateScore}>Calculate Score</button>
        </div>
      </div>
    </div>

    <div class="ratings-list">
      <h2>Recent Ratings</h2>
      
      <div class="opportunity-chart">
        <h3>Opportunity Visualization</h3>
        <div class="chart-placeholder">
          <p>Select a security to view opportunity chart</p>
        </div>
      </div>
      
      <div class="ratings-table">
        <table>
          <thead>
            <tr>
              <th>Symbol</th>
              <th>Date</th>
              <th>Sector</th>
              <th>Score</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each recentRatings as rating}
              <tr>
                <td>{rating.symbol}</td>
                <td>{new Date(rating.timestamp).toLocaleString()}</td>
                <td>{rating.sector}</td>
                <td class:positive={rating.overall_score > 0} class:negative={rating.overall_score < 0}>
                  {rating.overall_score}
                </td>
                <td>
                  <button class="view-btn" on:click={() => viewRatingDetails(rating.id)}>View</button>
                </td>
              </tr>
            {/each}
            
            {#if recentRatings.length === 0}
              <tr>
                <td colspan="5" class="no-data">No ratings available</td>
              </tr>
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

<style>
  .stock-rating-dashboard {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
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
  
  .dashboard-columns {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
  }
  
  .rating-section {
    margin-bottom: 1.5rem;
    padding: 1rem;
    background-color: #f8f9fa;
    border-radius: 6px;
  }
  
  .input-group {
    margin-bottom: 1rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.3rem;
    font-weight: bold;
    color: #555;
  }
  
  input[type="text"],
  input[type="number"],
  select,
  textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
  }
  
  textarea {
    resize: vertical;
  }
  
  .sentiment-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .sentiment-slider {
    width: 100%;
  }
  
  .slider-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
    color: #777;
  }
  
  .sector-selection {
    margin-bottom: 1.5rem;
    padding: 1rem;
    background-color: #f8f9fa;
    border-radius: 6px;
  }
  
  .sector-picker {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  
  .sector-picker select {
    flex: 1;
  }
  
  .sector-picker input {
    flex: 2;
  }
  
  .selected-sectors {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .sector-tag {
    background-color: #e1f5fe;
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    font-size: 0.8rem;
    display: flex;
    align-items: center;
  }
  
  .sector-tag .rationale {
    font-style: italic;
    margin-left: 0.3rem;
    color: #666;
  }
  
  .sector-tag button {
    background: none;
    border: none;
    color: #e74c3c;
    font-weight: bold;
    cursor: pointer;
    margin-left: 0.5rem;
    padding: 0 0.3rem;
  }
  
  .bull-bear .toggle-container {
    display: flex;
    width: 100%;
  }
  
  .bull-bear button {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid #ddd;
    background-color: #f5f5f5;
    cursor: pointer;
  }
  
  .bull-bear button:first-child {
    border-radius: 4px 0 0 4px;
  }
  
  .bull-bear button:last-child {
    border-radius: 0 4px 4px 0;
  }
  
  .bull-bear button.active {
    background-color: #3498db;
    color: white;
    border-color: #3498db;
  }
  
  .score-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 1.5rem;
  }
  
  .overall-score {
    font-size: 1.2rem;
    display: flex;
    flex-direction: column;
  }
  
  .score-value {
    font-size: 2rem;
    font-weight: bold;
  }
  
  .positive {
    color: #27ae60;
  }
  
  .negative {
    color: #e74c3c;
  }
  
  .calc-btn {
    background-color: #2980b9;
    color: white;
    border: none;
    padding: 0.8rem;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
  }
  
  .calc-btn:hover {
    background-color: #3498db;
  }
  
  .opportunity-chart {
    margin-bottom: 1.5rem;
    padding: 1rem;
    background-color: #f8f9fa;
    border-radius: 6px;
  }
  
  .chart-placeholder {
    height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #eee;
    border-radius: 4px;
    color: #777;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 0.8rem;
    text-align: left;
    border-bottom: 1px solid #eee;
  }
  
  th {
    background-color: #f8f9fa;
    font-weight: bold;
    color: #555;
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
</style> 