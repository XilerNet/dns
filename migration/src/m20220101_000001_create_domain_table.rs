use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Domain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Domain::Inscription)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Domain::Address).string().not_null())
                    .col(
                        ColumnDef::new(Domain::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Domain::ValidFrom).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Domain::Table)
                    .name("idx_domain_name")
                    .col(Domain::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Domain::Table)
                    .name("idx_domain_address")
                    .col(Domain::Address)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Domain::Table).if_exists().to_owned())
            .await
    }
}

#[derive(Iden)]
enum Domain {
    Table,
    #[iden(rename = "inscription")]
    Inscription,
    #[iden(rename = "address")]
    Address,
    #[iden(rename = "name")]
    Name,
    #[iden(rename = "valid_from")]
    ValidFrom,
}
