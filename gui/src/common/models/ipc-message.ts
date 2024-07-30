export type IpcMessage<T> = {
  result: T;
  error: unknown;
};
