use db::XDNSRepository;
use xdns_data::models::Data;

const INSCRIPTION_ID: &'static str =
    "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";

async fn db_setup_data_boilerplate() -> db::Repository {
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    for i in 0..10 {
        let data = Data {
            domain: "example.o".to_string(),
            data: b"custom data".to_vec(),
        };

        let result = db.add_data(&(i.to_string() + INSCRIPTION_ID), data).await;
        assert!(result);
    }

    db
}

#[tokio::test]
async fn add_data() {
    db_setup_data_boilerplate().await;
}

#[tokio::test]
async fn get_data() {
    let db = db_setup_data_boilerplate().await;

    let result = db.get_data("example.o").await;
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.len(), 10);

    for i in 0..10 {
        assert_eq!(result[i].domain, "example.o");
        assert_eq!(result[i].data, b"custom data");
    }
}

#[tokio::test]
async fn get_data_by_inscription_id() {
    let db = db_setup_data_boilerplate().await;

    for i in 0..10 {
        let result = db
            .get_data_by_inscription(&(i.to_string() + INSCRIPTION_ID))
            .await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.domain, "example.o");
        assert_eq!(result.data, b"custom data");
    }
}

#[tokio::test]
async fn remove_data() {
    let db = db_setup_data_boilerplate().await;

    let result = db.remove_data("example.o").await;
    assert!(result);

    let get_result = db.get_data("example.o").await;
    assert!(get_result.is_ok());

    let get_result = get_result.unwrap();
    assert_eq!(get_result.len(), 0);
}

#[tokio::test]
async fn remove_data_by_inscription_id() {
    let db = db_setup_data_boilerplate().await;

    for i in 0..10 {
        let result = db
            .remove_data_by_inscription(&(i.to_string() + INSCRIPTION_ID))
            .await;
        assert!(result);

        let get_result = db
            .get_data_by_inscription(&(i.to_string() + INSCRIPTION_ID))
            .await;
        assert!(get_result.is_err());
    }
}
