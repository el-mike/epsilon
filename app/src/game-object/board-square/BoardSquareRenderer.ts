import { Renderer } from "../../renderer";
import { BoardSquare } from "./BoardSquare";
import Konva from "konva";
import { BOARD_SIZE } from "../../common";

export class BoardSquareRenderer extends Renderer<BoardSquare> {
  public render(square: BoardSquare, layer: Konva.Layer) {
    const squareWidth = this._config.boardWidth / BOARD_SIZE;
    const squareHeight = this._config.boardHeight / BOARD_SIZE;

    const rect = new Konva.Rect({
      x: square.file * squareWidth,
      y: square.rank * squareHeight,
      height: squareHeight,
      width: squareWidth,
      fill: square.isDark()
        ? this._config.darkSquareColor
        : this._config.lightSquareColor,
    });

    layer.add(rect);
  }
}
