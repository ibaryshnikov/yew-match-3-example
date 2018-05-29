extern crate yew;
extern crate m3;

#[macro_use]
extern crate log;
extern crate web_logger;

use yew::prelude::*;
use m3::Model;

pub type Context = ();

fn main() {
    web_logger::init();
    info!("before initialization");
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
