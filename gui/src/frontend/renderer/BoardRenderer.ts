import { Renderer } from '@frontend/renderer/Renderer';
import { BoardState } from '../state';
import { SquareRenderer } from './SquareRenderer';
import { StageManager } from '@frontend/renderer/StageManager';

export class BoardRenderer extends Renderer<BoardState> {
  private _squareRenderer: SquareRenderer;

  public constructor(stageManager: StageManager) {
    super(stageManager);

    this._squareRenderer = new SquareRenderer(stageManager);

  }

  public render(boardState: BoardState) {
    boardState.squares.forEach(square => this._squareRenderer.render(square));
  }
}
