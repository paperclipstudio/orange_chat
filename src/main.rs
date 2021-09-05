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
    text: &'static str,
    from_current_user: bool
}

#[derive(Serialize, Clone)]
struct Messages {
    messages: Vec<Message>,
    rooms: Vec<String>
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
    let current_user = "paper";
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
        text: "Im good, how are you?",
        from_current_user: false,
    };
    let my_message = Message {
        name: "Paper",
        text: "Not too bad, wuu2?",
        from_current_user: true,
    };
    let mut some_convo = vec![test.clone()];
    for _ in 1..5 {
        some_convo.push(test.clone());
        some_convo.push(my_message.clone());
    }
    let messages = Messages {
        messages: some_convo,
        rooms: vec![String::from("BA"), String::from("Gaming")]
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

