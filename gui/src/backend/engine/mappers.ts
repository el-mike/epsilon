import {
  Piece,
  PIECE_COLORS,
  PIECE_KINDS,
  Square,
  SquareColor,
  EngineBoardState
} from '@common/models';

export const mapBitboardsToSquares = (engineBoardState: EngineBoardState): Square[] => {
  const squares: Square[] = [];

  let file = 0;
  let rank = 7;

  while (true) {
    const bitIndex = rank * 8 + file;

    let piece: Piece | null = null;

    for (const color of PIECE_COLORS) {
      for (const kind of PIECE_KINDS) {
        const bitboard = engineBoardState[color][kind];

        const bitmask = BigInt(1) << BigInt(bitIndex);

        if (bitboard && bitboard & bitmask) {
          piece = {
            color,
            kind,
          };
        }
      }
    }

    /**
     * Bitboards are Big Endian, and since we are rendering the board from last rank,
     * we need to reverse the index.
     */
    squares[bitIndex] = {
      piece,
      color: (rank + file) % 2 === 0 ? SquareColor.Dark : SquareColor.Light
    } as Square;

    file += 1;

    if (file >= 8) {
      file = 0;

      if (rank !== 0) {
        rank -= 1;
      } else {
        break;
      }
    }
  }

  return squares;
};
