import Konva from "konva";

import { Config } from "../config";
import { BoardRenderer } from "../game-object";
import { State } from "./State";

export class Game {
  private _state: State = new State();
  private _mainStage: Konva.Stage;

  /* @TODO: Introduce renderer provider. */
  private _boardRenderer: BoardRenderer;

  public constructor(private readonly _config: Config) {
    this._mainStage = new Konva.Stage({
      container: this._config.containerId,
      width: this._config.boardWidth,
      height: this._config.boardHeight,
    });

    this._boardRenderer = new BoardRenderer(_config);
  }

  public start() {
    this._render();
  }

  private _render() {
    const board = this._state.getBoard();

    const mainLayer = new Konva.Layer();

    this._boardRenderer.render(board, mainLayer);

    this._mainStage.add(mainLayer);
    this._mainStage.draw();
  }
}
