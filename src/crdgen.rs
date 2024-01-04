mod crds;

use kube::CustomResourceExt;

fn main() {
    println!("---");
    print!("{}", serde_yaml::to_string(&crds::Manager::crd()).unwrap());
    println!("---");
    print!("{}", serde_yaml::to_string(&crds::Table::crd()).unwrap());
    println!("---");
}
