import * as path from 'path';
import { app } from 'electron';
import {
  Library,
  LibraryObject,
  LibraryFunctionOptions
} from 'ffi-napi';

import { Type } from 'ref-napi';

export type EngineLibDefinition = {
  evaluate: [Type<string>, [], LibraryFunctionOptions],
  get_available_moves: [Type<string>, [], LibraryFunctionOptions],
  factorial: [Type<number | string>, [], LibraryFunctionOptions];
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
      'get_available_moves': ['char *', []],
      'factorial': ['uint64', []]
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
}
