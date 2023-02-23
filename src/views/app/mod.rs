pub mod items;
use actix_web::web;

use super::path::Path;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/"),
    };

    app.route(
        &base_path.define(String::new()),
        web::get().to(items::items),
    );
}
