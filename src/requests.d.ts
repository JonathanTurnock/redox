// requests

declare type Response = {
    get_status(this: Response): number;
    get_headers(this: Response): Record<string, string>;
    get_header_value(this: Response, key: string): string | undefined;
    get_text(this: Response): string;
    get_json<T = unknown>(this: Response): T;
};

declare type RequestsClient = {
    post<T = Response>(this: RequestsClient, url: string, data: string): Promise<T>;
    put<T = Response>(this: RequestsClient, url: string, data: string): Promise<T>;
    patch<T = Response>(this: RequestsClient, url: string, data: string): Promise<T>;
    delete<T = Response>(this: RequestsClient, url: string): Promise<T>;
};

declare type Requests = {
    get<T = Response>(this: void, url: string): Promise<T>;
    createClient(this: void): RequestsClient;
};

declare const requests: Requests;
