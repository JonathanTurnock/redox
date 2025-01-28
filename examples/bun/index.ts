import express from "express";
import promBundle from "express-prom-bundle";
import sqlite3 from "bun:sqlite";
import client from 'prom-client';

const app = express();

const metricsMiddleware = promBundle({includeMethod: true});
app.use(metricsMiddleware);


console.info("Opening database connections");
const appDb = sqlite3.open(":memory:");

console.info("Creating tables");
appDb.run(`
    CREATE TABLE IF NOT EXISTS users
    (
        id
        INTEGER
        PRIMARY
        KEY,
        name
        TEXT
    )
`);
appDb.run("INSERT OR REPLACE INTO users (id, name) VALUES (?, ?)", [1, "Alice"]);

// Enable the default metrics
client.collectDefaultMetrics();

app.get("/", (req, res) => {
    try {
        const user: any = appDb.query("SELECT id, name FROM users WHERE id = ?").get(1);
        if (user) {
            user.uuid = crypto.randomUUID();
        }
        res.json(user);
    } catch (error) {
        console.error("Error fetching user:", error);
        res.status(500).send("Error fetching user.");
    }
});

app.listen(8081);