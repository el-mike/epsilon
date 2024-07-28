import { Piece } from '../piece';
import { Texture } from '../../texture/texture.enum';

export class Queen extends Piece {
  protected getImage(): HTMLImageElement {
    return this._textureService.getImage(this.isWhite() ? Texture.WhiteQueen : Texture.BlackQueen);
  }
}
