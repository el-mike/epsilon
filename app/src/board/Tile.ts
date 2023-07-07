import { TileType } from './tile-type';

export class Tile {
  public rank: number;
  public file: number;

  public constructor(
    public type: TileType,
  ) {}

  public isDark() {
    return this.type === TileType.DARK;
  }

  public isLight() {
    return this.type === TileType.LIGHT;
  }
}
