use super::handler;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(handler::get_users)))
        .service(
            web::resource("/users/{id}")
                .route(web::get().to(handler::get_user_by_id))
                .route(web::delete().to(handler::delete_user))
                .route(web::put().to(handler::update_user)),
        );
}
