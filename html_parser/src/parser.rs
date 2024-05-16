use std::path::Path;

use itertools::Itertools;
use log::info;
use scraper::{ElementRef, Html, Selector};

pub fn initialize_logging() {
    let config_path = "src/log4rs.yaml";
    if Path::new(config_path).exists() {
        log4rs::init_file(config_path, Default::default()).unwrap_or_else(|err| {
            eprintln!("Failed to initialize logging: {}", err);
        });
    } else {
        eprintln!("Configuration file not found: {}", config_path);
    }
}

pub fn extract_text_from_html(html: &str) -> String {
    Html::parse_fragment(html)
        .root_element()
        .first_child()
        .and_then(ElementRef::wrap)
        .map(|elem| {
            elem.text()
                .collect::<Vec<_>>()
                .join(" ")
                .replace(['\n', '\t', ' '], "")
        })
        .unwrap_or_default()
}

pub fn normalize_class_attribute(value: &str) -> String {
    let mut classes: Vec<&str> = value.split_whitespace().collect();
    classes.sort_unstable();
    classes.join(" ")
}

pub fn generate_selector_from_html(html: &str) -> Result<Selector, String> {
    let fragment = Html::parse_fragment(html);
    fragment
        .root_element()
        .first_child()
        .and_then(ElementRef::wrap)
        .ok_or_else(|| "No elements found in the provided HTML.".to_string())
        .and_then(|elem| {
            let tag_name = elem.value().name();
            let classes = elem
                .value()
                .attr("class")
                .map(normalize_class_attribute)
                .map(|classes| format!(".{}", classes.replace(' ', ".")))
                .unwrap_or_default();

            let selector_string = format!("{}{}", tag_name, classes);
            info!("Generated selector string: '{}'", selector_string);
            Selector::parse(&selector_string)
                .map_err(|_| "Failed to parse generated selector.".to_string())
        })
}

pub fn check_full_path_uniqueness(document: &Html, path: &[(String, Vec<String>)]) -> usize {
    let path_string = path
        .iter()
        .rev()
        .map(|(tag, classes)| {
            let mut class_string = String::new();
            for c in classes {
                class_string.push('.');
                class_string.push_str(c);
            }
            format!("{}{}", tag, class_string)
        })
        .collect::<Vec<_>>()
        .join(" > ");

    let selector = Selector::parse(&path_string).expect("Failed to parse selector for path");
    let matches = document.select(&selector).count();
    info!(
        "Path '{}' occurs {} times in the document.",
        path_string, matches
    );
    matches
}

pub fn generate_class_combinations(classes: &[String]) -> Vec<Vec<String>> {
    let mut sorted_classes = classes.to_vec();
    sorted_classes.sort_unstable();

    (1..=sorted_classes.len())
        .flat_map(|size| sorted_classes.iter().combinations(size))
        .map(|combo| combo.into_iter().cloned().collect())
        .collect()
}

pub fn fetch_tag_and_classes(element: &ElementRef) -> (String, Vec<String>) {
    let tag_name = element.value().name().to_string();
    let classes = element.value().attr("class").unwrap_or("");
    let class_list: Vec<String> = classes
        .split_whitespace()
        .filter(|class| !class.is_empty())
        .map(String::from)
        .collect();
    (tag_name, class_list)
}

#[derive(Debug)]
pub struct SelectorResult {
    pub path: String,
    pub matches: usize,
}

pub fn find_unique_selectors_in_document(
    document: &Html,
    element: ElementRef,
) -> Vec<SelectorResult> {
    let mut unique_selectors = Vec::new();
    let mut current_element = Some(element);
    let mut path = Vec::new();

    while let Some(elem) = current_element {
        let (tag_name, classes) = fetch_tag_and_classes(&elem);
        let class_combinations = generate_class_combinations(&classes);

        for class_combo in class_combinations {
            path.push((tag_name.clone(), class_combo.clone()));
            let matches = check_full_path_uniqueness(document, &path);

            let selector_string = path
                .iter()
                .rev()
                .map(|(tag, classes)| {
                    let mut class_string = String::new();
                    for c in classes {
                        class_string.push('.');
                        class_string.push_str(c);
                    }
                    format!("{}{}", tag, class_string)
                })
                .collect::<Vec<_>>()
                .join(" > ");
            unique_selectors.push(SelectorResult {
                path: selector_string,
                matches,
            });

            path.pop();
        }

        path.push((tag_name, classes));
        current_element = elem.parent().and_then(ElementRef::wrap);
    }

    unique_selectors
}
