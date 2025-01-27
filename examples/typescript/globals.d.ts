// json

declare type Json = {
	encode<T>(this: void, data: T): string;
	decode<T>(this: void, data: string): T;
};

declare const json: Json;// logger

declare type Logger = {
    debug(this: void, message: string): void;
    info(this: void, message: string): void;
    warn(this: void, message: string): void;
    error(this: void, message: string): void;
  }

declare const logger: Logger;// lru_cache

declare type LruCacheInstance = {
    get<T>(this: LruCacheInstance, key: string): T;
    set<T>(this: LruCacheInstance, key: string, value: T): void;
}

declare type LruCache = {
    create(this: void, size: number): LruCacheInstance;
}

declare const lru_cache: LruCache;// main
declare function get(path: string, callback: () => string): void// requests

declare type Requests = {
	get<T>(this: void, url: string): T;
	post<T>(this: void, url: string, data: string): T;
	put<T>(this: void, url: string, data: string): T;
	patch<T>(this: void, url: string, data: string): T;
	delete<T>(this: void, url: string): T;
	head<T>(this: void, url: string): T;
};

declare const requests: Requests;// sqlite

declare type SQLiteConnection = {
	query<T>(this: SQLiteConnection, query: string): T[];
};

declare type SQLite = {
	connect(this: void, path: string): SQLiteConnection;
};

declare const sqlite: SQLite;// uuid

declare type UUID = {
	v4(this: void): string;
};

declare const uuid: UUID;