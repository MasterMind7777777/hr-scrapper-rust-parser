use crate::models::{NewElement, NewElementSelector, NewPage, NewSelector};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn insert_page(conn: &mut PgConnection, new_page: NewPage) -> QueryResult<Uuid> {
    use crate::schema::pages::dsl::*;
    diesel::insert_into(pages)
        .values(&new_page)
        .returning(id)
        .get_result(conn)
}

pub fn insert_element(conn: &mut PgConnection, new_element: NewElement) -> QueryResult<Uuid> {
    use crate::schema::elements::dsl::*;
    diesel::insert_into(elements)
        .values(&new_element)
        .returning(id)
        .get_result(conn)
}

pub fn insert_selector(conn: &mut PgConnection, new_selector: NewSelector) -> QueryResult<Uuid> {
    use crate::schema::selectors::dsl::*;
    diesel::insert_into(selectors)
        .values(&new_selector)
        .returning(id)
        .get_result(conn)
}

pub fn insert_element_selector(
    conn: &mut PgConnection,
    new_element_selector: NewElementSelector,
) -> QueryResult<Uuid> {
    use crate::schema::element_selector::dsl::*;
    diesel::insert_into(element_selector)
        .values(&new_element_selector)
        .returning(id)
        .get_result(conn)
}

pub fn check_page_html_hash_exist(conn: &PgConnection, html_hash: &str) -> bool {
    use crate::schema::pages::dsl::*;

    let result = pages.filter(html_hash.eq(html_hash)).first::<NewPage>(conn);
    match result {
        Ok(_) => true,  // Hash exists
        Err(_) => false, // Hash does not exist
    }
}
