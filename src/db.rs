use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::fs;

const PG_HOST: &str = "localhost";
const PG_DB: &str = "postgres";
const PG_USER: &str = "postgres";
const PG_PWD: &str = "postgres";
const PG_MAX_CONN: u32 = 5;

const SQL_FILE: &str = "sql/create_tables.sql";

pub type Db = Pool<Postgres>;

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_conn: u32,
) -> anyhow::Result<Db> {
    let db_url = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_conn)
        // .connect_timeout(std::time::Duration::from_millis(500))
        .connect(&db_url)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

async fn sql_exec(db: &Db, file: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(file).await.map_err(|e| anyhow::anyhow!(e))?;
    let sqls: Vec<&str> = content.split(';').collect();
    for sql in sqls {
        if sql.trim().is_empty() {
            continue;
        }
        match sqlx::query(sql).execute(db).await {
            Ok(_) => {}
            Err(e) => {
                log::debug!("sql_exec: {}", e);
            }
        }
    }
    Ok(())
}

pub async fn init_db() -> anyhow::Result<Db> {
    {
        let db = new_db_pool(PG_HOST, "postgres", PG_USER, PG_PWD, 1).await?;
        sql_exec(&db, SQL_FILE).await?;
    }
    let db = new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, PG_MAX_CONN).await?;
    Ok(db)
}
