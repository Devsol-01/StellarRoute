//! Database connection management

use sqlx::PgPool;
use tracing::{error, info};

use crate::config::IndexerConfig as Config;
use crate::error::{IndexerError, Result};

/// Database connection pool
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(config: &Config) -> Result<Self> {
        info!("Connecting to database: {}", config.database_url);

        let pool = PgPool::connect(&config.database_url).await.map_err(|e| {
            error!("Failed to connect to database: {}", e);
            IndexerError::Database(e)
        })?;

        info!("Database connection established");
        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations");

        // Read migration files from migrations directory
        let migrations = include_str!("../../migrations/0001_init.sql");

        // Execute migration SQL
        sqlx::query(migrations)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Migration failed: {}", e);
                IndexerError::Database(e)
            })?;

        info!("Database migrations completed");
        Ok(())
    }

    /// Check database health
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(IndexerError::Database)?;
        Ok(())
    }
}
