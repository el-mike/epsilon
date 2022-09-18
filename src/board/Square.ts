import { SquareType } from './square-type';

export class Square {
  public rank: number;
  public file: number;

  public constructor(
    public type: SquareType,
  ) {}

  public isDark() {
    return this.type === SquareType.DARK;
  }

  public isLight() {
    return this.type === SquareType.LIGHT;
  }
}
