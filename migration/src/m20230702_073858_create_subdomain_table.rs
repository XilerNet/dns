use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subdomain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subdomain::Inscription)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subdomain::Domain).string().not_null())
                    .col(ColumnDef::new(Subdomain::Subdomain).date().not_null())
                    .col(ColumnDef::new(Subdomain::Rtype).string().not_null())
                    .col(ColumnDef::new(Subdomain::Class).string().not_null())
                    .col(ColumnDef::new(Subdomain::Ttl).unsigned().not_null())
                    .col(ColumnDef::new(Subdomain::Rdata).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Subdomain::Table)
                    .name("idx_subdomain_domain")
                    .col(Subdomain::Domain)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Subdomain::Table)
                    .name("idx_subdomain_subdomain")
                    .col(Subdomain::Subdomain)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subdomain::Table).if_exists().to_owned())
            .await
    }
}

#[derive(Iden)]
enum Subdomain {
    Table,
    #[iden(rename = "inscription")]
    Inscription,
    #[iden(rename = "domain")]
    Domain,
    #[iden(rename = "subdomain")]
    Subdomain,
    #[iden(rename = "rtype")]
    Rtype,
    #[iden(rename = "class")]
    Class,
    #[iden(rename = "ttl")]
    Ttl,
    #[iden(rename = "rdata")]
    Rdata,
}
