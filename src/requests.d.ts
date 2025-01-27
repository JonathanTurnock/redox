// requests

declare type Requests = {
	get<T>(this: void, url: string): T;
	post<T>(this: void, url: string, data: string): T;
	put<T>(this: void, url: string, data: string): T;
	patch<T>(this: void, url: string, data: string): T;
	delete<T>(this: void, url: string): T;
	head<T>(this: void, url: string): T;
};

declare const requests: Requests;