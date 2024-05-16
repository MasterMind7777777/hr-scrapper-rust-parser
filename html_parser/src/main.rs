mod db_operations;
mod models;
mod schema;

mod io_helper;
mod parser;

use db_operations::{
    establish_connection, insert_element, insert_element_selector, insert_page, insert_selector,
};
use io_helper::*;
use log::{error, info};
use models::{NewElement, NewElementSelector, NewPage, NewSelector};
use parser::*;
use scraper::Html;

fn main() {
    initialize_logging();

    let configs = load_and_process_config("src/config.json").expect("Error loading configuration file");

    let pool = establish_connection();
    let mut conn = pool.get().expect("Failed to get a database connection");

    for config in configs {
        let find_unique_selectors = |full_html: &str, target_html: &str| {
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
                        let unique_selectors =
                            find_unique_selectors_in_document(&document, target_div);
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
        };

        // let output = process_fragments(&config, find_unique_selectors);

        let name = config.url.trim_end_matches('/').split('/').last().unwrap_or("unknown");

        let page_id = insert_page(
            &mut conn,
            NewPage {
                name,
                url: &config.url,
                html: &config.full_html
            },
        )
        .expect("Error inserting page");

        for fragment in &config.fragments {
            let unique_selectors = find_unique_selectors(&config.full_html, &fragment.html);

            let element_id = insert_element(
                &mut conn,
                NewElement {
                    page_id,
                    name: &fragment.name,
                    html: &fragment.html,
                },
            )
            .expect("Error inserting element");

            for selector in unique_selectors {
                let selector_id = insert_selector(
                    &mut conn,
                    NewSelector {
                        path: &selector.path,
                    },
                )
                .expect("Error inserting selector");

                insert_element_selector(
                    &mut conn,
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

    // Uncomment the following line if you still need to save to JSON for debugging
    // save_output("output.json", &output).expect("Error saving output to JSON file");
}
