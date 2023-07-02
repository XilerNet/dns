use db::XDNSRepository;
use shared::time::system_time_from_epoch_seconds;
use xdns_data::models::Domain;

#[tokio::test]
async fn add_and_get() {
    let mut db = db::Repository::new_memory().await;
    db.migrate().await;

    let domain = Domain {
        name: "example.o".to_string(),
        valid_from: system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64),
    };

    let result = db
        .add_domain(
            "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0",
            &domain,
        )
        .await;
    assert!(result);

    let result = db.get_domain("example.o").await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.name, "example.o");
}

#[tokio::test]
async fn test_get_by_inscription_id() {
    let inscription_id = "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";
    let valid_from = system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64);
    let mut db = db::Repository::new_memory().await;
    db.migrate().await;

    let domain = Domain { name: "example.o".to_string(), valid_from };

    let result = db.add_domain(inscription_id, &domain).await;
    assert!(result);

    let result = db.get_domain_by_inscription(inscription_id).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.name, "example.o");
    assert_eq!(result.valid_from.duration_since(valid_from).unwrap().as_secs(), 0);
}

#[tokio::test]
async fn get_non_existent() {
    let mut db = db::Repository::new_memory().await;
    db.migrate().await;

    let result = db.get_domain("example.o").await;
    println!("{:?}", result);
    assert!(result.is_err());
}

#[tokio::test]
async fn remove() {
    let mut db = db::Repository::new_memory().await;
    db.migrate().await;

    let domain = Domain {
        name: "example.o".to_string(),
        valid_from: system_time_from_epoch_seconds(chrono::Utc::now().timestamp() as u64),
    };

    let result = db
        .add_domain(
            "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0",
            &domain,
        )
        .await;

    assert!(result);

    let result = db.remove_domain("example.o").await;


    assert!(result);
}


#[tokio::test]
async fn remove_non_existent() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    let result = db.remove_domain("example.o").await;

    assert!(!result);
}