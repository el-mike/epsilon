import Konva from 'konva';
import { Config } from "../config";

export abstract class Renderer<TGameObject> {
  protected constructor(protected _config: Config) {}

  public abstract render(gameObject: TGameObject, layer: Konva.Layer): void;
}
