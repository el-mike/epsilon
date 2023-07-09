import Konva from 'konva';

import { Config } from '../config';
import { Board } from '../board';

import { GameRenderer } from './GameRenderer';

export class CanvasRenderer implements GameRenderer {
  private _stage: Konva.Stage;

  public constructor(private _config: Config) {
    this._stage = new Konva.Stage({
      container: this._config.containerId,
      width: this._config.boardWidth,
      height: this._config.boardHeight,



    });
  }

  public render(board: Board) {
    const squareWidth = this._config.boardWidth / Board.BOARD_SIZE;
    const squareHeight = this._config.boardHeight / Board.BOARD_SIZE;

    const mainLayer = new Konva.Layer();

    board.iterate((square, rank, file) => {
      const rect = new Konva.Rect({
        x: (file * squareWidth),
        y: (rank * squareHeight),
        height: squareHeight,
        width: squareWidth,
        fill: square.isDark() ? this._config.darkSquareColor : this._config.lightSquareColor,
      });


      mainLayer.add(rect);
    });

    this._stage.add(mainLayer);

    this._stage.draw();
  }
}
