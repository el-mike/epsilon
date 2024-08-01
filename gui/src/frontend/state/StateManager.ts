import { EventBus } from '../core/EventBus';
import { GameState } from './GameState';
import { StateEvent } from '@frontend/state/events';

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
    window.backendAPI.onNewState(payload => this.update(GameState.fromEngineState(payload.result.state)));

    this.update(await this.getCurrentState());
  }

  public update(newGameState: GameState) {
    this._gameState = newGameState;
    this._eventBus.dispatch(StateEvent.STATE_UPDATED, { state: this._gameState });
  }

  public triggerUpdateEvent() {
    this._eventBus.dispatch(StateEvent.STATE_UPDATED, { state: this._gameState });
  }

  public async getCurrentState() {
    const engineGameState = (await window.backendAPI.getCurrentState()).result;

    return GameState.fromEngineState(engineGameState);
  }

  public async makeMove() {
    const result = (await window.backendAPI.makeMove({})).result;

    return result;
  }
}
