import { Renderer } from "../../renderer";
import { Board } from "./Board";
import Konva from "konva";
import { Config } from "../../config";
import { BoardSquareRenderer } from "../board-square";

export class BoardRenderer extends Renderer<Board> {
  /* @TODO: Introduce renderer provider. */
  private _boardSquareRenderer: BoardSquareRenderer;

  public constructor(_config: Config) {
    super(_config);

    this._boardSquareRenderer = new BoardSquareRenderer(_config);
  }
  public render(board: Board, layer: Konva.Layer) {
    board.iterate((square) => {
      this._boardSquareRenderer.render(square, layer);
    });
  }
}
