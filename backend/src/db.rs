use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::DbErr;

pub async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect("sqlite:./sqlite.db?mode=rwc").await;
    db
}
