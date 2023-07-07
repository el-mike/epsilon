import { PlayerType } from './player-type';

export class Player {
  public type: PlayerType;

  public constructor(type: PlayerType) {
    this.type = type;
  }
}
