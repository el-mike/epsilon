import { Config } from './config';
import { Board } from './board';

export class Game {
  private _config: Config;
  private _board: Board;

  public constructor(config: Config) {
    this._config = config;
  }

  public start() {
    this._board = Board.getInstance(this._config);
  
    this._board.render();
  }
}
