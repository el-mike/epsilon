import { Board } from '../board';

export interface GameRenderer {
  render(board: Board): void;
}
