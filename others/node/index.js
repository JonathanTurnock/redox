const express = require("express");
const { randomUUID } = require("crypto"); // Use crypto for UUID
const Database = require("better-sqlite3");

// Create an Express application
const app = express();

// Define an in-memory SQLite database
const db = new Database(":memory:");

// Create and populate the "users" table
db.prepare(`CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)`).run();
db.prepare(`INSERT INTO users (name) VALUES (?)`).run("Alice");

// Query the database for the user "Alice"
const result = db
  .prepare(`SELECT id, name FROM users WHERE name = ?`)
  .get("Alice");

// Route handlers
app.get("/", (req, res) => {
  // Simulating a large object allocation
  let largeObject = {};

  for (let i = 0; i < 1e6; i++) {
    largeObject[i] = `Data-${i}`;
  }

  // Retaining references unnecessarily
  let retainedReference = largeObject;

  // Simulating an unnecessary closure that holds onto memory
  const leakyClosure = () => retainedReference;

  res.send(`Hello, World! ${JSON.stringify(result)}`);
});

// Handle undefined routes
app.use((req, res) => {
  res.status(404).send("Not Found");
});

// Start the server
const PORT = 8080;
app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
