use db::XDNSRepository;
use shared::time::system_time_from_epoch_seconds;
use xdns_data::models::Domain;

pub async fn add_domain(db: &db::Repository) {
    let domain = Domain {
        name: "example.o".to_string(),
        valid_from: system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64),
    };

    let result = db
        .add_domain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0",
            domain,
        )
        .await;

    assert!(result);
}

#[tokio::test]
async fn add_and_get() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let result = db.get_domain("example.o").await;
    assert!(result.is_ok());
    let result = result.unwrap().1;
    assert_eq!(result.name, "example.o");
}

#[tokio::test]
async fn test_add_after_existing() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_domain(&db).await;

    let domain = Domain {
        name: "example.o".to_string(),
        valid_from: system_time_from_epoch_seconds((chrono::Utc::now().timestamp() + 5) as u64),
    };

    let result = db
        .add_domain(
            "bc1pxwn9duraglsgr9f7q8ua33sx0vkq5wjft575h662995zf5m27v2qqxlf3k",
            "9ee554b35ad5f94bb28cda94951f5c8500bc457b299b3b4a4fd9701f3147017ci0",
            domain,
        )
        .await;

    assert!(!result);
}

#[tokio::test]
async fn get_by_inscription_id() {
    let inscription_id = "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";
    let valid_from = system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64);
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    let domain = Domain {
        name: "example.o".to_string(),
        valid_from,
    };

    let result = db
        .add_domain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id,
            domain,
        )
        .await;
    assert!(result);

    let result = db.get_domain_by_inscription(inscription_id).await;

    assert!(result.is_ok());
    let result = result.unwrap().1;
    assert_eq!(result.name, "example.o");
    assert_eq!(
        result
            .valid_from
            .duration_since(valid_from)
            .unwrap()
            .as_secs(),
        0
    );
}

#[tokio::test]
async fn get_non_existent() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    let result = db.get_domain("example.o").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn remove() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    let domain = Domain {
        name: "example.o".to_string(),
        valid_from: system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64),
    };

    let result = db
        .add_domain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0",
            domain,
        )
        .await;

    assert!(result);

    let result = db.remove_domain("example.o").await;

    assert!(result);
}

#[tokio::test]
async fn remove_by_inscription() {
    let inscription_id = "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    let domain = Domain {
        name: "example.o".to_string(),
        valid_from: system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64),
    };

    let result = db
        .add_domain(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            inscription_id,
            domain,
        )
        .await;
    assert!(result);

    let result = db.remove_domain_by_inscription(inscription_id).await;
    assert!(result);
}

#[tokio::test]
async fn remove_non_existent() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    let result = db.remove_domain("example.o").await;

    assert!(!result);
}
