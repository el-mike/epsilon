import {
  EngineState,
  PIECE_COLORS,
  PIECE_KINDS,
  PieceColor,
  PieceKind,
  Square,
  SquareColor
} from '@common/models';

import { config } from '@common/config';
import { StateObject } from '@frontend/state/StateObject';

export class PieceState extends StateObject{
  public constructor(public color: PieceColor, public kind: PieceKind) {
    super();
  }

  public get isWhite() {
    return this.color === PieceColor.White;
  }
}

export class SquareState extends StateObject{
  public selected = false;

  public constructor(public square: Square, public color: SquareColor, public piece?: PieceState) {
    super();
  }

  public get rank() {
    return Math.floor(this.square / config.board.size);
  }

  public get file() {
    return this.square % config.board.size;
  }

  public get isLight() {
    return this.color === SquareColor.Light;
  }

  public toggleSelection() {
    this.selected = !this.selected;
    this._stateManager.triggerUpdateEvent();
  }
}

export class BoardState extends StateObject{
  public constructor(public squares: SquareState[]) {
    super();
  }
}

export class GameState extends StateObject {
  public constructor(public boardState: BoardState) {
    super();
  }

  public static fromEngineState(engineState: EngineState): GameState {
    const squares: SquareState[] = [];

    let file = 0;
    let rank = 0;

    while (rank < 8) {
      const bitIndex = rank * 8 + file;

      let piece: PieceState | null = null;

      for (const color of PIECE_COLORS) {
        for (const kind of PIECE_KINDS) {
          const bitboard = engineState.boardState[color][kind];

          const bitmask = BigInt(1) << BigInt(bitIndex);

          if (bitboard && bitboard & bitmask) {
            piece = new PieceState(color, kind);
          }
        }
      }

      const square = bitIndex as Square;
      const squareColor = (rank + file) % 2 === 0 ? SquareColor.Dark : SquareColor.Light;

      squares[bitIndex] = new SquareState(square, squareColor, piece);

      file += 1;

      if (file >= 8) {
        file = 0;
        rank += 1;
      }
    }

    const boardState = new BoardState(squares);

    return new GameState(boardState);
  }
}
