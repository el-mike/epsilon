import { PieceColor } from '../../common';
import { GameObject } from '../GameObject';
import { StageManager } from '../../stage';

export abstract class Piece extends GameObject {
  protected color: PieceColor;

  public isWhite() {
    return this.color === PieceColor.White;
  }

  public isBlack() {
    return this.color === PieceColor.Black;
  }

  public destroy() {

  }

  public init() {

  }

  public render(stageManager: StageManager) {

  }
}
