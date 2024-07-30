// See the Electron documentation for details on how to use preload scripts:
// https://www.electronjs.org/docs/latest/tutorial/process-model#preload-scripts

import { contextBridge, ipcRenderer } from 'electron';
import {
  IpcEvent,
  MakeMovePayload,
  NewStatePayload,
} from '@common/models/ipc-event';
import {
  GameState,
  IpcMessage
} from '@common/models';

declare global {
  interface Window {
    backendAPI: {
      onNewState: (callback: (payload: NewStatePayload) => void) => void,
      getCurrentState: () => IpcMessage<GameState>,
      makeMove: (args: MakeMovePayload) => IpcMessage<string>
    };
  }
}

contextBridge.exposeInMainWorld('backendAPI', {
  onNewState: (callback: (payload: NewStatePayload) => void) => {
    ipcRenderer.on(IpcEvent.NewState, (_, value: NewStatePayload) => callback(value))
  },

  getCurrentState: async () => {
    const result = await ipcRenderer.invoke(IpcEvent.GetCurrentState) as IpcMessage<GameState>;

    return result;
  },
  makeMove: async (payload: MakeMovePayload) => {
    const result = await ipcRenderer.invoke(IpcEvent.MakeMove, payload) as IpcMessage<string>;

    return result;
  }
});
