import {
  EventBus,
  StageManager,
} from '../core';
import { Board } from '../game-object';
import { TextureService } from '../texture/TextureService';
import {
  GameEvent,
  GameState,
} from '@common/models';

export class Game {
  private _texturesService: TextureService = TextureService.getInstance();
  private _stageManager: StageManager = new StageManager();
  private _eventBus: EventBus = EventBus.getInstance();

  public async start() {
    await this._preload();

    this._eventBus.register(GameEvent.STATE_UPDATED, (args: { state: GameState }) => this._render(args.state));

    const gameState = (await window.electronAPI?.getCurrentState()).result;

    this._render(gameState);
  }


  private async _preload() {
    await this._texturesService.load();
  }

  private _render(state: GameState) {
    const board = new Board(state.squares);

    board.init();
    board.render(this._stageManager);

    this._stageManager.draw();
  }
}
