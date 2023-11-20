import { PieceColor } from "../../common";
import { GameObject } from "../GameObject";
import { StageManager } from "../../stage";

export abstract class Piece extends GameObject {
  public constructor(
    protected _color: PieceColor,
    protected _x: number,
    protected _y: number,
  ) {
    super();
  }

  public isWhite() {
    return this._color === PieceColor.White;
  }

  public isBlack() {
    return this._color === PieceColor.Black;
  }

  public destroy() {}

  public init() {}

  public render(stageManager: StageManager) {}
}
