use super::handler;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tasks")
            .route(web::get().to(handler::get_tasks))
            .route(web::post().to(handler::create_task)),
    )
    .service(
        web::resource("/tasks/{id}")
            .route(web::get().to(handler::get_task_by_id))
            .route(web::delete().to(handler::delete_task))
            .route(web::put().to(handler::update_task)),
    );
}
