import { Piece } from '../piece';
import { Texture } from '../../texture/texture.enum';

export class Bishop extends Piece {
  protected getImage(): HTMLImageElement {
    return this._textureService.getImage(this.isWhite() ? Texture.WhiteBishop : Texture.BlackBishop);
  }
}
