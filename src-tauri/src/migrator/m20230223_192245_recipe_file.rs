use sea_orm_migration::prelude::*;

use crate::migrator::index_name;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeFile::Table)
                    .col(
                        ColumnDef::new(RecipeFile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RecipeFile::Name).string().not_null())
                    .col(ColumnDef::new(RecipeFile::Order).integer().not_null())
                    .col(ColumnDef::new(RecipeFile::Mime).string().not_null())
                    .col(ColumnDef::new(RecipeFile::Path).string().not_null())
                    .col(
                        ColumnDef::new(RecipeFile::RecipeStepId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeFile::Table, RecipeFile::RecipeStepId)
                            .to(RecipeStep::Table, RecipeStep::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .col(RecipeFile::Order)
                            .col(RecipeFile::RecipeStepId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(&RecipeFile::Table, &RecipeFile::Order))
                    .table(RecipeFile::Table)
                    .col(RecipeFile::Order)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(&index_name(&RecipeFile::Table, &RecipeFile::RecipeStepId))
                    .table(RecipeFile::Table)
                    .col(RecipeFile::RecipeStepId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RecipeStep::Table)
                    .drop_column(RecipeStep::Image)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum RecipeFile {
    Table,
    Id,
    Name,
    Order,
    Mime,
    Path,
    RecipeStepId,
}

#[derive(Iden)]
enum RecipeStep {
    Table,
    Id,
    Image,
}
