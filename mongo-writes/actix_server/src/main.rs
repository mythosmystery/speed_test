use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::DateTime, results::InsertOneResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    description: String,
    random_number: f64,
    created_at: DateTime,
}

#[derive(Clone, Debug)]
struct MongoRepo {
    client: mongodb::Client,
    db: mongodb::Database,
    collection: mongodb::Collection<TestData>,
}

impl MongoRepo {
    async fn init() -> MongoRepo {
        let client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Failed to initialize mongodb client");
        let db = client.database("speed_test");
        let coll = db.collection::<TestData>("test_collection");
        coll.drop(None).await.expect("Failed to drop collection");
        db.create_collection("test_collection", None)
            .await
            .expect("Failed to create collection");
        let collection = db.collection("test_collection");
        MongoRepo {
            client,
            db,
            collection,
        }
    }
    async fn insert(&self, data: TestData) -> Result<InsertOneResult, mongodb::error::Error> {
        self.collection.insert_one(data, None).await
    }
}

#[get("/")]
async fn insert(repo: web::Data<MongoRepo>) -> impl Responder {
    let data = TestData {
        id: None,
        name: "Test Name".to_string(),
        description: "Test Description".to_string(),
        random_number: fastrand::f64(),
        created_at: DateTime::now(),
    };
    match repo.insert(data).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = MongoRepo::init().await;

    println!("Server listening on port 3000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .service(insert)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
