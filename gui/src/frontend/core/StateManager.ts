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

  public async init() {
    window.backendAPI.onNewState(payload => this.update(payload.state));

    this.update(await this.getCurrentState());
  }

  public update(newGameState: GameState) {
    this._gameState = newGameState;
    this._eventBus.dispatch(GameEvent.STATE_UPDATED, { state: this._gameState });
  }

  public async getCurrentState() {
    const gameState = (await window.backendAPI.getCurrentState()).result;

    return gameState;
  }

  public async makeMove() {
    const result = (await window.backendAPI.makeMove({})).result;

    return result;
  }
}
