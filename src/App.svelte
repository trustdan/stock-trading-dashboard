<script lang="ts">
  import { onMount } from 'svelte';
  import RiskManagementDashboard from './components/RiskManagementDashboard.svelte';
  import StockRatingDashboard from './components/StockRatingDashboard.svelte';
  import TradeCalendar from './components/TradeCalendar.svelte';
  
  let activeTab = 'risk';
  
  function setActiveTab(tab) {
    activeTab = tab;
  }
  
  onMount(() => {
    // Initialize app
  });
</script>

<main>
  <header>
    <h1>Stock Trading Dashboard</h1>
    <nav>
      <ul>
        <li class:active={activeTab === 'risk'}>
          <button on:click={() => setActiveTab('risk')}>Risk Management</button>
        </li>
        <li class:active={activeTab === 'rating'}>
          <button on:click={() => setActiveTab('rating')}>Stock Rating</button>
        </li>
        <li class:active={activeTab === 'calendar'}>
          <button on:click={() => setActiveTab('calendar')}>Trade Calendar</button>
        </li>
      </ul>
    </nav>
  </header>

  <div class="dashboard-container">
    {#if activeTab === 'risk'}
      <RiskManagementDashboard />
    {:else if activeTab === 'rating'}
      <StockRatingDashboard />
    {:else if activeTab === 'calendar'}
      <TradeCalendar />
    {/if}
  </div>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    font-family: Arial, sans-serif;
    color: #333;
  }
  
  header {
    background-color: #2c3e50;
    color: white;
    padding: 1rem;
  }
  
  h1 {
    margin: 0;
    padding: 0;
    font-size: 1.5rem;
  }
  
  nav ul {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 1rem 0 0 0;
  }
  
  nav li {
    margin-right: 1rem;
  }
  
  nav button {
    background: transparent;
    border: none;
    color: #ddd;
    cursor: pointer;
    padding: 0.5rem 1rem;
    border-bottom: 3px solid transparent;
    transition: all 0.3s;
  }
  
  nav li.active button {
    border-bottom: 3px solid #27ae60;
    color: white;
  }
  
  nav button:hover {
    color: white;
  }
  
  .dashboard-container {
    flex: 1;
    padding: 1rem;
    overflow: auto;
    background-color: #f5f5f5;
  }
</style> 