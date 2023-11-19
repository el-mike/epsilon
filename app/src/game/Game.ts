import { StageManager } from "../stage";
import { Board } from '../game-object';

export class Game {
  private _board: Board = new Board();
  private _stageManager: StageManager = new StageManager();

  public start() {
    this._board.init();

    this._render();
  }

  private _render() {
    this._board.render(this._stageManager);

    this._stageManager.draw();
  }
}
