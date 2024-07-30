// See the Electron documentation for details on how to use preload scripts:
// https://www.electronjs.org/docs/latest/tutorial/process-model#preload-scripts

import { contextBridge, ipcRenderer } from 'electron';
import {
  IpcEvent,
  MakeMovePayload,
} from '@common/models/ipc-event';
import {
  GameState,
  IpcMessage
} from '@common/models';

contextBridge.exposeInMainWorld('electronAPI', {
  getCurrentState: async () => {
    const result = await ipcRenderer.invoke(IpcEvent.GetCurrentState) as IpcMessage<GameState>;

    return result;
  },
  makeMove: async (args: MakeMovePayload) => {
    const result = await ipcRenderer.invoke(IpcEvent.MakeMove, args) as IpcMessage<string>;

    return result;
  }
});
