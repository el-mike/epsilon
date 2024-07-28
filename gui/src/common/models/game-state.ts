import {
  PieceColor,
  PieceType
} from './piece';
import { Bitboard } from './bitboard';

export type ColorBitboards = {
  [key in PieceType]: Bitboard;
};
export type BoardState = {
  [key in PieceColor]: ColorBitboards;
};

export type GameState = {

};
