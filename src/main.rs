#[macro_use] extern crate rocket;

extern crate tera;

use rocket::fs::FileServer;
use rocket::form::*;
use rocket_dyn_templates::{Template};
use rocket::serde::json;
use serde::Deserialize;


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

use rocket::response::stream::EventStream;
use rocket::response::stream::Event;

#[get("/stream")]
fn messages_loop() -> EventStream![] {
    EventStream!{
        let  mut interval = rocket::tokio::time::interval(rocket::tokio::time::Duration::from_secs(2));
        loop {
            let message = Message {
                name: "Lily",
                text: "Oh hey I just logged on",
                from_current_user: false
            };
            let event = Event::json(&message);
            yield event;
            interval.tick().await;
        }
    }
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct MessageData<'v> {
    user: &'v str,
    text: &'v str
}

#[post("/send_message", format="json", data = "<message_data>")]
async fn get_message<'l>(message_data:json::Json<MessageData<'l>>) {
    println!("got a new message to process");
    println!("{}: said {}",message_data.user, message_data.text);
}


#[post("/login", data = "<user_form>")]
fn login<'r>(user_form: Form<UserLogin<'r>> ) -> rocket::response::Redirect {
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

    let fs = FileServer::from("static/");

    //Template::render("login", messages);
    rocket::response::Redirect::to("/login.html")
    
}


#[launch]
fn rocket() -> _ {
    let mes = Message {
        name: "one",
        text: "two",
        from_current_user: true
    };
    use json::json;
    let thisis = json!(mes);
    println!("{}", thisis);
    let paths = std::fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    rocket::build()
        //.mount("/", routes![index,delay])
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![login])
        .mount("/", routes![messages_loop])
        .mount("/", routes![get_message])
        .attach(Template::fairing())
        

} 

use rocket::tokio::time::{sleep, Duration};

