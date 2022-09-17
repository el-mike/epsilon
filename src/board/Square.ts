import { SquareType } from './square-type';

export class Square {
  public type: SquareType;

  public constructor(type: SquareType) {
    this.type = type;
  }

  public isDark() {
    return this.type === SquareType.DARK;
  }

  public isLight() {
    return this.type === SquareType.LIGHT;
  }
}
