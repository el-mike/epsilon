export class Config {
  public containerId: string;

  public boardWidth: number;
  public boardHeight: number;

  public lightSquareColor: string;
  public darkSquareColor: string;

  public constructor(payload: Partial<Config>) {
    this.containerId = payload.containerId || 'app';

    this.boardWidth = payload.boardWidth || 640;
    this.boardHeight = payload.boardHeight || 640;

    this.lightSquareColor = payload.lightSquareColor || '#F1D9C0';
    this.darkSquareColor = payload.darkSquareColor || '#A97A65';
  }
}
