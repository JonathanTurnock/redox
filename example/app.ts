
get("/", () => {
  return "Hello World!" + uuid.v4();
});

get("/todos", () => {
  const data = requests.get("https://jsonplaceholder.typicode.com/todos/1");

  const d = json.decode<{ id: number | string }>(data);

  d.id = uuid.v4();

  return json.encode(d);
});
