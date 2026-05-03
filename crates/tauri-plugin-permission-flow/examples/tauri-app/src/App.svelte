<script>
  import { onMount } from 'svelte'
  import {
    Permission,
    PermissionAuthorizationState,
    PermissionFlow,
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

  const useClickSourceFrame = true
  let permission = $state(Permission.Accessibility)
  let flow = $state(null)
  let appPath = $state(null)
  let authorizationState = $state(PermissionAuthorizationState.Checking)
  let statusOverride = $state('')

  function uiState() {
    if (!flow) {
      return {
        buttonLabel: 'Preparing…',
        buttonClass: 'primary',
        canStart: false,
      }
    }

    if (!appPath) {
      return {
        buttonLabel: 'Unavailable',
        buttonClass: 'primary',
        canStart: false,
      }
    }

    if (statusOverride) {
      return {
        buttonLabel:
          authorizationState === PermissionAuthorizationState.Granted
            ? 'Granted'
            : 'Grant Access',
        buttonClass:
          authorizationState === PermissionAuthorizationState.Granted
            ? 'primary granted'
            : 'primary',
        canStart: authorizationState !== PermissionAuthorizationState.Checking,
      }
    }

    switch (authorizationState) {
      case PermissionAuthorizationState.Granted:
        return {
          buttonLabel: 'Granted',
          buttonClass: 'primary granted',
          canStart: true,
        }
      case PermissionAuthorizationState.Checking:
        return {
          buttonLabel: 'Checking…',
          buttonClass: 'primary',
          canStart: false,
        }
      case PermissionAuthorizationState.Unknown:
        return {
          buttonLabel: 'Grant Access',
          buttonClass: 'primary',
          canStart: true,
        }
      default:
        return {
          buttonLabel: 'Grant Access',
          buttonClass: 'primary',
          canStart: true,
        }
    }
  }

  onMount(() => {
    let cancelled = false

    Promise.all([
      PermissionFlow.create(),
      PermissionFlow.suggestedHostAppPath(),
    ])
      .then(([createdFlow, suggestedAppPath]) => {
        if (cancelled) {
          void createdFlow.close()
          return
        }

        flow = createdFlow
        appPath = suggestedAppPath
        statusOverride = ''
      })
      .catch((error) => {
        statusOverride = error instanceof Error ? error.message : String(error)
      })

    return () => {
      cancelled = true
      const currentFlow = flow
      flow = null
      if (currentFlow) {
        void currentFlow.close()
      }
    }
  })

  $effect(() => {
    const selectedPermission = permission
    statusOverride = ''

    return PermissionFlow.watchAuthorizationStatus(
      selectedPermission,
      (nextState) => {
        authorizationState = nextState
        statusOverride = ''
      },
      {
        onError(error) {
          statusOverride = error instanceof Error ? error.message : String(error)
        },
      }
    )
  })

  async function handleStartFlow() {
    if (!flow) {
      statusOverride = 'Preparing the permission flow…'
      return
    }

    if (!appPath) {
      statusOverride = 'No host app was detected for this launch.'
      return
    }

    try {
      statusOverride = ''
      await flow.startFlow({
        permission,
        appPath,
        useClickSourceFrame,
      })
    } catch (error) {
      statusOverride = error instanceof Error ? error.message : String(error)
    }
  }
</script>

<main class="container">
  <section class="stack">
    <select bind:value={permission}>
      {#each permissionOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>

    <button
      class={uiState().buttonClass}
      disabled={!uiState().canStart}
      onclick={handleStartFlow}
      type="button"
    >
      {uiState().buttonLabel}
    </button>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family:
      'SF Pro Display',
      'Avenir Next',
      'Segoe UI',
      sans-serif;
    background: #2b2b2b;
  }

  .container {
    min-height: 100vh;
    display: grid;
    place-items: center;
    padding: 1.5rem;
  }

  .stack {
    width: min(100%, 16rem);
    display: grid;
    gap: 0.9rem;
  }

  select {
    width: 100%;
    height: 2.5rem;
    box-sizing: border-box;
    padding: 0 0.85rem;
    background: white;
    color: #0f172a;
    font-size: 0.95rem;
  }

  button {
    width: 100%;
    min-height: 2.9rem;
    border: 0;
    border-radius: 999px;
    padding: 0.75rem 1rem;
    font-size: 0.98rem;
    font-weight: 650;
    cursor: pointer;
    transition:
      background-color 140ms ease,
      box-shadow 140ms ease,
      opacity 140ms ease;
  }

  .primary {
    background: #0f172a;
    color: white;
    box-shadow: 0 12px 30px rgba(15, 23, 42, 0.18);
  }

  .primary.granted {
    background: #15803d;
    box-shadow: 0 12px 30px rgba(21, 128, 61, 0.22);
  }

  button:disabled {
    cursor: default;
    opacity: 0.7;
  }
</style>
