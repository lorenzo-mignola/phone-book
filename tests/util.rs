use axum_test::TestServer;
use phone_book::entity::{contacts, country_code::CountryCode, number::Number, phone_numbers};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectOptions, Database, DatabaseConnection, EntityTrait,
};

async fn connect_in_memory() -> DatabaseConnection {
    let mut opts = ConnectOptions::new("sqlite::memory:");
    opts.max_connections(1);
    let db = Database::connect(opts)
        .await
        .expect("Error getting db connection");

    phone_book::db::setup_schema(&db)
        .await
        .expect("Error setup schema");

    db
}

async fn seed_db(db: &DatabaseConnection) {
    let contact = contacts::ActiveModel {
        id: Set(1),
        first_name: Set(String::from("test")),
        ..Default::default()
    };

    let saved_contact = contact.insert(db).await.unwrap();

    let number = phone_numbers::ActiveModel {
        country_code: Set(CountryCode::CH),
        number: Set(Number(String::from("1234"))),
        contact_id: Set(saved_contact.id),
        ..Default::default()
    };

    number.insert(db).await.unwrap();
}

pub async fn setup_test() -> TestServer {
    let db = connect_in_memory().await;

    seed_db(&db).await;

    let router = phone_book::routes::router(phone_book::state::AppState { db });
    TestServer::new(router)
}
