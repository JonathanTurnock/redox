// @ts-types="npm:@types/express@4.21.2"
import express from "npm:express@4.21.2";

const app = express();

app.get("/", (req, res) => {
  res.json({
    id: 1,
    name: "Alice",
    uuid: crypto.randomUUID()
  });
});

app.listen(8080);