# spring-gen

Spring Boot 4 project generator written in Rust.

Generates a ready-to-use Spring Boot application with Docker, JWT security and database support.

---

## Features

- Spring Boot 4
- Databases: PostgreSQL, MySQL, MongoDB
- Docker & docker-compose
- JWT security
- Clean project structure

---

## Requirements

- Rust
- Java 21
- Maven
- Docker (optional)

---

## Install

```bash
git clone https://github.com/MAWAAW/spring-gen.git
cd spring-gen
cargo build --release
```

---

## Usage
```bash
spring-gen --name myapp --database postgres
```

Run the generated project:
```bash
cd myapp
docker compose up --build
```

---

## License
MIT
