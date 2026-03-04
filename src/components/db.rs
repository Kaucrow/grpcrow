use sqlx::{
    postgres::{PgArguments, PgPoolOptions, PgRow},
    FromRow, PgPool, Postgres, Error, query_with, query_as_with
};

#[derive(Debug, Clone)]
pub struct DbComponent {
    pool: PgPool,
}

impl DbComponent {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn fetch_one<'a, T>(
        &self,
        sql: &'a str,
        args: PgArguments,
    ) -> Result<Option<T>, Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        query_as_with::<Postgres, T, PgArguments>(sql, args)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn fetch_all<'a, T>(
        &self,
        sql: &'a str,
        args: PgArguments,
    ) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        query_as_with::<Postgres, T, PgArguments>(sql, args)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn execute<'a>(
        &self,
        sql: &'a str,
        args: PgArguments,
    ) -> Result<u64, Error> {
        let result = query_with::<Postgres, PgArguments>(sql, args)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}

#[macro_export]
macro_rules! __db_args {
    ($($arg:expr),* $(,)?) => {{
        let mut args = sqlx::postgres::PgArguments::default();
        use sqlx::Arguments; 
        $(
            args.add($arg).unwrap(); 
        )*
        args
    }};
}

pub use crate::__db_args as args;