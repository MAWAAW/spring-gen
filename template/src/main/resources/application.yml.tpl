spring:
  application:
    name: {{APPLICATION_NAME}}
  threads:
    virtual:
      enabled: true
{{DB_CONFIG}}

management:
  endpoints:
    web:
      exposure:
        include: health,info,metrics
  endpoint:
    health:
      show-details: when-authorized

server:
  port: {{SERVER_PORT}}
  error:
    include-message: never
    include-binding-errors: never

jwt:
  secret: ${JWT_SECRET:{{JWT_SECRET}}}
  expiration: 86400000

cors:
  allowed-origins: ${CORS_ORIGINS:http://localhost:4200}
  allowed-methods: GET,POST,PUT,DELETE,OPTIONS
  allowed-headers: Authorization,Content-Type
  exposed-headers: Authorization
  allow-credentials: true
  max-age: 3600

logging:
  level:
    root: INFO
    "{{BASE_PACKAGE}}": INFO
    org.springframework.security: INFO
