use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Users::OpenId).string().not_null())
                    .col(ColumnDef::new(Users::SessionKey).string())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().extra("DEFAULT now()"))
                    .col(ColumnDef::new(Users::UpdatedAt).date_time().extra("DEFAULT now()"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    OpenId,
    SessionKey,
    CreatedAt,
    UpdatedAt,
}
