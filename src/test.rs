#[cfg(test)]
mod tests {
    use crate::structs::{ConfigInstall, Configs, Step};
    use crate::{EasyCommandBuilder, load_configs_from_str};

    #[test]
    fn test_load_configs() {
        let json_example = "
            {
                \"config_install\": {
                    \"steps\": [
                        {
                            \"name\": \"Install Packages\",
                            \"description\": \"Lorem\",
                            \"type_step\": \"Custom\",
                            \"cmd\": \"sudo pacman -S\",
                            \"os_target\": \"linux\",
                            \"args\": [
                                \"alacritty\",
                                \"zsh\"
                            ]
                        }
                    ]
                }
            }
            ";
        let config = load_configs_from_str(json_example);
        assert_eq!(config.config_install.steps.len(), 1);
        assert_eq!(config.config_install.steps[0].type_step, "Custom");
        assert_eq!(config.config_install.steps[0].description, "Lorem");
        assert_eq!(config.config_install.steps[0].args.len(), 2);
    }

    #[test]
    fn test_build_easy_command() {
        let config = Configs {
            config_install: ConfigInstall {
                steps: vec![ Step {
                    name: "Hello World".to_string(),
                    description: "The global".to_string(),
                    type_step: "Custom".to_string(),
                    cmd: "echo".to_string(),
                    args: vec![ "\"Hello World!!\"".to_string() ],
                    os_target: "Linux".to_string(),
                } ]
            }
        };
        let easy = EasyCommandBuilder::new()
                        .with_config(config)
                        .build();
        assert_eq!(easy.current_step, 0);
        assert_eq!(easy.configs.config_install.steps.len(), 1);
        assert_eq!(easy.configs.config_install.steps[0].cmd, "echo".to_string());
    }
}
