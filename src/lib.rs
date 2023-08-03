use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct Controller {
    cloud: String,
    region: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "agent-version")]
    agent_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Controllers {
    controllers: HashMap<String, Controller>,

    #[serde(rename = "current-controller")]
    #[serde(default)]
    current_controller: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Models {
    controllers: HashMap<String, ControllerModels>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ControllerModels {
    models: HashMap<String, ModelType>,

    #[serde(rename = "current-model")]
    #[serde(default)]
    current_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ModelType {
    Iaas(Model),
    Caas(Model),
}

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    uuid: String,
    branch: String,
}

fn juju_status(controller_name: String, model_name: String) -> String {
    let output = Command::new("juju")
        .arg("status")
        .arg("--format")
        .arg("yaml")
        .arg(format!("{}:{}", controller_name, model_name))
        .output()
        .expect("juju status command failed to start");

    String::from_utf8_lossy(&output.stdout).to_string()
}

// Controller -> Model -> App
type JujuTree = HashMap<String, HashMap<String, Vec<String>>>;

pub struct JujuArtifacts {
    controllers_filepath: std::path::PathBuf,
    models_filepath: std::path::PathBuf,
}

impl JujuArtifacts {
    pub fn new(
        controllers_filepath: std::path::PathBuf,
        models_filepath: std::path::PathBuf,
    ) -> Self {
        Self {
            controllers_filepath,
            models_filepath,
        }
    }

    fn load_model_from_path<T>(path: &std::path::PathBuf) -> T
    where
        T: DeserializeOwned,
    {
        let f = std::fs::File::open(path).unwrap_or_else(|_| panic!("File not found: {:?}", path));
        let data: T =
            serde_yaml::from_reader(f).unwrap_or_else(|_| panic!("Error reading file: {:?}", path));
        data
    }

    fn get_tree(&self) -> JujuTree {
        let mut tree = JujuTree::new();
        let controllers = Self::load_model_from_path::<Controllers>(&self.controllers_filepath);
        let models = Self::load_model_from_path::<Models>(&self.models_filepath);

        for (ctl_name, ctl_type) in &controllers.controllers {
            let ctl_display_name = format!(
                "{} ({}, {})",
                ctl_name, ctl_type.agent_version, ctl_type.type_
            );

            // Prepend a '*' if it's the active controller
            let ctl_display_name = if ctl_name == &controllers.current_controller {
                format!("* {}", ctl_display_name)
            } else {
                format!("  {}", ctl_display_name)
            };

            let mut mdls: HashMap<String, Vec<String>> = HashMap::new();
            for model_name in models.controllers[ctl_name].models.keys() {
                // Prepend a '*' if it's the active model
                let model_display_name =
                    if model_name == &models.controllers[ctl_name].current_model {
                        format!("* {}", model_name)
                    } else {
                        format!("  {}", model_name)
                    };
                let apps = vec![]; // TODO: jst -m microk8s:welcome-k8s --format yaml
                mdls.insert(model_display_name, apps.clone());
            }
            tree.insert(ctl_display_name, mdls);
        }
        tree
    }

    pub fn print_tree(&self) {
        let data = self.get_tree();

        // We need to sort keys, otherwise the output will jump around
        let mut ctl_names: Vec<&String> = data.keys().collect();
        ctl_names.sort_by_key(|k| k.trim_start_matches(|c| c == ' ' || c == '*'));
        for ctl_name in ctl_names {
            println!("{}{}", "  ".repeat(0), ctl_name);

            let mdl_map = data.get(ctl_name).unwrap();
            let mut mdl_names: Vec<&String> = mdl_map.keys().collect();
            mdl_names.sort_by_key(|k| k.trim_start_matches(|c| c == ' ' || c == '*'));
            for mdl_name in mdl_names {
                println!("{}{}", " ".repeat(4), mdl_name);

                let app_vec = mdl_map.get(mdl_name).unwrap();
                for app_name in app_vec {
                    println!("{}{}", " ".repeat(8), app_name);
                }
            }
        }
    }
}
