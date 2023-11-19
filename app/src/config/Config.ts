export type Config = {
  containerId: string;
  board: {
    size: number;
    width: number;
    height: number;
    lightSquareColor: string;
    darkSquareColor: string;
  }
};

export const config: Config = {
  containerId: 'app',
  board: {
    size: 8,
    width: 800,
    height: 800,
    lightSquareColor: '#F1D9C0',
    darkSquareColor: '#A97A65'
  }
}
