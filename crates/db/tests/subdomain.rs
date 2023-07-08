use db::XDNSRepository;
use xdns_data::models::subdomain::{Class, Type};
use xdns_data::models::{ SubDomain};

#[path = "./domain.rs"]
mod domain;

use domain::add_domain;

#[tokio::test]
async fn add_subdomain() {
    let inscription_id_2 = "26482871f33f1051f450f2da9af275794c0b5f1c61ebf35e4467fb42c2813403i0";
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let result = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_2,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example.o".to_string(),
            },
        )
        .await;

    assert!(result);
}

#[tokio::test]
async fn get_subdomain() {
    let inscription_id_2 = "26482871f33f1051f450f2da9af275794c0b5f1c61ebf35e4467fb42c2813403i0";
    let inscription_id_3 = "c17dd02a7f216f4b438ab1a303f518abfc4d4d01dcff8f023cf87c4403cb54cai0";
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let result = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_2,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example.o".to_string(),
            },
        )
        .await;

    let result2 = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_3,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example2.o".to_string(),
            },
        )
        .await;

    assert!(result);
    assert!(result2);

    let subdomains = db.get_subdomain("example.o", "test").await;

    assert!(subdomains.is_ok());
    let subdomains = subdomains.unwrap();

    assert_eq!(subdomains.len(), 2);

    let data = vec!["example.o", "example2.o"];

    'data: for d in data.iter() {
        for (_, subdomain) in subdomains.iter() {
            if subdomain.rdata == *d {
                assert!(true);
                continue 'data;
            }
        }
        assert!(false, "Subdomain {} not found within queried domain", d);
    }
}

#[tokio::test]
async fn get_subdomain_by_inscription() {
    let inscription_id_2 = "26482871f33f1051f450f2da9af275794c0b5f1c61ebf35e4467fb42c2813403i0";
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let result = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_2,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example.o".to_string(),
            },
        )
        .await;

    assert!(result);

    let subdomain = db.get_subdomain_by_inscription(inscription_id_2).await;
    assert!(subdomain.is_ok());

    let subdomain = subdomain.unwrap().1;
    assert_eq!(subdomain.rdata, "example.o");
}

#[tokio::test]
async fn remove_subdomains() {
    let inscription_id_2 = "26482871f33f1051f450f2da9af275794c0b5f1c61ebf35e4467fb42c2813403i0";
    let inscription_id_3 = "c17dd02a7f216f4b438ab1a303f518abfc4d4d01dcff8f023cf87c4403cb54cai0";
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let result = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_2,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example.o".to_string(),
            },
        )
        .await;

    let result2 = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_3,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example2.o".to_string(),
            },
        )
        .await;

    assert!(result);
    assert!(result2);

    let result = db.remove_subdomains("example.o", "test").await;
    assert!(result);
}

#[tokio::test]
async fn remove_subdomain() {
    let inscription_id_2 = "26482871f33f1051f450f2da9af275794c0b5f1c61ebf35e4467fb42c2813403i0";
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let result = db
        .add_subdomain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id_2,
            SubDomain {
                domain: "example.o".to_string(),
                subdomain: "test".to_string(),
                rtype: Type::CNAME,
                class: Class::IN,
                ttl: 0,
                rdata: "example.o".to_string(),
            },
        )
        .await;

    assert!(result);

    let result = db.remove_subdomain(inscription_id_2).await;
    assert!(result);
}
