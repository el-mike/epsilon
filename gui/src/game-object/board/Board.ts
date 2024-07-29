import { BoardSquare } from "./BoardSquare";
import {
  Square,
  SquareColor
} from '../../common';
import { GameObject } from "../GameObject";
import { config } from '../../config';
import { StageManager } from '../../core';

export type IterateFn = (
  square: BoardSquare,
  rank: number,
  file: number,
) => void;

export class Board extends GameObject {
  private _boardSquares: BoardSquare[] = [];

  public constructor(private _squares: Square[]) {
    super();
  }

  public init() {
    for (let i = 0; i < this._squares.length; i += 1) {
      const square = this._squares[i];
      const rank = Math.floor(i / config.board.size);
      const file = i % config.board.size;

      const boardSquare = new BoardSquare(square.color, rank ,file);

      boardSquare.init(square.piece);

      this._boardSquares.push(boardSquare);
    }
  }

  public render(stageManager: StageManager) {
    this.iterate(square => square.render(stageManager));
  }

  public destroy() {
    this.iterate(square => square.destroy());
  }

  public iterate(callback: IterateFn) {
    for (let i = 0; i < this._boardSquares.length; i += 1) {
      const rank = Math.floor(i / config.board.size);
      const file = i % config.board.size;

      callback(this._boardSquares[i], rank, file);
    }
  }

  private _getIndex(rank: number, file: number) {
    return rank * config.board.size + file;
  }
}
