use crate::{
    db,
    model::{Product, ProductController},
};
use serde::Serialize;
use std::sync::Arc;
use warp::{reply::Json, Filter};

#[allow(opaque_hidden_inferred_bound)]
pub fn product_rest_filter(
    prefix: &'static str,
    db: Arc<db::Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let product_path = warp::path(prefix).and(warp::path("products"));
    let db = db::with_db(db);

    // list all products
    // GET /api/products
    let list = product_path
        .and(warp::get())
        .and(warp::path::end())
        .and(db.clone())
        .and_then(product_list);

    // get a product by id
    // GET /api/products/{id}
    let get = product_path
        .and(warp::get())
        .and(warp::path::param())
        .and(db.clone())
        .and_then(product_get);

    // create a new product
    // POST /api/products
    let create = product_path
        .and(warp::post())
        .and(warp::body::json())
        .and(db.clone())
        .and_then(product_create);

    // update a product
    // PUT /api/products/{id}
    let update = product_path
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::body::json())
        .and(db.clone())
        .and_then(product_update);

    // delete a product
    // DELETE /api/products/{id}
    let delete = product_path
        .and(warp::delete())
        .and(warp::path::param())
        .and(db)
        .and_then(product_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn product_list(db: Arc<db::Db>) -> Result<Json, warp::Rejection> {
    let products = ProductController::get_all(&db).await.map_err(|e| cvt_err(e))?;
    json_response(&products).map_err(|e| cvt_err(e))
}

async fn product_get(id: i64, db: Arc<db::Db>) -> Result<Json, warp::Rejection> {
    let product = ProductController::get_by_id(&db, id).await.map_err(|e| cvt_err(e))?;
    json_response(&product).map_err(|e| cvt_err(e))
}

async fn product_create(product: Product, db: Arc<db::Db>) -> Result<Json, warp::Rejection> {
    let product = ProductController::create(&db, &product).await.map_err(|e| cvt_err(e))?;
    json_response(&product).map_err(|e| cvt_err(e))
}

async fn product_update(id: i64, product: Product, db: Arc<db::Db>) -> Result<Json, warp::Rejection> {
    let product = ProductController::update(&db, id, &product)
        .await
        .map_err(|e| cvt_err(e))?;
    json_response(&product).map_err(|e| cvt_err(e))
}

async fn product_delete(id: i64, db: Arc<db::Db>) -> Result<Json, warp::Rejection> {
    let product = ProductController::delete(&db, id).await.map_err(|e| cvt_err(e))?;
    json_response(&product).map_err(|e| cvt_err(e))
}

fn json_response<T: Serialize>(data: &T) -> anyhow::Result<Json> {
    let json = serde_json::to_string(data).map_err(|e| anyhow::anyhow!(e))?;
    Ok(warp::reply::json(&json))
}

#[derive(Debug)]
struct DbError(anyhow::Error);
impl warp::reject::Reject for DbError {}

fn cvt_err(err: anyhow::Error) -> warp::reject::Rejection {
    warp::reject::custom(DbError(err))
}
