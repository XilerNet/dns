use db::XDNSRepository;
use xdns_data::models::{Algorithm, Credentials, Validity, ValidityTransfer};

const INSCRIPTION_ID: &'static str =
    "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";

async fn db_setup_boilerplate() -> db::Repository {
    let db = db::Repository::new_memory().await;
    db.migrate().await;
    add_validity_helper(INSCRIPTION_ID, &db).await;

    db
}

async fn add_validity_helper(id: &str, db: &db::Repository) {
    let validity = Validity {
        domain: "example.o".to_string(),
        credentials: Credentials::new(Algorithm::Ed25519, "xiler".to_string()),
    };

    let result = db
        .add_validity(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            id,
            validity,
        )
        .await;
    assert!(result);
}

#[tokio::test]
async fn add_validity() {
    let db = db::Repository::new_memory().await;
    db.migrate().await;

    for i in 0..2 {
        let validity = Validity {
            domain: "example.o".to_string(),
            credentials: Credentials::new(Algorithm::Ed25519, "xiler".to_string()),
        };

        let result = db
            .add_validity(
                "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
                &(i.to_string() + INSCRIPTION_ID),
                validity,
            )
            .await;
        assert_eq!(result, i == 0);
    }
}

#[tokio::test]
async fn get_validity() {
    let db = db_setup_boilerplate().await;

    let result = db.get_validity("example.o").await;
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.1.domain, "example.o");
    assert_eq!(result.1.credentials.algorithm, Algorithm::Ed25519);
    assert_eq!(result.1.credentials.public_key, "xiler");
}

#[tokio::test]
async fn get_validity_by_inscription() {
    let db = db_setup_boilerplate().await;

    let result = db.get_validity_by_inscription(INSCRIPTION_ID).await;
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.1.domain, "example.o");
    assert_eq!(result.1.credentials.algorithm, Algorithm::Ed25519);
    assert_eq!(result.1.credentials.public_key, "xiler");
}

#[tokio::test]
async fn remove_validity() {
    let db = db_setup_boilerplate().await;

    for i in 0..2 {
        let result = db.remove_validity("example.o").await;
        assert_eq!(result, i == 0);
    }
}

#[tokio::test]
async fn remove_validity_by_inscription() {
    let db = db_setup_boilerplate().await;

    for i in 0..2 {
        let result = db.remove_validity_by_inscription(INSCRIPTION_ID).await;
        assert_eq!(result, i == 0);
    }
}

#[tokio::test]
async fn update_validity() {
    let db = db_setup_boilerplate().await;

    let transfer = ValidityTransfer {
        domain: "example.o".to_string(),
        new_credentials: Some(Credentials::new(Algorithm::Ed25519, "hello".to_string())),
    };

    let result = db.update_validity(transfer).await;
    assert!(result);

    let result = db.get_validity("example.o").await;
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.1.domain, "example.o");
    assert_eq!(result.1.credentials.algorithm, Algorithm::Ed25519);
    assert_eq!(result.1.credentials.public_key, "hello");
}

#[tokio::test]
async fn update_validity_delete() {
    let db = db_setup_boilerplate().await;

    let transfer = ValidityTransfer {
        domain: "example.o".to_string(),
        new_credentials: None,
    };

    let result = db.update_validity(transfer).await;
    assert!(result);

    let result = db.get_validity("example.o").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn update_validity_by_inscription() {
    let db = db_setup_boilerplate().await;

    let transfer = ValidityTransfer {
        domain: "xiler.o".to_string(),
        new_credentials: Some(Credentials::new(Algorithm::Ed25519, "hello".to_string())),
    };

    let result = db
        .update_validity_by_inscription(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            INSCRIPTION_ID,
            transfer,
        )
        .await;
    assert!(result);

    let result = db.get_validity_by_inscription(INSCRIPTION_ID).await;
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.1.domain, "xiler.o");
    assert_eq!(result.1.credentials.algorithm, Algorithm::Ed25519);
    assert_eq!(result.1.credentials.public_key, "hello");
}

#[tokio::test]
async fn update_validity_by_inscription_delete() {
    let db = db_setup_boilerplate().await;

    let transfer = ValidityTransfer {
        domain: "xiler.o".to_string(),
        new_credentials: None,
    };

    let result = db
        .update_validity_by_inscription(
            "tb1pm3q4drt7suvdsfndz5uyge652xswl09nvshr7k00964xtchmnemqyuuvd5",
            INSCRIPTION_ID,
            transfer,
        )
        .await;
    assert!(result);

    let result = db.get_validity_by_inscription(INSCRIPTION_ID).await;
    assert!(result.is_err());
}
