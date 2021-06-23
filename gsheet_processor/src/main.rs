#[macro_use] extern crate rocket;


// see <https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/hello/src/main.rs>



// in this section, get a route to '/info', and a route to '/version' to work

#[get("/")]
async fn index() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "coming: root-response"
}

#[get("/version")]
async fn version() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "coming: version-response"
}

#[get("/info")]
async fn info() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "coming: info-response"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![version])
        .mount("/", routes![info])
}




// works -- start

// #[get("/")]
// async fn index() -> &'static str {
//     use rocket::tokio::time::{sleep, Duration};
//     sleep(Duration::from_secs(2)).await;
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
// }

// works -- end
