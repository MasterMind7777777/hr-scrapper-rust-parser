mod db_operations;
mod models;
mod schema;
mod utils;

mod io_helper;
mod parser;

use db_operations::*;
use utils::compute_hash;
use io_helper::*;
use log::{error, info};
use models::{NewElement, NewElementSelector, NewSelector};
use parser::*;
use scraper::Html;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;

fn main() {
    initialize_logging();

    let configs = load_and_process_config("src/config.json").expect("Error loading configuration file");

    let pool = establish_connection();
    let mut conn = pool.get().expect("Failed to get a database connection");

    for config in configs {
        let html_hash = compute_hash(&config.full_html);
        let name = config.url.trim_end_matches('/').split('/').last().unwrap_or("unknown");

        match try_insert_page(&mut conn, name, &config.url, &config.full_html, &html_hash) {
            Ok(page_id) => {
                process_fragments(&mut conn, page_id, &config);
            }
            Err(e) => match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    info!("Page with name '{}' already exists.", name);
                    let existing_page_id = get_page_id_by_name(&mut conn, name).expect("Error fetching existing page id");
                    if !is_hash_matching(&mut conn, existing_page_id, &html_hash) {
                        info!("Hash does not match. Updating page.");
                        update_page(&mut conn, name, config.url.clone(), config.full_html.clone(), html_hash.clone())
                            .expect("Error updating page");
                        process_fragments(&mut conn, existing_page_id, &config);
                    } else {
                        info!("Hash matches. Skipping page.");
                    }
                }
                _ => {
                    error!("Error inserting page: {}", e);
                }
            },
        }
    }
}

fn process_fragments(conn: &mut diesel::PgConnection, page_id: uuid::Uuid, config: &OutputConfig) {
    for fragment in &config.fragments {
        let unique_selectors = find_unique_selectors(&config.full_html, &fragment.html);

        let element_id = insert_element(
            conn,
            NewElement {
                page_id,
                name: &fragment.name,
                html: &fragment.html,
            },
        )
        .expect("Error inserting element");

        for selector in unique_selectors {
            let selector_id = insert_selector(
                conn,
                NewSelector {
                    path: &selector.path,
                },
            )
            .expect("Error inserting selector");

            insert_element_selector(
                conn,
                NewElementSelector {
                    element_id,
                    selector_id,
                    matches: selector.matches as i32,
                },
            )
            .expect("Error inserting element selector");
        }
    }
}

fn find_unique_selectors(full_html: &str, target_html: &str) -> Vec<SelectorResult> {
    let document = Html::parse_document(full_html);

    match generate_selector_from_html(target_html) {
        Ok(target_selector) => {
            let target_text = extract_text_from_html(target_html);
            info!("Target inner text: '{}'", target_text);

            let mut selected_elements = document.select(&target_selector);
            let found = selected_elements.find(|elem| {
                let elem_html = elem.html();
                let elem_text = extract_text_from_html(&elem_html);
                info!("Checking element with text: '{}'", elem_text);
                let text_match = elem_text == target_text;
                info!("Text match result: '{}'", text_match);
                text_match
            });

            if let Some(target_div) = found {
                info!("Found the exact target div in the document.");
                let unique_selectors = find_unique_selectors_in_document(&document, target_div);
                return unique_selectors;
            } else {
                info!("Exact target div not found in the document.");
            }
        }
        Err(e) => {
            error!("Error generating selector: {}", e);
        }
    }

    vec![]
}

