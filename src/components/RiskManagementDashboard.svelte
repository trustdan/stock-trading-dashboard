<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let gainLossYesterday = 0;
  let emotionalState = 0;  // -3 to +3
  let fomo = 0;            // -3 to +3
  let marketBias = 0;      // -3 to +3
  let hunger = 0;          // 0 to +3
  let headachePain = 0;    // 0 to +3
  let extraFactors = {};   // Custom factors
  let totalRiskScore = 0;
  let extraFactorName = '';
  let extraFactorValue = 0;
  
  const MIN_SLIDER = -3;
  const MAX_SLIDER = 3;
  const MIN_POSITIVE_SLIDER = 0;
  
  function calculateTotalRisk() {
    const base = emotionalState + fomo + marketBias;
    const physical = hunger + headachePain;
    const pnlEffect = (gainLossYesterday / 100.0) * 3.0;
    const clampedPnlEffect = Math.max(-3, Math.min(3, pnlEffect));
    
    let extrasSum = 0;
    Object.values(extraFactors).forEach(value => {
      extrasSum += Number(value);
    });
    
    totalRiskScore = (base + physical + clampedPnlEffect + extrasSum) / 3.0;
    
    // Save to backend
    saveRiskAssessment();
  }
  
  async function saveRiskAssessment() {
    try {
      await invoke('save_psychological_state', {
        state: {
          gain_loss_yesterday: gainLossYesterday,
          emotional_state: emotionalState,
          fomo: fomo,
          market_bias: marketBias,
          hunger: hunger,
          headache_pain: headachePain,
          extra_factors: extraFactors,
          total_risk_score: totalRiskScore
        }
      });
    } catch (error) {
      console.error("Failed to save risk assessment:", error);
    }
  }
  
  function addExtraFactor() {
    if (extraFactorName.trim() !== '') {
      extraFactors[extraFactorName] = extraFactorValue;
      extraFactorName = '';
      extraFactorValue = 0;
    }
  }
  
  function removeExtraFactor(factor) {
    const newFactors = { ...extraFactors };
    delete newFactors[factor];
    extraFactors = newFactors;
  }
  
  onMount(async () => {
    // Load the most recent psychological state if it exists
    try {
      const recentStates = await invoke('get_recent_psychological_states', { limit: 1 });
      if (recentStates && recentStates.length > 0) {
        const state = recentStates[0];
        gainLossYesterday = state.gain_loss_yesterday;
        emotionalState = state.emotional_state;
        fomo = state.fomo;
        marketBias = state.market_bias;
        hunger = state.hunger;
        headachePain = state.headache_pain;
        extraFactors = state.extra_factors;
        totalRiskScore = state.total_risk_score;
      }
    } catch (error) {
      console.error("Failed to load recent psychological states:", error);
    }
  });
</script>

<div class="risk-dashboard">
  <h2>Psychological & Risk Assessment</h2>
  
  <div class="risk-meter">
    <div class="risk-score" style={`--risk-level: ${totalRiskScore}`}>
      <span class="risk-value">{totalRiskScore.toFixed(1)}</span>
    </div>
    <div class="risk-labels">
      <span>Low Risk</span>
      <span>High Risk</span>
    </div>
  </div>
  
  <div class="risk-factors">
    <div class="factor">
      <label for="gain-loss">Gain/Loss Yesterday (%)</label>
      <input 
        type="number" 
        id="gain-loss" 
        bind:value={gainLossYesterday} 
        step="0.01"
      />
    </div>
    
    <div class="factor">
      <label for="emotional-state">Emotional State ({emotionalState})</label>
      <input 
        type="range" 
        id="emotional-state" 
        bind:value={emotionalState} 
        min={MIN_SLIDER} 
        max={MAX_SLIDER} 
        step="1"
      />
      <div class="slider-labels">
        <span>Negative</span>
        <span>Positive</span>
      </div>
    </div>
    
    <div class="factor">
      <label for="fomo">FOMO ({fomo})</label>
      <input 
        type="range" 
        id="fomo" 
        bind:value={fomo} 
        min={MIN_SLIDER} 
        max={MAX_SLIDER} 
        step="1"
      />
      <div class="slider-labels">
        <span>None</span>
        <span>Strong</span>
      </div>
    </div>
    
    <div class="factor">
      <label for="market-bias">Market Bias ({marketBias})</label>
      <input 
        type="range" 
        id="market-bias" 
        bind:value={marketBias} 
        min={MIN_SLIDER} 
        max={MAX_SLIDER} 
        step="1"
      />
      <div class="slider-labels">
        <span>Bearish</span>
        <span>Bullish</span>
      </div>
    </div>
    
    <div class="factor">
      <label for="hunger">Hunger ({hunger})</label>
      <input 
        type="range" 
        id="hunger" 
        bind:value={hunger} 
        min={MIN_POSITIVE_SLIDER} 
        max={MAX_SLIDER} 
        step="1"
      />
      <div class="slider-labels">
        <span>None</span>
        <span>Starving</span>
      </div>
    </div>
    
    <div class="factor">
      <label for="headache">Headache/Pain ({headachePain})</label>
      <input 
        type="range" 
        id="headache" 
        bind:value={headachePain} 
        min={MIN_POSITIVE_SLIDER} 
        max={MAX_SLIDER} 
        step="1"
      />
      <div class="slider-labels">
        <span>None</span>
        <span>Severe</span>
      </div>
    </div>
    
    <!-- Extra factors -->
    <div class="extra-factors">
      <h3>Additional Factors</h3>
      
      {#each Object.entries(extraFactors) as [factor, value]}
        <div class="extra-factor">
          <span>{factor}: {value}</span>
          <button on:click={() => removeExtraFactor(factor)}>Remove</button>
        </div>
      {/each}
      
      <div class="add-factor">
        <input 
          type="text" 
          placeholder="Factor name" 
          bind:value={extraFactorName}
        />
        <input 
          type="range" 
          bind:value={extraFactorValue} 
          min={MIN_SLIDER} 
          max={MAX_SLIDER} 
          step="1"
        />
        <span>{extraFactorValue}</span>
        <button on:click={addExtraFactor}>Add Factor</button>
      </div>
    </div>
    
    <button class="calculate-btn" on:click={calculateTotalRisk}>Compute Risk Score</button>
  </div>
</div>

<style>
  .risk-dashboard {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }
  
  h2 {
    margin-top: 0;
    color: #2c3e50;
    border-bottom: 2px solid #eee;
    padding-bottom: 0.5rem;
  }
  
  .risk-meter {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 2rem 0;
  }
  
  .risk-score {
    width: 150px;
    height: 150px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: conic-gradient(
      from 225deg,
      #27ae60 0%,
      #f1c40f 33%,
      #e74c3c 66%
    );
    position: relative;
    --risk-level: 0;
  }
  
  .risk-score::before {
    content: '';
    width: 125px;
    height: 125px;
    border-radius: 50%;
    background-color: white;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
  
  .risk-value {
    position: relative;
    z-index: 1;
    font-size: 2rem;
    font-weight: bold;
    color: var(--risk-value-color, #2c3e50);
  }
  
  .risk-labels {
    display: flex;
    justify-content: space-between;
    width: 150px;
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: #777;
  }
  
  .risk-factors {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }
  
  .factor {
    display: flex;
    flex-direction: column;
  }
  
  .factor label {
    margin-bottom: 0.3rem;
    font-weight: bold;
    color: #555;
  }
  
  .slider-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
    color: #777;
    margin-top: 0.2rem;
  }
  
  .extra-factors {
    border-top: 1px solid #eee;
    padding-top: 1rem;
    margin-top: 1rem;
  }
  
  .extra-factor {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    background-color: #f5f5f5;
    border-radius: 4px;
  }
  
  .add-factor {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin-top: 1rem;
  }
  
  .calculate-btn {
    background-color: #2980b9;
    color: white;
    border: none;
    padding: 0.8rem;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
    transition: background-color 0.3s;
    margin-top: 1.5rem;
  }
  
  .calculate-btn:hover {
    background-color: #3498db;
  }
  
  button {
    background-color: #e74c3c;
    color: white;
    border: none;
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    cursor: pointer;
  }
  
  input[type="text"],
  input[type="number"] {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
  
  input[type="range"] {
    width: 100%;
  }
</style> 