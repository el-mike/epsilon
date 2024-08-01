import { GameState } from '../state';
import { Renderer } from './Renderer';
import { BoardRenderer } from './BoardRenderer';
import { StageManager } from './StageManager';

export class GameRenderer extends Renderer<GameState> {
  private _boardRenderer: BoardRenderer;

  public constructor() {
    const stageManager = new StageManager()

    super(stageManager);

    this._boardRenderer = new BoardRenderer(stageManager);
  }

  public render(gameState: GameState) {
    this._boardRenderer.render(gameState.boardState);

    this._stageManager.draw();
  }
}
