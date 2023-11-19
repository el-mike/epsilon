import { BoardSquare } from "../board-square";
import { BOARD_SIZE, BoardSquareColor } from "../../common";
import { GameObject } from "../GameObject";

export type State = BoardSquare[][];

export type IterateFn = (
  square: BoardSquare,
  rank: number,
  file: number,
) => void;

export class Board extends GameObject {
  private readonly _state: State;

  public constructor() {
    super();

    this._state = [[], [], [], [], [], [], [], []];

    this.iterate((_, rank, file) => {
      const isDarkSquare = (rank + file) % 2 !== 0;

      const square = new BoardSquare(
        isDarkSquare ? BoardSquareColor.Dark : BoardSquareColor.Light,
        rank,
        file,
      );

      this._state[rank][file] = square;
    });
  }

  public iterate(callback: IterateFn) {
    for (let rank = 0; rank < BOARD_SIZE; rank += 1) {
      for (let file = 0; file < BOARD_SIZE; file += 1) {
        callback(this._state[rank][file], rank, file);
      }
    }
  }

  private _getIndex(rank: number, file: number) {
    return rank * BOARD_SIZE + file;
  }
}
