import Konva from 'konva';

import { Config } from '../config';

import { Square } from './Square';
import { SquareType } from './square-type';

export type State = Square[][];

const BOARD_SIZE = 8;

let board: Board | null = null;

export class Board {
  private _config: Config;
  private _state: State;

  private _stage: Konva.Stage;

  public static getInstance(config: Config) {
    if (!board) {
      board = new Board(config);
    }

    return board;
  }

  private constructor(config: Config) {
    this._config = config;
  
    this._state = [
      []
    ];

    this._stage = new Konva.Stage({
      container: this._config.containerId,
      width: this._config.boardWidth,
      height: this._config.boardHeight,
    });
  }

  public render() {
    const squareWidth = this._config.boardWidth / BOARD_SIZE;
    const squareHeight = this._config.boardHeight / BOARD_SIZE;
  
    const mainLayer = new Konva.Layer();

    for (let rank = 0; rank < BOARD_SIZE; rank += 1) {
      for (let file = 0; file < BOARD_SIZE; file += 1) {
        const index = this._getIndex(rank, file);

        const isDarkSquare = ((rank + file) % 2) !== 0;

        const square = new Square(isDarkSquare ? SquareType.DARK : SquareType.LIGHT);
        
        const rect = new Konva.Rect({
          x: (file * squareWidth),
          y: (rank * squareHeight),
          height: squareHeight,
          width: squareWidth,
          fill: square.isDark() ? this._config.darkSquareColor : this._config.lightSquareColor,
        });

        mainLayer.add(rect);
      }
    }

    this._stage.add(mainLayer);

    mainLayer.draw();
  }

  private _getIndex(rank: number, file: number) {
    return (rank * BOARD_SIZE) + file;
  }
}
