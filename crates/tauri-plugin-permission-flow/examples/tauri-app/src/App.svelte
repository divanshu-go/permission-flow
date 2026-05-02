<script>
  import {
    Permission,
    startFlow,
    stopCurrentFlow,
  } from 'tauri-plugin-permission-flow-api'

  const permissionOptions = [
    { label: 'Accessibility', value: Permission.Accessibility },
    { label: 'Input Monitoring', value: Permission.InputMonitoring },
    { label: 'Screen Recording', value: Permission.ScreenRecording },
    { label: 'App Management', value: Permission.AppManagement },
    { label: 'Bluetooth', value: Permission.Bluetooth },
    { label: 'Developer Tools', value: Permission.DeveloperTools },
    { label: 'Full Disk Access', value: Permission.FullDiskAccess },
    { label: 'Media & Apple Music', value: Permission.MediaAppleMusic },
  ]

  let permission = $state(Permission.Accessibility)
  let appPath = $state('/System/Applications/TextEdit.app')
  let useClickSourceFrame = $state(true)
  let status = $state('Idle')

  function stamp(message) {
    status = `[${new Date().toLocaleTimeString()}] ${message}`
  }

  async function handleStartFlow() {
    try {
      await startFlow({
        permission,
        appPath,
        useClickSourceFrame,
      })
      stamp(`Started ${permission} flow for ${appPath}`)
    } catch (error) {
      stamp(error instanceof Error ? error.message : String(error))
    }
  }

  async function handleStopCurrentFlow() {
    try {
      await stopCurrentFlow()
      stamp('Closed the current flow')
    } catch (error) {
      stamp(error instanceof Error ? error.message : String(error))
    }
  }
</script>

<main class="container">
  <div class="card">
    <p class="eyebrow">Tauri Plugin Demo</p>
    <h1>permission-flow</h1>
    <p class="lede">
      Start the macOS permission flow from your Tauri frontend.
    </p>

    <label class="field">
      <span>Permission</span>
      <select bind:value={permission}>
        {#each permissionOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </label>

    <label class="field">
      <span>Suggested app path</span>
      <input bind:value={appPath} placeholder="/Applications/MyApp.app" />
    </label>
    <p class="hint">
      Built-in macOS apps like TextEdit usually live under
      <code>/System/Applications</code>.
    </p>

    <label class="toggle">
      <input type="checkbox" bind:checked={useClickSourceFrame} />
      <span>Use the click source frame animation</span>
    </label>

    <div class="actions">
      <button class="primary" onclick={handleStartFlow}>Start flow</button>
      <button class="secondary" onclick={handleStopCurrentFlow}>Stop flow</button>
    </div>

    <p class="status">{status}</p>
  </div>
</main>

<style>
  .container {
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: 2rem;
  }

  .card {
    width: min(100%, 36rem);
    padding: 2rem;
    border-radius: 1.25rem;
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 20px 60px rgba(15, 23, 42, 0.12);
  }

  .eyebrow {
    margin: 0 0 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 0.78rem;
    color: #475569;
  }

  h1 {
    margin: 0;
  }

  .lede {
    margin: 0.75rem 0 1.5rem;
    color: #334155;
  }

  .field {
    display: grid;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .field span,
  .toggle {
    color: #0f172a;
  }

  select,
  input {
    width: 100%;
    border: 1px solid #cbd5e1;
    border-radius: 0.85rem;
    padding: 0.85rem 1rem;
    background: white;
  }

  .toggle {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin: 1rem 0 1.5rem;
  }

  .hint {
    margin: -0.4rem 0 1rem;
    color: #64748b;
    font-size: 0.92rem;
  }

  .hint code {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  button {
    border: 0;
    border-radius: 999px;
    padding: 0.8rem 1.2rem;
    cursor: pointer;
  }

  .primary {
    background: #0f172a;
    color: white;
  }

  .secondary {
    background: #e2e8f0;
    color: #0f172a;
  }

  .status {
    margin-top: 1rem;
    color: #475569;
  }
</style>
