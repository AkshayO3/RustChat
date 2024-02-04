#[macro_use] extern crate rocket;

use std::net::Shutdown;
use rocket::{get, launch, routes};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::broadcast::{Sender, error::RecvError, channel};
use rocket::form::Form;
use rocket::fs::{FileServer, relative};
use rocket::tokio::select;
use rocket::futures::SinkExt;
use rocket::response::stream::{Event,EventStream};
use rocket::State;

#[derive(Debug,Clone,FromForm,Serialize,Deserialize)]

#[serde(crate="rocket::serde")]
struct Message{
    #[field(validate = len(..30))]
    pub room:String,
    #[field(validate = len(..20))]
    pub username:String,
    pub message:String
}

#[post("/message",data="<form>")]
fn post(form:Form<Message>,queue:&State<Sender<Message>>){
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue:&State<Sender<Message>>,mut end:Shutdown) -> EventStream![]{
    let mut rx = queue.subscribe();
    EventStream!{
        loop{
            let msg = select!{
                msg = rx.recv() => match msg{
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[launch]                       // Launch attribute generates a main function and starts the rocket server.
fn rocket() -> _{
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/",routes![post,events])     // base path,and a list of routes
        .mount("/",FileServer::from(relative!("static")))
}