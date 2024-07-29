import * as path from 'path';
import { app } from 'electron';
import {
  Library,
  LibraryObject,
  LibraryFunctionOptions
} from 'ffi-napi';

import { Type } from 'ref-napi';
import {
  EngineBoardState,
  PieceColor,
  PieceKind,
} from '@common/models';

export type EngineLibDefinition = {
  evaluate: [Type<string>, [], LibraryFunctionOptions],
  get_available_moves: [Type<string>, [], LibraryFunctionOptions],
};

export class EngineAdapter {
  private _lib: LibraryObject<EngineLibDefinition>;

  private get _libPath() {
    /**
     * @TODO:
     * Copy the lib with webpack.
     */
    return path.join(app.getAppPath(), '../epsilon/target/release/libepsilon_api.so');
  }

  public constructor() {
    this._lib = Library(this._libPath, {
      'evaluate': ['string', []],
      'get_available_moves': ['char *', []]
    });
  }

  public evaluate() {
    const evaluation = this._lib.evaluate().toString();

    return evaluation;
  }

  public getAvailableMoves() {
    const movesBuf = this._lib.get_available_moves() as any as Buffer;
    const moves = movesBuf.readCString();

    return moves;
  }

  /**
   * @TODO:
   * FFI integration.
   */
  public getBoardState() {
    return {
      [PieceColor.White]: {
        [PieceKind.Pawn]: BigInt(0x000000000000FFFF),
      },
      [PieceColor.Black]: {
        [PieceKind.Pawn]: BigInt(0x00FF000000000000)
      },
    } as EngineBoardState;
  }
}
