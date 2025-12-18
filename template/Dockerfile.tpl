FROM maven:3.9.6-eclipse-temurin-21-alpine AS build
WORKDIR /app

# Debug : liste les fichiers
RUN ls -la

COPY pom.xml .
COPY src ./src

# Debug : après copy
RUN ls -la src/main/java/
RUN ls -la pom.xml

# Build explicite
RUN mvn clean package -DskipTests

# Debug : vérifie target/
RUN ls -la target/
RUN ls -la target/*.jar

FROM eclipse-temurin:21-jre-alpine
WORKDIR /app
COPY --from=build /app/target/*.jar app.jar
EXPOSE {{SERVER_PORT}}
ENTRYPOINT ["java", "-jar", "app.jar"]
