import { GameState } from './game-state';

export enum GameEvent {
  STATE_UPDATED = 'state-updated'
}

export type StateUpdatedArgs = {
  state: GameState;
};
