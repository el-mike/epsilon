import {
  Square,
  SquareCoord
} from './square';


export type GameState = {
  squares: Square[];
  activeSquare?: SquareCoord;
};
