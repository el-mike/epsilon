import { EngineBoardState } from '../common/models/engine';
import {
  getSquareCoordIndices,
  Piece,
  PIECE_COLORS,
  PIECE_KINDS,
  Square,
  SquareColor
} from '../common';

export const mapBitboardsToSquares = (engineBoardState: EngineBoardState): Square[] => {
  const squares: Square[] = [];

  for (const squareIndex of getSquareCoordIndices()) {
    let piece: Piece | null = null;

    for (const color of PIECE_COLORS) {
      for (const kind of PIECE_KINDS) {
        const bitboard = engineBoardState[color][kind];

        const bitmask = 1 << squareIndex;

        if (bitboard && bitboard & BigInt(bitmask)) {
          piece = {
            color,
            kind,
          };
        }
      }
    }

    squares[squareIndex] = {
      piece,
      color: (squareIndex + Math.floor(squareIndex / 8)) % 2 === 0 ? SquareColor.Dark : SquareColor.Light
    } as Square;
  }

  return squares;
};
