// logger

declare type Logger = {
    debug(this: void, message: string): void;
    info(this: void, message: string): void;
    warn(this: void, message: string): void;
    error(this: void, message: string): void;
  }

declare const logger: Logger;