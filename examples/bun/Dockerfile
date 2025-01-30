FROM oven/bun:latest

COPY index.ts /app/index.ts
COPY package.json /app/package.json
COPY tsconfig.json /app/tsconfig.json

WORKDIR /app

RUN bun install

CMD [ "bun", "." ]