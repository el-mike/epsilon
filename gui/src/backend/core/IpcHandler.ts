import {
  BrowserWindow,
  ipcMain
} from 'electron';
import { IpcEvent } from '@common/models/ipc-event';
import {
  EngineBoardState,
  GameState,
  IpcMessage,
  PieceColor,
  PieceKind
} from '@common/models';
import { mapBitboardsToSquares } from '@backend/engine';

export class IpcHandler {
  public constructor(private _mainWindow: BrowserWindow) {}

  public listen() {
    ipcMain.handle(IpcEvent.GetCurrentState, async () => {
      const engineBoardState = {
        [PieceColor.White]: {
          [PieceKind.Pawn]: BigInt(0x000000000000FF00),
          [PieceKind.Knight]: BigInt(0x0000000000000042),
          [PieceKind.Bishop]: BigInt(0x0000000000000024),
          [PieceKind.Rook]: BigInt(0x0000000000000000081),
          [PieceKind.Queen]: BigInt(0x0000000000000000008),
          [PieceKind.King]: BigInt(0x0000000000000000010),
        },
        [PieceColor.Black]: {
          [PieceKind.Pawn]: BigInt(0x00FF000000000000),
          [PieceKind.Knight]: BigInt(0x4200000000000000),
          [PieceKind.Bishop]: BigInt(0x2400000000000000),
          [PieceKind.Rook]: BigInt(0x8100000000000000),
          [PieceKind.Queen]: BigInt(0x0800000000000000),
          [PieceKind.King]: BigInt(0x1000000000000000)
        },
      } as EngineBoardState;

      return this.createIpcMessage({
        squares: mapBitboardsToSquares(engineBoardState),
      } as GameState, null);
    });

    ipcMain.handle(IpcEvent.MakeMove, async () => {
      return this.createIpcMessage('works', null);
    });
  }

  public send(event: IpcEvent, payload: unknown) {
    this._mainWindow.webContents.send(event, payload);
  }

  private createIpcMessage = <TResult = unknown, TError = unknown>(result: TResult, error: TError) => {
    return {
      result: result || null,
      error: error || null,
    } as IpcMessage<TResult>;
  };
}
