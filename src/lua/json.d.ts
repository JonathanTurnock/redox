// json

declare type Json = {
	encode<T>(this: void, data: T): string;
	decode<T>(this: void, data: string): T;
};

declare const json: Json;