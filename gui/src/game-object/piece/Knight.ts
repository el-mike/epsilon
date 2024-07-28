import { Piece } from '../piece';
import { Texture } from '../../texture/texture.enum';

export class Knight extends Piece {
  protected getImage(): HTMLImageElement {
    return this._textureService.getImage(this.isWhite() ? Texture.WhiteKnight : Texture.BlackKnight);
  }
}
