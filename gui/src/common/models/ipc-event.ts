import { GameState } from '@common/models';

export enum IpcEvent {
  NewState = "new-state",
  GetCurrentState = "get-current-state",
  MakeMove = "make-move"
}

export type MakeMovePayload = {

};
