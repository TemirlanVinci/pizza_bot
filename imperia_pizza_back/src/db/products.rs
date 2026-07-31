use crate::models::products::{Product, ProductFilter, ProductFull};
use sqlx::{PgPool, Postgres, QueryBuilder};

pub async fn get_products(
    pool: &PgPool,
    filter: &ProductFilter,
) -> Result<Vec<Product>, sqlx::Error> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id, name, price, category_id FROM products WHERE 1=1");

    if let Some(category_id) = filter.category_id {
        query_builder.push(" AND category_id = ");
        query_builder.push_bind(category_id);
    }

    if let Some(limit) = filter.limit {
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
    }

    if let Some(offset) = filter.offset {
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);
    }

    query_builder
        .build_query_as::<Product>()
        .fetch_all(pool)
        .await
}

pub async fn get_product_by_id(
    pool: &PgPool,
    product_id: i32,
) -> Result<Option<ProductFull>, sqlx::Error> {
    sqlx::query_as::<_, ProductFull>(
        r#"
        SELECT 
            id, 
            category_id, 
            name, 
            description, 
            price, 
            weight, 
            image_url 
        FROM products 
        WHERE id = $1
        "#,
    )
    .bind(product_id)
    .fetch_optional(pool)
    .await
}
