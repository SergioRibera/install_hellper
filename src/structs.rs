fn parse_line (l: String) -> Option<(String, String)> {
    let words: Vec<&str> = l.splitn(2, '=').collect();
    if words.len() < 2 {
        return None
    }
    let mut trim_value = String::from(words[1]);

    if trim_value.starts_with('"') {
        trim_value.remove(0);
    }
    if trim_value.ends_with('"') {
        let len = trim_value.len();
        trim_value.remove(len - 1);
    }

    return Some((String::from(words[0]), trim_value))
}
pub fn get_os() -> Result<OsTarget, Error> {
    match env::consts::OS {
        "windows" => Ok(OsTarget::Windows),
        "macos" => Ok(OsTarget::Mac),
        "linux" => {
            let mut s = String::new();
            File::open("/etc/os-release")?.read_to_string(&mut s)?;

            let mut info: (String, String) = ("linux".to_string(), "Linux".to_string());
            for l in s.split('\n') {
                match parse_line (l.trim().to_string()) {
                    Some((key, value)) =>
                        match (key.as_ref(), value) {
                            ("ID", val) => info.0 = val,
                            ("PRETTY_NAME", val) => info.1 = val,
                            _ => {}
                        }
                    None => {}
                }
            }
            Ok(OsTarget::Linux(info))
        },
        _ => Err(Error::UnsupportedSystem)
    }
}

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;

use evalexpr::{ context_map, eval_boolean_with_context };
use regex::Regex;
use serde::Deserialize;

use crate::err::Error;

#[derive(Debug, Clone)]
pub enum OsTarget {
    Any,
    Windows,
    Mac,
    // (id, pretty_name)
    Linux((String, String)), // String is a distro name
}

fn default_type_step() -> String {
    "Custom".to_string()
}
fn default_os_target() -> String {
    "Any".to_string()
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Cmd {
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Step {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_type_step")]
    pub type_step: String, // Posibilities: Custom
    #[serde(default)]
    pub commands: Vec<Cmd>,
    #[serde(default = "default_os_target")]
    pub os_target: String, // Posibilities: Any, Windows, Mac, Linux
    #[serde(default)]
    pub show_outputs: bool,
    #[serde(default)]
    pub sentence: String,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ConfigInstall {
    pub steps: Vec<Step>,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Configs {
    pub config_install: ConfigInstall,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            name: String::new(),
            description: String::new(),
            type_step: String::from("Custom"),
            commands: Vec::new(),
            os_target: String::from("Any"),
            show_outputs: false,
            sentence: String::new(),
        }
    }
}
impl Step {
    #[allow(dead_code)]
    pub fn exec(&mut self, mut options_selecteds: Vec<String>) {
        // TODO: only os compatible
        let linux_distro: String = match get_os() {
            Ok(OsTarget::Linux((name, _))) => name,
            Ok(_) => "".to_string(),
            Err(_) => "".to_string(),
        };
        let context = context_map! {
            "os" => env::consts::OS,
            "linux_distro" => linux_distro,
        }.unwrap(); // Do proper error handling here
        let sentence_eval: bool = eval_boolean_with_context(&self.sentence, &context)
               .unwrap();

        // TODO: move regex to Installer and create new Step class for add more parametters
        let exp_braces = Regex::new(r"\{\{|\}\}|\{([^}]+)\}").unwrap();

        if sentence_eval == true || self.sentence.is_empty() {
            for cmd in &self.commands {
                let mut new_args: Vec<String> = Vec::new();

                for arg in cmd.args.clone() {
                    for capture in exp_braces.captures_iter(arg.as_str()) {
                        if let Some(matched) = capture.get(1) {
                            if matched.as_str() == "options" {
                                new_args.append(&mut options_selecteds);
                            }
                        }
                    }
                }
                if new_args.len() == 0 {
                    new_args = options_selecteds.clone();
                }
                let output = Command::new(cmd.cmd.clone())
                    .args(new_args)
                    .output()
                    .expect("failed to execute process");
                if self.show_outputs {
                    if output.status.success() {
                        print!("{:?}", output.stdout);
                    } else {
                        print!("{:?}", output.stderr);
                    }
                }
            }
        }
    }
}
