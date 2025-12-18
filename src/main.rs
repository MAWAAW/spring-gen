use clap::{Parser, ValueEnum};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "spring-gen", about = "Générateur Spring Boot 4")]
struct Args {
    #[arg(short, long, default_value = "app")]
    name: String,

    #[arg(short, long, default_value = "com.example.myapp")]
    package: String,

    #[arg(short = 'g', long, default_value = "com.example")]
    group: String,

    #[arg(short = 'a', long, default_value = "myapp")]
    artifact: String,

    #[arg(short, long, value_enum, default_value = "postgres")]
    database: DatabaseType,

    #[arg(short = 'P', long, default_value_t = 8080)]
    server_port: u16,

    #[arg(short, long, default_value = "template")]
    template: String,
}

#[derive(Clone, ValueEnum, Debug)]
enum DatabaseType {
    Postgres,
    Mysql,
    H2,
    Mongo,
}

fn get_db_vars(db: &DatabaseType) -> Vec<(&'static str, &'static str)> {
    match db {
        DatabaseType::Postgres => vec![
            ("{{DB_IMAGE}}", "postgres:17-alpine"),
            ("{{DB_TYPE}}", "postgres"),
            ("{{DB_DATA_PATH}}", "postgresql/data"),
            ("{{DB_HEALTHCHECK}}", "pg_isready -U appuser -d appdb"),
            ("{{DB_JDBC_URL}}", "jdbc:postgresql://db:5432/appdb"),
            ("{{DB_PORT}}", "5432"),
            ("{{DB_DIALECT}}", "org.hibernate.dialect.PostgreSQLDialect"),
            ("{{DB_DRIVER}}", "org.postgresql.Driver"),
        ],
        DatabaseType::Mysql => vec![
            ("{{DB_IMAGE}}", "mysql:8"),
            ("{{DB_TYPE}}", "mysql"),
            ("{{DB_DATA_PATH}}", "mysql/data"),
            ("{{DB_HEALTHCHECK}}", "mysqladmin ping -h localhost -u root -proot"),
            ("{{DB_JDBC_URL}}", "jdbc:mysql://db:3306/appdb?useSSL=false&serverTimezone=UTC"),
            ("{{DB_PORT}}", "3306"),
            ("{{DB_DIALECT}}", "org.hibernate.dialect.MySQLDialect"),
            ("{{DB_DRIVER}}", "com.mysql.cj.jdbc.Driver"),
        ],
        DatabaseType::H2 => vec![
            ("{{DB_IMAGE}}", "h2database/h2:latest"),
            ("{{DB_TYPE}}", "h2"),
            ("{{DB_DATA_PATH}}", "h2/data"),
            ("{{DB_HEALTHCHECK}}", "java -cp /opt/h2/bin/h2*.jar org.h2.tools.Server -web -webPort 8082"),
            ("{{DB_JDBC_URL}}", "jdbc:h2:mem:appdb"),
            ("{{DB_PORT}}", "8082"),
            ("{{DB_DIALECT}}", "org.hibernate.dialect.H2Dialect"),
            ("{{DB_DRIVER}}", "org.h2.Driver"),
        ],
        DatabaseType::Mongo => vec![
            ("{{DB_IMAGE}}", "mongo:7"),
            ("{{DB_TYPE}}", "mongo"),
            ("{{DB_DATA_PATH}}", "mongodb/data"),
            ("{{DB_HEALTHCHECK}}", "mongosh --eval 'db.adminCommand(\"ping\")'"),
            ("{{DB_JDBC_URL}}", "mongodb://db:27017/appdb"),
            ("{{DB_PORT}}", "27017"),
            ("{{DB_DIALECT}}", ""),
            ("{{DB_DRIVER}}", ""),
        ],
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!(
        "🚀 Génération Spring Boot 4: {} (DB: {:?})",
        args.name, args.database
    );

    let template_dir = Path::new(&args.template);
    let output_dir = Path::new(&args.name);

    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }

    let server_port_str = args.server_port.to_string();

    let mut replacements = vec![
        ("{{APPLICATION_NAME}}", &args.name[..]),
        ("{{PACKAGE_NAME}}", &args.package[..]),
        ("{{GROUP_ID}}", &args.group[..]),
        ("{{ARTIFACT_ID}}", &args.artifact[..]),
        ("{{APP_NAME}}", &args.name[..]),
        ("{{BASE_PACKAGE}}", &args.package[..]),
        ("{{JWT_SECRET}}", "myapp-super-secret-key-change-in-prod"),
        ("{{SERVER_PORT}}", &server_port_str),
    ];

    replacements.extend(get_db_vars(&args.database));

    copy_and_process_template(template_dir, output_dir, &replacements)?;
    override_db_config(output_dir, &args.database)?;
    generate_pom_db_dependency(output_dir, &args.database)?;
    restructure_package(output_dir, &args.package)?;

    println!(
        "✅ Projet '{}' généré avec DB {:?}",
        args.name, args.database
    );
    println!("👉 cd {} && docker compose up --build", args.name);
    Ok(())
}


fn copy_and_process_template(
    template_dir: &Path,
    output_dir: &Path,
    replacements: &[(&str, &str)],
) -> anyhow::Result<()> {

    fs::create_dir_all(output_dir)?;

    for entry in WalkDir::new(template_dir).follow_links(true) {
        let entry = entry?;
        let path = entry.path();
        let rel_path = path.strip_prefix(template_dir)?;

        if path.is_dir() {
            let target_dir = output_dir.join(rel_path);
            fs::create_dir_all(&target_dir)?;
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext == "tpl" {
                let out_path = output_dir.join(rel_path.with_extension(""));

                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let content = fs::read_to_string(path)?;
                let processed = replacements.iter().fold(content, |acc, (from, to)| acc.replace(from, to));
                fs::write(&out_path, processed)?;

                println!("✅ Généré: {:?}", out_path);
            }
        }
    }

    Ok(())
}

fn override_db_config(project_dir: &Path, db: &DatabaseType) -> anyhow::Result<()> {
    let yml_path = project_dir
    .join("src")
    .join("main")
    .join("resources")
    .join("application.yml");

    if !yml_path.exists() {
        println!("⚠️  application.yml non trouvé, override DB sauté");
        return Ok(());
    }

    let mut content = fs::read_to_string(&yml_path)?;

    let db_block = match db {
        DatabaseType::Postgres => r#"
  datasource:
    url: ${DATABASE_URL:jdbc:postgresql://localhost:5432/appdb}
    username: ${DATABASE_USERNAME:appuser}
    password: ${DATABASE_PASSWORD:changeme}
    driver-class-name: org.postgresql.Driver
  jpa:
    hibernate:
      ddl-auto: update
    show-sql: false
    properties:
      hibernate:
        dialect: org.hibernate.dialect.PostgreSQLDialect
        format_sql: true"#,
        DatabaseType::Mysql => r#"
  datasource:
    url: ${DATABASE_URL:jdbc:mysql://localhost:3306/appdb}
    username: ${DATABASE_USERNAME:root}
    password: ${DATABASE_PASSWORD:root}
    driver-class-name: com.mysql.cj.jdbc.Driver
  jpa:
    hibernate:
      ddl-auto: update
    show-sql: false
    properties:
      hibernate:
        dialect: org.hibernate.dialect.MySQLDialect
        format_sql: true"#,
        DatabaseType::H2 => r#"
  datasource:
    url: jdbc:h2:mem:testdb
    driver-class-name: org.h2.Driver
  h2:
    console:
      enabled: true
  jpa:
    hibernate:
      ddl-auto: create-drop
    defer-datasource-initialization: true"#,
        DatabaseType::Mongo => r#"
  data:
    mongodb:
      uri: ${MONGODB_URI:mongodb://localhost:27017/appdb}"#,
    };

    content = content.replace("{{DB_CONFIG}}", db_block);

    fs::write(&yml_path, content)?;
    println!("✅ DB override dans application.yml ({:?})", db);
    Ok(())
}

fn generate_pom_db_dependency(project_dir: &Path, db: &DatabaseType) -> anyhow::Result<()> {
    let pom_path = project_dir.join("pom.xml");
    if !pom_path.exists() {
        println!("⚠️  pom.xml non trouvé, injection DB sautée");
        return Ok(());
    }

    let mut pom = fs::read_to_string(&pom_path)?;

    let db_dep = match db {
        DatabaseType::Postgres => r#"
        <dependency>
            <groupId>org.postgresql</groupId>
            <artifactId>postgresql</artifactId>
            <scope>runtime</scope>
        </dependency>"#,
        DatabaseType::Mysql => r#"
        <dependency>
            <groupId>mysql</groupId>
            <artifactId>mysql-connector-java</artifactId>
            <scope>runtime</scope>
        </dependency>"#,
        DatabaseType::H2 => r#"
        <dependency>
            <groupId>com.h2database</groupId>
            <artifactId>h2</artifactId>
            <scope>runtime</scope>
        </dependency>"#,
        DatabaseType::Mongo => r#"
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-data-mongodb</artifactId>
        </dependency>"#,
    };

    pom = pom.replace("{{DB_DEPENDENCY}}", db_dep);
    fs::write(&pom_path, pom)?;
    println!("✅ pom.xml enrichi avec la dépendance {:?}", db);
    Ok(())
}

fn restructure_package(project_dir: &Path, package: &str) -> anyhow::Result<()> {
    let src_java = project_dir.join("src/main/java");
    if !src_java.exists() {
        return Ok(());
    }

    let new_pkg_path = package.replace('.', "/");
    let old_pkg_path = "{{PACKAGE_PATH}}";

    let old_path = src_java.join(old_pkg_path);
    let new_path = src_java.join(&new_pkg_path);

    if old_path.exists() {
        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&old_path, &new_path)?;
        println!("✅ Package déplacé: {} → {}", old_pkg_path, new_pkg_path);
    } else {
        println!("⚠️  Dossier {} non trouvé, aucun déplacement", old_pkg_path);
    }

    Ok(())
}

