import { Texture } from "./texture.enum";

import whitePawn from "@assets/board-pieces/256h/w_pawn_png_256px.png";
import whiteRook from "@assets/board-pieces/256h/w_rook_png_256px.png";
import whiteKnight from "@assets/board-pieces/256h/w_knight_png_256px.png";
import whiteBishop from "@assets/board-pieces/256h/w_bishop_png_256px.png";
import whiteQueen from "@assets/board-pieces/256h/w_queen_png_256px.png";
import whiteKing from "@assets/board-pieces/256h/w_king_png_256px.png";

import blackPawn from "@assets/board-pieces/256h/b_pawn_png_256px.png";
import blackRook from "@assets/board-pieces/256h/b_rook_png_256px.png";
import blackKnight from "@assets/board-pieces/256h/b_knight_png_256px.png";
import blackBishop from "@assets/board-pieces/256h/b_bishop_png_256px.png";
import blackQueen from "@assets/board-pieces/256h/b_queen_png_256px.png";
import blackKing from "@assets/board-pieces/256h/b_king_png_256px.png";

const TEXTURE_SRC_MAP: { [key in Texture]: string } = {
  [Texture.WhitePawn]: whitePawn,
  [Texture.WhiteRook]: whiteRook,
  [Texture.WhiteKnight]: whiteKnight,
  [Texture.WhiteBishop]: whiteBishop,
  [Texture.WhiteQueen]: whiteQueen,
  [Texture.WhiteKing]: whiteKing,
  [Texture.BlackPawn]: blackPawn,
  [Texture.BlackRook]: blackRook,
  [Texture.BlackKnight]: blackKnight,
  [Texture.BlackBishop]: blackBishop,
  [Texture.BlackQueen]: blackQueen,
  [Texture.BlackKing]: blackKing,
};

export class TextureService {
  private static _instance: TextureService;

  private _imagesMap = new Map<Texture, HTMLImageElement>();

  private constructor() {}

  public static getInstance() {
    if (!TextureService._instance) {
      TextureService._instance = new TextureService();
    }


    return TextureService._instance;
  }

  public load() {
    return Promise.all(
      Object.entries(TEXTURE_SRC_MAP).map(
        ([texture, src]: [Texture, string]) =>
          new Promise<void>((resolve, reject) => {
            const image = new Image();

            image.onload = () => {
              this._imagesMap.set(texture, image);
              resolve();
            };

            image.onerror = reject;

            image.src = src;
          }),
      ),
    );
  }

  public getImage(texture: Texture) {
    return this._imagesMap.get(texture);
  }
}
