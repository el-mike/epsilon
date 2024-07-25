import { BoardSquare } from "./BoardSquare";
import { BoardSquareColor } from "../../common";
import { GameObject } from "../GameObject";
import { config } from '../../config';
import { StageManager } from '../../stage';

export type IterateFn = (
  square: BoardSquare,
  rank: number,
  file: number,
) => void;

export class Board extends GameObject {
  private _squares: BoardSquare[][];

  public constructor() {
    super();
  }

  public init() {
    this._squares = [[], [], [], [], [], [], [], []];

    this.iterate((_, rank, file) => {
      const isDarkSquare = (rank + file) % 2 !== 0;

      const square = new BoardSquare(
        isDarkSquare ? BoardSquareColor.Dark : BoardSquareColor.Light,
        rank,
        file,
      );

      square.init();

      this._squares[rank][file] = square;
    });
  }

  public render(stageManager: StageManager) {
    this.iterate(square => square.render(stageManager));
  }

  public destroy() {
    this.iterate(square => square.destroy());
  }

  public iterate(callback: IterateFn) {
    for (let rank = 0; rank < config.board.size; rank += 1) {
      for (let file = 0; file < config.board.size; file += 1) {
        callback(this._squares[rank][file], rank, file);
      }
    }
  }

  private _getIndex(rank: number, file: number) {
    return rank * config.board.size + file;
  }
}
