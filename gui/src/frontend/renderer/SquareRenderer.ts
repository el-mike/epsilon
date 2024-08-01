import Konva from 'konva';

import { config } from '@common/config';

import {
  PieceState,
  SquareState
} from '../state';
import { Renderer } from './Renderer';
import { StageManager } from '@frontend/renderer/StageManager';
import { PieceKind } from '@common/models';
import {
  Texture,
  TextureService
} from '@frontend/renderer/texture';


export class SquareRenderer extends Renderer<SquareState> {
  private _textureService = TextureService.getInstance();

  public constructor(stageManager: StageManager) {
    super(stageManager);
  }

  public render(squareState: SquareState) {
    const squareWidth = config.board.width / config.board.size;
    const squareHeight = config.board.height / config.board.size;

    const x = squareState.file * squareWidth;

    /**
     * Konva's x/y origin is at the top left of the viewport,
     * therefore we need to "inverse" the ranks to get th white's perspective.
     *
     * @TODO:
     * Parametrize to get black's perspective when needed.
     */
    const y = ((config.board.size - 1) - squareState.rank) * squareHeight;

    const fill = squareState.selected
      ? '#ff0000'
      : squareState.isLight
        ? config.board.lightSquareColor
        : config.board.darkSquareColor;

    const rect = new Konva.Rect({
      x,
      y,
      width: squareWidth,
      height: squareHeight,
      fill,
    });

    rect.on('click', () => squareState.toggleSelection());

    if (squareState.piece) {
      this._renderPiece(squareState.piece, x, y);
    }

    this._stageManager.mainLayer.add(rect);
  }

  private _renderPiece(piece: PieceState, x: number, y: number) {
    const squareSize = config.board.width / config.board.size;
    const size = squareSize * 0.75;
    const padding = (squareSize - size) / 2;

    const image = new Konva.Image({
      x: x + padding,
      y: y + padding,
      image:  this._getPieceImage(piece),
      width: size,
      height: size
    });

    this._stageManager.pieceLayer.add(image);
  }

  private _getPieceImage(piece: PieceState): HTMLImageElement {
    switch (piece.kind) {
      case PieceKind.Pawn:
        return this._textureService.getImage(piece.isWhite ? Texture.WhitePawn : Texture.BlackPawn);
      case PieceKind.Knight:
        return this._textureService.getImage(piece.isWhite ? Texture.WhiteKnight : Texture.BlackKnight);
      case PieceKind.Bishop:
        return this._textureService.getImage(piece.isWhite ? Texture.WhiteBishop : Texture.BlackBishop);
      case PieceKind.Rook:
        return this._textureService.getImage(piece.isWhite ? Texture.WhiteRook : Texture.BlackRook);
      case PieceKind.Queen:
        return this._textureService.getImage(piece.isWhite ? Texture.WhiteQueen : Texture.BlackQueen);
      case PieceKind.King:
        return this._textureService.getImage(piece.isWhite ? Texture.WhiteKing : Texture.BlackKing);
    }
  }
}
