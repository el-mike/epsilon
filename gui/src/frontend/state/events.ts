import { GameState } from '@frontend/state/GameState';

export enum StateEvent {
  STATE_UPDATED = 'state-updated'
}

export type StateUpdatedArgs = {
  state: GameState;
};
