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

pub fn update_page(
    conn: &mut PgConnection,
    page_name: &str,
    new_url: String,
    new_html: String,
    new_html_hash: String,
) -> QueryResult<usize> {
    use crate::schema::pages::dsl::*;
    diesel::update(pages.filter(name.eq(page_name)))
        .set((
            url.eq(new_url),
            html.eq(new_html),
            html_hash.eq(new_html_hash),
        ))
        .execute(conn)
}

pub fn try_insert_page(
    conn: &mut diesel::PgConnection,
    name: &str,
    url: &str,
    html: &str,
    html_hash: &str,
) -> Result<uuid::Uuid, diesel::result::Error> {
    insert_page(
        conn,
        NewPage {
            name: name.to_string(),
            url: url.to_string(),
            html: html.to_string(),
            html_hash: html_hash.to_string(),
        },
    )
}

pub fn get_page_id_by_name(conn: &mut diesel::PgConnection, page_name: &str) -> Result<uuid::Uuid, diesel::result::Error> {
    use crate::schema::pages::dsl::*;
    pages.filter(name.eq(page_name)).select(id).first(conn)
}

pub fn is_hash_matching(conn: &mut diesel::PgConnection, page_id: uuid::Uuid, hash: &str) -> bool {
    use crate::schema::pages::dsl::*;
    pages.filter(id.eq(page_id)).select(html_hash).first::<String>(conn).map_or(false, |db_hash| db_hash == hash)
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

