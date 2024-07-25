import { Piece } from '../piece';
import Konva from 'konva';
import { config } from '../../config';
import { PieceColor } from '../../common';
import { TextureService } from '../../texture/TextureService';
import { Texture } from '../../texture/texture.enum';
import { StageManager } from '../../stage';

export class Pawn extends Piece {
  private _textureService = TextureService.getInstance();

  private _image: Konva.Image;

  public constructor(color: PieceColor, x: number, y: number) {
    super(color, x, y);
  }

  public init() {
    const squareSize = config.board.width / config.board.size;
    const size = squareSize * 0.75;
    const padding = (squareSize - size) / 2;

    const image = this._textureService.getImage(this.isWhite() ? Texture.WhitePawn : Texture.BlackPawn);

    this._image = new Konva.Image({
      x: this._x + padding,
      y: this._y + padding,
      image,
      width: size,
      height: size,
    });
  }

  public render(stageManager: StageManager) {
    stageManager.pieceLayer.add(this._image);
  }

  public destroy() {
    this._image.destroy();
  }
}
