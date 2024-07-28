import { Piece } from "../piece";
import { Texture } from '../../texture/texture.enum';

export class King extends Piece {
  protected getImage(): HTMLImageElement {
    return this._textureService.getImage(this.isWhite() ? Texture.WhiteKing : Texture.BlackKing);
  }
}
