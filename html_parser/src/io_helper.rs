use serde::{Deserialize, Serialize};
use std::fs;

// Input structure
#[derive(Deserialize)]
pub struct InputConfig {
    pub url: String,
    pub full_html_path: String,
    pub fragments: Vec<InputFragment>,
}

#[derive(Deserialize)]
pub struct InputFragment {
    pub name: String,
    pub path: String,
}

// Output structure
#[derive(Serialize)]
pub struct OutputConfig {
    pub url: String,
    pub full_html: String,
    pub fragments: Vec<OutputFragment>,
}

#[derive(Serialize)]
pub struct OutputFragment {
    pub name: String,
    pub html: String,
}

// Function to load input configurations and generate output
pub fn load_and_process_config(path: &str) -> Result<Vec<OutputConfig>, std::io::Error> {
    let config_data = fs::read_to_string(path)?;
    let input_configs: Vec<InputConfig> = serde_json::from_str(&config_data)?;

    let mut output_configs = Vec::new();

    for input_config in input_configs {
        let full_html = fs::read_to_string(&input_config.full_html_path)?;

        let mut fragments = Vec::new();

        for fragment in input_config.fragments {
            let html = fs::read_to_string(&fragment.path)?;
            let output_fragment = OutputFragment {
                name: fragment.name,
                html,
            };
            fragments.push(output_fragment);
        }

        let output_config = OutputConfig {
            url: input_config.url,
            full_html,
            fragments,
        };

        output_configs.push(output_config);
    }

    Ok(output_configs)
}

