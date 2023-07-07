import Konva from 'konva';

import { Config } from '../config';

import { Tile } from './Tile';
import { TileType } from './tile-type';

export type State = Tile[][];

export type IterateFn = (square: Tile, rank: number, file: number) => void;

let board: Board | null = null;

export class Board {
  public static BOARD_SIZE = 8;

  public state: State;

  public static getInstance(config: Config) {
    if (!board) {
      board = new Board(config);
    }

    return board;
    
  }
  

  private constructor(private _config: Config) {
    this.state = [
      [],
      [],
      [],
      [],
      [],
      [],
      [],
      []
    ];

    this.iterate((_, rank, file) => {
      const isDarkSquare = ((rank + file) % 2) !== 0;

        const square = new Tile(isDarkSquare ? TileType.DARK : TileType.LIGHT);

        this.state[rank][file] = square;
    });
  }

  public iterate(callback: IterateFn) {
    for (let rank = 0; rank < Board.BOARD_SIZE; rank += 1) {
      for (let file = 0; file < Board.BOARD_SIZE; file += 1) {
        callback(this.state[rank][file], rank, file);
      }
    }
  }

  private _getIndex(rank: number, file: number) {
    return (rank * Board.BOARD_SIZE) + file;
  }
}
