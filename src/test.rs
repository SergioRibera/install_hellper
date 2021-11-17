#[cfg(test)]
mod tests {
    use std::env;

    use evalexpr::{context_map, eval_boolean_with_context};

    use crate::structs::{Cmd, ConfigInstall, Configs, Step};
    use crate::{EasyCommandBuilder, load_configs_from_str};

    #[test]
    fn test_load_configs() {
        let json_example = r#"{
            "config_install": {
                "steps": [
                    {
                        "name": "Cloning Repo",
                        "description": "Lorem",
                        "commands": [
                            {
                                "cmd": "git",
                                "args": [
                                    "clone",
                                    "https://alskfksdfh.alkjd",
                                    "{options}"
                                ]
                            }
                        ]
                    }
                ]
            }
        }"#;
        let config = load_configs_from_str(json_example);
        assert_eq!(config.config_install.steps.len(), 1);
        assert_eq!(config.config_install.steps[0].type_step, "Custom");
        assert_eq!(config.config_install.steps[0].description, "Lorem");
        assert_eq!(config.config_install.steps[0].commands.len(), 1);
        assert_eq!(config.config_install.steps[0].os_target, "Any");
    }

    #[test]
    fn test_build_easy_command() {
        let config = Configs {
            config_install: ConfigInstall {
                steps: vec![ Step {
                    name: "Hello World".to_string(),
                    description: "The global".to_string(),
                    commands: vec![
                        Cmd {
                            cmd: "echo".to_string(),
                            args: vec![ "\"Hello World!!\"".to_string() ],
                        }
                    ],
                    os_target: "Linux".to_string(),
                    ..Default::default()
                } ]
            }
        };
        let easy = EasyCommandBuilder::new()
                        .with_config(config)
                        .build();
        assert_eq!(easy.current_step, 0);
        assert_eq!(easy.configs.config_install.steps.len(), 1);
        assert_eq!(easy.configs.config_install.steps[0].commands[0].cmd, "echo".to_string());
        assert_eq!(easy.configs.config_install.steps[0].os_target, "Linux".to_string());
    }

    #[test]
    fn test_sentence() {
        let json_example = format!(r#"{{
                "config_install": {{
                    "steps": [
                        {{
                            "name": "Test sentence",
                            "sentence": "os == \"{}\"",
                            "commands": [ ]
                        }}
                    ]
                }}
            }}
        "#, env::consts::OS);
        let config = load_configs_from_str(json_example.as_str());

        let context = context_map! {
            "os" => env::consts::OS,
        }.unwrap(); // Do proper error handling here

        // println!("{}", config.config_install.steps[0].sentence);
        assert_eq!(config.config_install.steps[0].sentence, format!("os == \"{}\"", env::consts::OS));
        assert_eq!(
            eval_boolean_with_context(&config.config_install.steps[0].sentence, &context),
            Ok(true));
    }
}
