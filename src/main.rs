#[macro_use] extern crate rocket;

extern crate tera;

use rocket::fs::FileServer;
use rocket::form::*;
use rocket_dyn_templates::{Template};


#[get("/")]
fn index() -> &'static str {
    "<h1>Hello, world!</h1>
        <p>
        This is my website that is made in rust
        </p>
        "
}
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Message {
    name: &'static str,
    text: &'static str
}
#[derive(Serialize, Clone)]
struct Messages {
    messages: Vec<Message>
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[derive(Debug, FromForm)]
struct Submit {
    account: String,
    submission: String
}

#[derive(FromForm, Debug, )]
struct UserLogin<'v> {
    username: &'v str,
    password: &'v str
}




#[post("/login", data = "<user_form>")]
fn login<'r>(user_form: Form<UserLogin<'r>> ) -> Template {
    let t = match tera::Tera::new("templates/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error with {}", e);
            std::process::exit(1);
        }
    };
    
    let mut m = tera::Context::new();
    m.insert("name", "Tom");
    m.insert("text", "A whole new world");

    let test = Message {
        name: "Tom",
        text: "You should go to sleep"
    };
    let messages = Messages {
        messages: vec![test;30]
    };

    Template::render("login", messages)
}


#[launch]
fn rocket() -> _ {
    let paths = std::fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    rocket::build()
        //.mount("/", routes![index,delay])
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![login])
        .attach(Template::fairing())
} 

use rocket::tokio::time::{sleep, Duration};

