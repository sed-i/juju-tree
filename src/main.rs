use juju_tree::JujuArtifacts;

fn main() {
    let home_dir = home::home_dir().unwrap();
    let controllers_path = home_dir.join(".local/share/juju/controllers.yaml");
    let models_path = home_dir.join(".local/share/juju/models.yaml");

    let artifacts = JujuArtifacts::new(controllers_path, models_path);
    artifacts.print_tree();
}
