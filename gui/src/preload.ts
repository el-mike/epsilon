// See the Electron documentation for details on how to use preload scripts:
// https://www.electronjs.org/docs/latest/tutorial/process-model#preload-scripts

import { contextBridge, ipcRenderer } from 'electron';
import {
  IpcEvent,
  MakeMovePayload,
  NewEngineStatePayload,
} from '@common/models/ipc-event';
import {
  EngineState,
  IpcMessage
} from '@common/models';

declare global {
  interface Window {
    backendAPI: {
      onNewState: (callback: (payload: IpcMessage<NewEngineStatePayload>) => void) => void,
      getCurrentState: () => IpcMessage<EngineState>,
      makeMove: (args: MakeMovePayload) => IpcMessage<string>
    };
  }
}

contextBridge.exposeInMainWorld('backendAPI', {
  onNewState: (callback: (payload: IpcMessage<NewEngineStatePayload>) => void) => {
    ipcRenderer.on(IpcEvent.NewEngineState, (_, value: IpcMessage<NewEngineStatePayload>) => callback(value))
  },

  getCurrentState: async () => {
    const result = await ipcRenderer.invoke(IpcEvent.GetCurrentEngineState) as IpcMessage<EngineState>;

    return result;
  },
  makeMove: async (payload: MakeMovePayload) => {
    const result = await ipcRenderer.invoke(IpcEvent.MakeMove, payload) as IpcMessage<string>;

    return result;
  }
});
