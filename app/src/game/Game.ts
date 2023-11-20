import { StageManager } from "../stage";
import { Board } from '../game-object';
import { TextureService } from '../texture/TextureService';

export class Game {
  private _texturesService: TextureService = TextureService.getInstance();
  private _stageManager: StageManager = new StageManager();

  private _board: Board = new Board();

  public async start() {
    await this._preload();

    this._board.init();

    this._render();
  }

  private async _preload() {
    await this._texturesService.load();
  }

  private _render() {
    this._board.render(this._stageManager);

    this._stageManager.draw();
  }
}
