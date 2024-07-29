import { StageManager } from '../core';

export abstract class GameObject {
  protected _rendered: boolean;

  public get rendered() {
    return this._rendered;
  }

  public abstract init(): void;
  public abstract render(stageManager: StageManager): void;
  public abstract destroy(): void;
}
