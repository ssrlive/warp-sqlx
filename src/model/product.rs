use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlb::HasFields;

#[derive(sqlb::Fields, sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i64>,
    pub name: String,
}

impl Product {
    const TABLE: &'static str = "product";
    const ID: &'static str = "id";
    const NAME: &'static str = "name";
    const FIELDS: &'static [&'static str] = &[Self::ID, Self::NAME];

    pub fn fields_without_id(&self) -> Vec<sqlb::Field> {
        let mut fields = self.fields();
        if let Some(idx) = fields.iter().position(|r| r.0 == Self::ID) {
            fields.remove(idx);
        }
        fields
    }
}

pub struct ProductController;

impl ProductController {
    pub async fn get_all(db: &Db) -> Result<Vec<Product>, warp::Rejection> {
        let sb = sqlb::select().table(Product::TABLE).columns(Product::FIELDS);
        let products: Vec<Product> = sb.fetch_all(db).await.unwrap();
        Ok(products)
    }

    pub async fn get_by_id(db: &Db, id: i64) -> Result<Product, warp::Rejection> {
        let sb = sqlb::select()
            .table(Product::TABLE)
            .columns(Product::FIELDS)
            .and_where(Product::ID, "=", id);
        let product: Product = sb.fetch_one(db).await.unwrap();
        Ok(product)
    }

    pub async fn create(db: &Db, product: &Product) -> Result<Product, warp::Rejection> {
        let fields = product.fields_without_id();
        let sb = sqlb::insert()
            .table(Product::TABLE)
            .data(fields)
            .returning(Product::FIELDS);
        let product: Product = sb.fetch_one(db).await.unwrap();
        Ok(product)
    }

    pub async fn update(db: &Db, id: i64, product: &Product) -> Result<Product, warp::Rejection> {
        let fields = product.fields_without_id();
        let sb = sqlb::update()
            .table(Product::TABLE)
            .data(fields)
            .and_where(Product::ID, "=", id)
            .returning(Product::FIELDS);
        let product: Product = sb.fetch_one(db).await.unwrap();
        Ok(product)
    }

    pub async fn delete(db: &Db, id: i64) -> Result<Product, warp::Rejection> {
        let sb = sqlb::delete()
            .table(Product::TABLE)
            .and_where_eq(Product::ID, id)
            .returning(Product::FIELDS);
        let product: Product = sb.fetch_one(db).await.unwrap();
        Ok(product)
    }
}
