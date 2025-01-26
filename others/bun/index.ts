import { serve } from "bun";
import { Database } from "bun:sqlite";
import QuickLRU from "quick-lru";
const lru = new QuickLRU({ maxSize: 5 });

// Define an in-memory SQLite database
const db = new Database(":memory:");

db.query(`CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)`).run();
db.query(`INSERT INTO users (name) VALUES ('Alice')`).run();

// Define the index handler
const index = () => {
  let indexed = lru.get("index");

  if (!indexed) {
    const result: any = db
      .query(`SELECT id, name FROM users WHERE name = 'Alice'`)
      .get();

    result.uuid = crypto.randomUUID();
    indexed = JSON.stringify(result);
    lru.set("index", indexed);
  }

  return new Response(`Hello, World! ${indexed}`);
};

// Define the todos handler
const getTodos = async () => {
  const result = await fetch("https://jsonplaceholder.typicode.com/todos/1");
  const data = await result.json();

  // Attach a new UUID
  data.id = crypto.randomUUID();

  return new Response(JSON.stringify(data), {
    headers: { "Content-Type": "application/json" },
  });
};

// Start the server
serve({
  fetch(req) {
    const url = new URL(req.url);

    if (url.pathname === "/") {
      return index();
    } else if (url.pathname === "/todos") {
      return getTodos();
    } else {
      return new Response("Not Found", { status: 404 });
    }
  },
  port: 8080,
});

console.log("Server running on http://localhost:8080");
