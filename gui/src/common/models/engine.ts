import {
  PieceColor,
  PieceKind
} from './piece';
import { Bitboard } from './bitboard';

export type ColorBitboards = {
  [key in PieceKind]: Bitboard;
};

export type EngineBoardState = {
  [key in PieceColor]: ColorBitboards;
};

export type EngineState = {
  boardState: EngineBoardState;
};
