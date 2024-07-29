import {
  GameEvent,
  GameState
} from '@common/models';
import { EventBus } from './EventBus';

export class StateManager {
  private static _instance: StateManager;

  private _eventBus = EventBus.getInstance();
  private _gameState: GameState;

  private constructor() {}

  public static getInstance() {
    if (!StateManager._instance) {
      StateManager._instance = new StateManager();
    }

    return StateManager._instance;
  }

  public init(gameState: GameState) {
    this._gameState = gameState;
  }

  public update() {
    this._eventBus.dispatch(GameEvent.STATE_UPDATED, { state: this._gameState });
  }
}
