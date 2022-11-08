#[macro_use] extern crate rocket;
extern crate tokio_postgres;
extern crate tokio;

use tokio_postgres::{NoTls};

async fn test_db() -> Result<(), tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=hello1234$#@! dbname=portfolio_test", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let ret =  client.query("SELECT id, name FROM tb_user;", &[]).await?;

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