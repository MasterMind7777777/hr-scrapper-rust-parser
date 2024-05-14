mod parser;
mod io_helper;

use scraper::Html;
use parser::*;
use io_helper::*;
use log::error;
use log::info;

fn main() {
    initialize_logging();

    let config = load_config("config.json").expect("Error loading configuration file");

    let find_unique_selectors = |main_html: &str, target_html: &str| {
        let document = Html::parse_document(main_html);

        match generate_selector_from_html(target_html) {
            Ok(target_selector) => {
                let target_text = extract_text_from_html(target_html);
                info!("Target inner text: '{}'", target_text);

                let found = document.select(&target_selector).find(|elem| {
                    let elem_text = extract_text_from_html(&elem.html());
                    info!("Checking element with text: '{}'", elem_text);
                    elem_text == target_text
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
    };

    let results = process_fragments(&config, find_unique_selectors);
    save_output("output.json", &results).expect("Error saving output to JSON file");
}
