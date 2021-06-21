// fn main() {
//     println!("Hello, world!");
// }



#[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }



// see <https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/hello/src/main.rs>



// in this section, get a route to '/info', and a route to '/version' to work



// below works

#[get("/")]
async fn index() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
