
export enum PieceColor {
  White,
  Black,
}

export enum PieceKind {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

export type Piece = {
  color: PieceColor;
  kind: PieceKind;
};

export const PIECE_COLORS = [PieceColor.White, PieceColor.Black];
export const PIECE_KINDS = [PieceKind.Pawn, PieceKind.Knight, PieceKind.Bishop, PieceKind.Rook, PieceKind.Queen, PieceKind.King];

export const getPieceColorIndices = () => Object.keys(PieceColor).filter(key => !isNaN(Number(key)));
export const getPieceKindIndices = () => Object.keys(PieceKind).filter(key => !isNaN(Number(key)));
