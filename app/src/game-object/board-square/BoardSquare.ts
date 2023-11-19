import { Piece } from "../piece";
import { BoardSquareColor } from "../../common/models";
import { GameObject } from "../GameObject";

export class BoardSquare extends GameObject {
  public constructor(
    private _color: BoardSquareColor,
    private _rank: number,
    private _file: number,
    private _piece: Piece | null = null,
  ) {
    super();
  }

  public get rank() {
    return this._rank;
  }

  public get file() {
    return this._file;
  }

  public setPiece(piece: Piece) {
    this._piece = piece;
  }

  public isDark() {
    return this._color === BoardSquareColor.Dark;
  }

  public isLight() {
    return this._color === BoardSquareColor.Light;
  }
}
