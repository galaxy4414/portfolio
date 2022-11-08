#[macro_use] extern crate rocket;
extern crate tokio_postgres;
extern crate tokio;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls};

async fn test_db() -> Result<(), tokio_postgres::Error> {
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=postgres password=hello1234$#@! dbname=portfolio_test",
        tokio_postgres::NoTls,
    ).unwrap();

    let pool = Pool::builder().build(pg_mgr).await.unwrap();

    let mut connection = pool.get().await.unwrap();
    let ret = connection.query("SELECT id, name FROM tb_user", &[]).await.unwrap();

    for row in ret {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
    
        println!("found user: {} {}", id, name);
    }

    return Ok(())
}

#[get("/")]
async fn index() -> &'static str {
    test_db().await;
    return "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}