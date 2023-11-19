import { Board } from '../game-object';

export class State {
  private readonly _board: Board;

  public constructor() {
    this._board = new Board();
  }

  public getBoard() {
    return this._board;
  }
}
