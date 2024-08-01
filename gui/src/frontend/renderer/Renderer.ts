import { StageManager } from './StageManager';

export abstract class Renderer<TGameObject> {
  public constructor(protected _stageManager: StageManager) {}

  public abstract render(gameObject: TGameObject): void;
}
