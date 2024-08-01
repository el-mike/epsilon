import { EventBus } from '../core';
import { StateManager, StateEvent, GameState } from '../state';
import { GameRenderer, TextureService } from '../renderer';

export class Game {
  private _stateManager = StateManager.getInstance();
  private _texturesService: TextureService = TextureService.getInstance();
  private _eventBus: EventBus = EventBus.getInstance();
  private _gameRenderer = new GameRenderer();

  public async start() {
    this._eventBus.register(StateEvent.STATE_UPDATED, (args: { state: GameState }) => this._gameRenderer.render(args.state));

    await this._preload();
    await this._stateManager.init();
  }

  private async _preload() {
    await this._texturesService.load();
  }
}
