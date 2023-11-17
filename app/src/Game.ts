import { Config } from './config';
import { Board } from './board';
import { GameRenderer } from './game-renderer';

export class Game {
  private _board: Board;

  public constructor(
    private _config: Config,
    private _renderer: GameRenderer,
  ) {}

  public start() {
    this._board = Board.getInstance(this._config);

    this._renderer.render(this._board);
  }
}
