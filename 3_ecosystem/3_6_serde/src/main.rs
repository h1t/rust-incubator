use std::error::Error;
use step_3_6::Request;

fn main() -> Result<(), Box<dyn Error>> {
    let json_data = include_str!("../request.json");
    let request: Request = serde_json::from_str(json_data)?;
    let yaml_data = serde_yaml::to_string(&request)?;
    let toml_data = toml::to_string(&request)?;

    println!("YAML:\n{yaml_data:?}");
    println!();
    println!("TOML:\n{toml_data:?}");

    Ok(())
}
