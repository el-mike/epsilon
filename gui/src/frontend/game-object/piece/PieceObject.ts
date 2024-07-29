import Konva from 'konva';
import { PieceColor } from '@common/models';
import { GameObject } from "../GameObject";
import { StageManager } from "../../core";
import { config } from '../../config';
import { TextureService } from '../../texture/TextureService';

export abstract class PieceObject extends GameObject {
  protected _textureService = TextureService.getInstance();

  protected _image: Konva.Image;

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

  public init() {
    const squareSize = config.board.width / config.board.size;
    const size = squareSize * 0.75;
    const padding = (squareSize - size) / 2;

    const image = this.getImage();

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

  protected abstract getImage(): HTMLImageElement;
}
