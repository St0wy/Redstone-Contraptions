use crate::db::Pool;
use crate::models::root::{create_schema, Context, Schema};
use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql;
use juniper::http::GraphQLRequest;

pub async fn graphiql() -> HttpResponse {
    let html = graphiql::graphiql_source("/api", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn api(
    schema: web::Data<Schema>,
    pool: web::Data<Pool>,
    req: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        dbpool: pool.get_ref().to_owned(),
    };
    let json = web::block(move || {
        let res = req.execute_sync(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .service(web::resource("/api").route(web::post().to(api)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)));
}
