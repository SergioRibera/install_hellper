pub mod structs;
pub mod err;

#[cfg(test)]
mod test;

use std::fs::{File, create_dir_all};
use std::io::Read;
use std::path::{Path, PathBuf};
use serde_json::from_str;

use structs::{ Configs, Step };

// Load Configs Block
pub fn load_configs_from_path(file_path: &str) -> Configs {
    let mut config_file_path: PathBuf = PathBuf::new();
    config_file_path.push(Path::new(&file_path));

    let config_file_display = config_file_path.display();
    create_dir_all(config_file_path.parent().unwrap()).unwrap(); // Create directory if not exists
    let mut file: File;
    if config_file_path.exists() {
        file = match File::open(&config_file_path) {
            Err(why) => panic!("Couldn't open {}: {}", config_file_display, why),
            Ok(file) => file,
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Err(why) => panic!("Couldn't read from {}: {}", config_file_display, why),
            Ok(_) => load_configs_from_str(&raw)
        }
    } else {
        println!("Config File not Exists");
        Configs::default()
    }
}

pub fn load_configs_from_str(content: &str) -> Configs {
    match from_str(&content) {
        Ok(data) => data,
        Err(why) => {
            println!("Failed to load Configs, then load default configs. The reason: \"{:?}\"", why);
            Configs::default()
        }
    }
}

pub struct EasyCommand {
    configs: Configs,
    current_step: usize,
}
pub struct EasyCommandBuilder {
    __configs: Configs,
    __current_step: usize,
}

impl EasyCommand {
    /// Get a count of Steps
    pub fn steps_coutn (&self) -> usize {
        self.configs.config_install.steps.len()
    }
    /// Get a reference to the easy command's configs.
    pub fn get_configs(&self) -> &Configs {
        &self.configs
    }
}

impl Iterator for EasyCommand {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step < self.configs.config_install.steps.len() {
            let next: Step = self.configs.config_install.steps.get(self.current_step).unwrap().to_owned();
            self.current_step += 1;
            return Some(next);
        }
        None
    }
}

impl EasyCommandBuilder {
    pub fn new() -> Self {
        Self {
            __configs: Configs::default(),
            __current_step: 0
        }
    }

    pub fn with_config(self, conf: Configs) -> Self {
        Self {
            __configs: conf,
            __current_step: self.__current_step
        }
    }

    pub fn build(self) -> EasyCommand {
        EasyCommand {
            configs: self.__configs,
            current_step: self.__current_step
        }
    }
}
