import { Piece } from '../piece';
import { Texture } from '../../texture/texture.enum';

export class Rook extends Piece {
  protected getImage(): HTMLImageElement {
    return this._textureService.getImage(this.isWhite() ? Texture.WhiteRook : Texture.BlackRook);
  }
}
