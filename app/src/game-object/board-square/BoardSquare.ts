import { Piece } from '../piece';
import {
  BoardSquareColor,
  PieceColor
} from '../../common/models';
import { GameObject } from '../GameObject';
import { config } from '../../config';
import Konva from 'konva';
import { StageManager } from '../../stage';
import { Pawn } from '../pawn';

export class BoardSquare extends GameObject {
  private _rect: Konva.Rect;

  public constructor(
    private _color: BoardSquareColor,
    private _rank: number,
    private _file: number,
    private _piece: Piece | null = null,
  ) {
    super();
  }

  public destroy() {
    this._rect.destroy();
  }

  public init() {
    const squareWidth = config.board.width / config.board.size;
    const squareHeight = config.board.height / config.board.size;

    const x = this._file * squareWidth;
    const y = this._rank * squareHeight;

    this._rect = new Konva.Rect({
      x,
      y,
      height: squareHeight,
      width: squareWidth,
      fill: this.isLight()
        ? config.board.lightSquareColor
        : config.board.darkSquareColor,
    });


    if (this._rank < 2 || this._rank > 5) {
      this._piece = new Pawn(PieceColor.Black, x, y);

      this._piece.init();
    }
  }

  public render(stageManager: StageManager) {
    if (this.rendered) {
      return;
    }

    stageManager.mainLayer.add(this._rect);

    if (this._piece) {
      this._piece.render(stageManager);
    }

    this._rendered = true;
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
