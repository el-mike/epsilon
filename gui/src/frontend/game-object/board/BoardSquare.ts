import {
  Bishop,
  King,
  Knight,
  Pawn,
  PieceObject,
  Queen,
  Rook
} from '../piece';
import {
  Piece,
  PieceKind,
  SquareColor
} from '@common/models';
import { GameObject } from '../GameObject';
import { config } from '../../config';
import Konva from 'konva';
import { StageManager } from '../../core';

export class BoardSquare extends GameObject {
  private _rect: Konva.Rect;

  public constructor(
    private _color: SquareColor,
    private _rank: number,
    private _file: number,
    private _piece: PieceObject | null = null,
  ) {
    super();
  }

  public destroy() {
    this._rect.destroy();
  }

  public init(piece?: Piece) {
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


    if (piece) {
      let pieceObject;

      switch (piece.kind) {
        case PieceKind.Pawn:
          pieceObject = new Pawn(piece.color, x, y);
          break;
        case PieceKind.Knight:
          pieceObject = new Knight(piece.color, x, y);
          break;
        case PieceKind.Bishop:
          pieceObject = new Bishop(piece.color, x, y);
          break;
        case PieceKind.Rook:
          pieceObject = new Rook(piece.color, x, y);
          break;
        case PieceKind.Queen:
          pieceObject = new Queen(piece.color, x, y);
          break;
        case PieceKind.King:
          pieceObject = new King(piece.color, x, y);
          break;
      }

      this._piece = pieceObject;
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

  public setPiece(piece: PieceObject) {
    this._piece = piece;
  }

  public isDark() {
    return this._color === SquareColor.Dark;
  }

  public isLight() {
    return this._color === SquareColor.Light;
  }
}
