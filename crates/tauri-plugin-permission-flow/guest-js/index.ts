import { invoke } from '@tauri-apps/api/core'

const PLUGIN_NAME = 'permission-flow'

export const Permission = {
  Accessibility: 'accessibility',
  InputMonitoring: 'inputMonitoring',
  ScreenRecording: 'screenRecording',
  AppManagement: 'appManagement',
  Bluetooth: 'bluetooth',
  DeveloperTools: 'developerTools',
  FullDiskAccess: 'fullDiskAccess',
  MediaAppleMusic: 'mediaAppleMusic',
} as const

export type Permission = (typeof Permission)[keyof typeof Permission]

export interface StartFlowOptions {
  permission: Permission
  appPath: string
  useClickSourceFrame?: boolean
}

export async function startFlow(options: StartFlowOptions): Promise<void> {
  await invoke(`plugin:${PLUGIN_NAME}|start_flow`, {
    payload: options,
  })
}

export async function stopCurrentFlow(): Promise<void> {
  await invoke(`plugin:${PLUGIN_NAME}|stop_current_flow`)
}
