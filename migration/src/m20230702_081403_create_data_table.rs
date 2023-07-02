use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // pub inscription: String,
        // pub domain: String,
        // pub data: Vec<u8>,

        manager
            .create_table(
                Table::create()
                    .table(Data::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Data::Inscription)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Data::Domain).string().not_null())
                    .col(ColumnDef::new(Data::Data).binary().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Data::Table)
                    .name("idx_data_domain")
                    .col(Data::Domain)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Data::Table).if_exists().to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Data {
    Table,
    #[iden(rename = "inscription")]
    Inscription,
    #[iden(rename = "domain")]
    Domain,
    #[iden(rename = "data")]
    Data,
}
