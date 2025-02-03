// sqlite

declare type SQLiteConnection = {
	query<T>(this: SQLiteConnection, query: string): T[];
};

declare type SQLite = {
	connect(this: void, path: string): SQLiteConnection;
};

declare const sqlite: SQLite;