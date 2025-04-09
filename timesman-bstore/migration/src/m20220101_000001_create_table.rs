// use sea_orm::ColumnDef;
use sea_orm_migration::{prelude::*, schema::*};
use table::ColumnDef;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Times table
        manager
            .create_table(
                Table::create()
                    .table(Times::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Times::Id)
                            .integer()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Times::Title).string().not_null())
                    .col(
                        ColumnDef::new(Times::CreatedAt).date_time().not_null(),
                    )
                    .col(ColumnDef::new(Times::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await?;
        // Post table
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Tid).integer().not_null())
                    .col(ColumnDef::new(Post::Text).string())
                    .col(ColumnDef::new(Post::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Post::UpdatedAt).date_time())
                    .col(ColumnDef::new(Post::FileId).integer())
                    .to_owned(),
            )
            .await?;
        // File table
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(File::Tid).integer().not_null())
                    .col(ColumnDef::new(File::Name).string().not_null())
                    .col(ColumnDef::new(File::Path).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    Tid,
    Name,
    Path,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Tid,
    Text,
    CreatedAt,
    UpdatedAt,
    FileId,
}

#[derive(DeriveIden)]
enum Times {
    Table,
    Id,
    Title,
    CreatedAt,
    UpdatedAt,
}
