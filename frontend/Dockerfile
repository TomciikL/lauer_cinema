FROM oven/bun:1.1

WORKDIR /app

COPY bun.lock ./
COPY package.json ./
RUN bun install

COPY . .

EXPOSE 5173

CMD ["bun", "run", "dev", "--", "--host"]
