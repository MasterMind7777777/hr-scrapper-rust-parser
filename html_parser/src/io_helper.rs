use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::parser::SelectorResult;

#[derive(Deserialize)]
pub struct Config {
    pub main_html: String,
    pub fragments: Vec<FragmentConfig>,
}

#[derive(Deserialize)]
pub struct FragmentConfig {
    pub name: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct Output {
    pub results: HashMap<String, HashMap<String, Vec<String>>>,
}

pub fn load_config(path: &str) -> Result<Config, std::io::Error> {
    let config_data = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}

pub fn save_output(path: &str, output: &Output) -> Result<(), std::io::Error> {
    let output_data = serde_json::to_string_pretty(output)?;
    fs::write(path, output_data)
}

pub fn process_fragments(config: &Config, find_unique_selectors: fn(&str, &str) -> Vec<SelectorResult>) -> Output {
    let mut results = HashMap::new();

    let main_html_content = fs::read_to_string(&config.main_html).expect("Error reading main HTML file");
    info!("Main HTML content loaded successfully");

    for fragment in &config.fragments {
        let target_html_content = fs::read_to_string(&fragment.path).expect("Error reading fragment HTML file");
        info!("Target HTML content '{}' loaded successfully", fragment.name);

        let selectors = find_unique_selectors(&main_html_content, &target_html_content);
        let selectors_map: HashMap<String, Vec<String>> = selectors.into_iter().map(|s| (s.path, vec![s.matches.to_string()])).collect();

        results.insert(fragment.name.clone(), selectors_map);
    }

    Output { results }
}

