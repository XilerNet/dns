use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // pub inscription: String,
        // pub domain: String,
        // pub algorithm: String,
        // pub public_key: String,

        manager
            .create_table(
                Table::create()
                    .table(Validity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Validity::Inscription)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Validity::Domain).string().not_null())
                    .col(ColumnDef::new(Validity::Algorithm).string().not_null())
                    .col(ColumnDef::new(Validity::PublicKey).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Validity::Table)
                    .name("idx_validity_domain")
                    .col(Validity::Domain)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Validity::Table).if_exists().to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Validity {
    Table,
    #[iden(rename = "inscription")]
    Inscription,
    #[iden(rename = "domain")]
    Domain,
    #[iden(rename = "algorithm")]
    Algorithm,
    #[iden(rename = "public_key")]
    PublicKey,
}
