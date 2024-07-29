import { PieceObject } from '../piece';
import { PieceColor } from '../../common';
import { Texture } from '../../texture/texture.enum';

export class Pawn extends PieceObject {
  public constructor(color: PieceColor, x: number, y: number) {
    super(color, x, y);
  }

  protected getImage(): HTMLImageElement {
    return this._textureService.getImage(this.isWhite() ? Texture.WhitePawn : Texture.BlackPawn);
  }
}
