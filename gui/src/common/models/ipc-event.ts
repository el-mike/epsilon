import { EngineState } from './engine';

export enum IpcEvent {
  NewEngineState = "new-engine-state",
  GetCurrentEngineState = "get-current-engine-state",
  MakeMove = "make-move"
}

export type NewEngineStatePayload = {
  state: EngineState;
}

export type MakeMovePayload = {

};
