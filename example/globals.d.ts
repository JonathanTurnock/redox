declare function get(
  this: void,
  path: string,
  handler: () => string,
): void;

declare function sleep(this: void, ms: number): void;

declare interface UUID {
  v4(this: void): string;
}

declare interface Requests {
  get(this: void, url: string): string;
}

declare interface Logger {
  debug(this: void, message: string): void;
  info(this: void, message: string): void;
  warn(this: void, message: string): void;
  error(this: void, message: string): void;
}

declare interface Json {
  encode<T>(this: void, data: T): string;
  decode<T>(this: void, data: string): T;
}

declare const uuid: UUID;
declare const requests: Requests;
declare const logger: Logger;
declare const json: Json;
