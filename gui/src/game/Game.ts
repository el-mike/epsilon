import {
  EventBus,
  StageManager,
} from '../core';
import { Board } from '../game-object';
import { TextureService } from '../texture/TextureService';
import {
  GameEvent,
  GameState
} from '../common';

export class Game {
  private _texturesService: TextureService = TextureService.getInstance();
  private _stageManager: StageManager = new StageManager();
  private _eventBus: EventBus = EventBus.getInstance();

  private _board: Board = new Board();

  public async start() {
    await this._preload();

    this._board.init();

    this._eventBus.register(GameEvent.STATE_UPDATED, (args: { state: GameState }) => this._render(args.state));

    this._render({});
  }

  private async _preload() {
    await this._texturesService.load();
  }

  private _render(state: GameState) {
    this._board.render(this._stageManager);

    this._stageManager.draw();
  }
}
