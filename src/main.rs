pub mod models;
pub mod schema;
mod services;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![services::items::list_items])
        .mount("/", routes![services::items::get_item])
        .mount("/", routes![services::items::delete_item])
        .mount("/", routes![services::items::create_item])
        .mount("/", routes![services::spells::list_spells])
        .mount("/", routes![services::spells::get_spell])
        .mount("/", routes![services::spells::delete_spell])
        .mount("/", routes![services::spells::create_spell])
        .mount("/", routes![services::worlds::create_world])
        .mount("/", routes![services::worlds::get_world])
        .mount("/", routes![services::worlds::delete_world])
        .mount("/", routes![services::worlds::list_worlds])
        .attach(Template::fairing())
}
