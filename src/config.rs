use once_cell::sync::OnceCell;
use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Deserialize;
use serde::Serialize;
use std::{collections::HashMap, fs, iter::FromIterator, path::PathBuf};

static EDITOR: OnceCell<String> = OnceCell::new();
static CONFIG: OnceCell<Config> = OnceCell::new();
static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

pub fn setup_config() {
    EDITOR.set(std::env::var("EDITOR").unwrap_or("vim".to_string()));
    let config_path = std::env::home_dir()
        .map(|home| home.join(".config").join("kielip").join("config.ron"))
        .unwrap();
    CONFIG_PATH.set(config_path);
    CONFIG.set(Config::from_file(get_config_path())).unwrap();
}

pub fn get_config_path() -> &'static PathBuf {
    CONFIG_PATH.get().unwrap()
}

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}
pub fn get_editor() -> &'static String {
    EDITOR.get().unwrap()
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    max_history: usize,
    actions: HashMap<String, (bool, Action)>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_history: 50,
            actions: HashMap::new(),
        }
    }
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Config {
        let result = if path.exists() {
            let contents =
                fs::read_to_string(path).expect("Something went wrong reading the config file");
            match from_str(&contents) {
                Ok(config) => config,
                Err(_) => Default::default(),
            }
        } else {
            Default::default()
        };
        result
    }
    pub fn max_history(self: &Config) -> usize {
        self.max_history
    }
    pub fn actions(self: &Config) -> &HashMap<String, (bool, Action)> {
        &self.actions
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Action {
    None,
    Remove,
    Scramble,
    Exec { command_pattern: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ron() {
        let mut actions = HashMap::new();
        actions.insert("/home/jigsaw".to_string(), (false, Action::Scramble));
        actions.insert("/home/bignaw".to_string(), (false, Action::Scramble));
        actions.insert(
            "https://.*".to_string(),
            (
                true,
                Action::Exec {
                    command_pattern: "firefox -private-window {}".to_string(),
                },
            ),
        );
        let data = Config {
            max_history: 50,
            actions,
        };
        let pretty = PrettyConfig::new()
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true);
        let s = to_string_pretty(&data, pretty).expect("Serialization failed");
        println!("{}", &s);
        let data1: Config = match from_str(&s) {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to load config: {}", e);
            }
        };
        assert_eq!(&data1, &data);
    }
}
