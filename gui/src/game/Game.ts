import {
  EventBus,
  StageManager,
} from '../core';
import { Board } from '../game-object';
import { TextureService } from '../texture/TextureService';
import {
  GameEvent,
  GameState,
  PieceColor,
  PieceKind
} from '../common';
import { EngineBoardState } from '../common/models/engine';
import { mapBitboardsToSquares } from '../engine/mappers';

export class Game {
  private _texturesService: TextureService = TextureService.getInstance();
  private _stageManager: StageManager = new StageManager();
  private _eventBus: EventBus = EventBus.getInstance();

  public async start() {
    await this._preload();

    this._eventBus.register(GameEvent.STATE_UPDATED, (args: { state: GameState }) => this._render(args.state));

    const gameState = this.getGameState();

    this._render(gameState);
  }

  /**
   * @TODO:
   * Connect with backend via IPC.
   */
  private getGameState(): GameState {
    const engineBoardState = {
      [PieceColor.White]: {
        [PieceKind.Pawn]: BigInt(0x000000000000FF00),
        [PieceKind.Knight]: BigInt(0x0000000000000042),
        [PieceKind.Bishop]: BigInt(0x0000000000000024),
        [PieceKind.Rook]: BigInt(0x0000000000000000081),
        [PieceKind.Queen]: BigInt(0x0000000000000000008),
        [PieceKind.King]: BigInt(0x0000000000000000010),
      },
      [PieceColor.Black]: {
        [PieceKind.Pawn]: BigInt(0x00FF000000000000),
        [PieceKind.Knight]: BigInt(0x4200000000000000),
        [PieceKind.Bishop]: BigInt(0x2400000000000000),
        [PieceKind.Rook]: BigInt(0x8100000000000000),
        [PieceKind.Queen]: BigInt(0x0800000000000000),
        [PieceKind.King]: BigInt(0x1000000000000000)
      },
    } as EngineBoardState;


    return {
      squares: mapBitboardsToSquares(engineBoardState),
    } as GameState;
  }

  private async _preload() {
    await this._texturesService.load();
  }

  private _render(state: GameState) {
    const board = new Board(state.squares);

    board.init();
    board.render(this._stageManager);

    this._stageManager.draw();
  }
}
