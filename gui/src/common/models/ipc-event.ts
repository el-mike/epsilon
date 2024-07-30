import { GameState } from '@common/models/game-state';

export enum IpcEvent {
  NewState = "new-state",
  GetCurrentState = "get-current-state",
  MakeMove = "make-move"
}

export type NewStatePayload = {
  state: GameState;
}

export type MakeMovePayload = {

};
