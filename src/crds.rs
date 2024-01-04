use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube::CustomResource;

#[derive(CustomResource, JsonSchema, Serialize, Deserialize, Clone, Debug)]
#[kube(group="psql.midugh.fr", version="v1alpha1", kind = "Manager")]
#[kube(plural = "managers", singular="manager")]
pub struct ManagerSpec {
    pub uri: String,
}

#[allow(non_snake_case)]
#[derive(CustomResource, JsonSchema, Serialize, Deserialize, Clone, Debug)]
#[kube(group="psql.midugh.fr", version="v1alpha1", kind = "Table", namespaced)]
#[kube(plural = "tables", singular = "table")]
pub struct TableSpec {
    pub tableName: String,
    pub username: String,
    pub password: String,
}
