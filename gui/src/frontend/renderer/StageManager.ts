import Konva from "konva";
import { config } from "@common/config";

export class StageManager {
  private _mainStage = new Konva.Stage({
    container: config.containerId,
    width: config.board.width,
    height: config.board.height,
  });

  private _mainLayer = new Konva.Layer();
  private _pieceLayer = new Konva.Layer();
  private _dragLayer = new Konva.Layer();

  public get mainLayer() {
    return this._mainLayer;
  }

  public get pieceLayer() {
    return this._pieceLayer;
  }

  public get dragLayer() {
    return this._dragLayer;
  }

  public constructor() {
    this._mainStage.add(this._mainLayer);
    this._mainStage.add(this._pieceLayer);
    this._mainStage.add(this._dragLayer);
  }

  public draw() {
    this._mainStage.draw();
  }

  public clear() {
    this._mainLayer.removeChildren();
    this._pieceLayer.removeChildren();
    this._dragLayer.removeChildren();
  }
}
