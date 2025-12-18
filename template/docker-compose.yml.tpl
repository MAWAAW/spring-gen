version: '3.8'
services:
  db:
    image: {{DB_IMAGE}}
    container_name: {{APP_NAME}}-{{DB_TYPE}}
    environment:
      POSTGRES_DB: appdb
      POSTGRES_USER: appuser
      POSTGRES_PASSWORD: changeme
      MYSQL_DATABASE: appdb
      MYSQL_ROOT_PASSWORD: changeme
    ports:
      - "{{DB_PORT}}:5432"
    volumes:
      - db_data:/var/lib/{{DB_DATA_PATH}}
    healthcheck:
      test: ["CMD-SHELL", "{{DB_HEALTHCHECK}}"]
      interval: 10s
      timeout: 5s
      retries: 5

  backend:
    build: .
    ports:
      - "{{SERVER_PORT}}:8080"
    environment:
      SPRING_DATASOURCE_URL: {{DB_JDBC_URL}}
      SPRING_DATASOURCE_USERNAME: appuser
      SPRING_DATASOURCE_PASSWORD: changeme
      SPRING_JPA_HIBERNATE_DDL_AUTO: update
      SPRING_JPA_DATABASE_PLATFORM: {{DB_DIALECT}}
      SPRING_DATASOURCE_DRIVER_CLASS_NAME: {{DB_DRIVER}}
      JWT_SECRET: {{JWT_SECRET}}
    depends_on:
      db:
        condition: service_healthy

volumes:
  db_data:
