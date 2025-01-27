import express from "express";

const app = express();

app.get("/", (req, res) => {
    res.json({
        id: 1,
        name: "Alice",
        uuid: crypto.randomUUID()
    });
});

app.listen(8080);