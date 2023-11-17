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
};

export class EngineAdapter {
  private _lib: LibraryObject<EngineLibDefinition>;

  private get _libPath() {
    /**
     * @TODO:
     * Copy the lib with webpack.
     */
    return path.join(app.getAppPath(), '../engine/target/release/libengine_lib.so');
  }

  public constructor() {
    this._lib = Library(this._libPath, {
      'evaluate': ['string', []],
      'get_available_moves': ['string', []]
    });
  }

  public evaluate() {
    const evaluation = this._lib.evaluate();

    return evaluation;
  }

  public getAvailableMoves() {
    const moves = this._lib.get_available_moves();

    return moves;
  }
}
