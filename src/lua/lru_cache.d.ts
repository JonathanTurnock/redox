// lru_cache

declare type LruCacheInstance = {
    get<T>(this: LruCacheInstance, key: string): T;
    set<T>(this: LruCacheInstance, key: string, value: T): void;
}

declare type LruCache = {
    create(this: void, size: number): LruCacheInstance;
}

declare const lru_cache: LruCache;