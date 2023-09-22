//package field
#[derive(serde::Deserialize)]
pub struct Package {
    pub version: String,
    #[serde(default)]
    pub metadata: Option<MetaData>,
}

//  package.metadata field
#[derive(serde::Deserialize)]
pub struct MetaData {
    #[serde(default)]
    pub hermes: Option<HermesMetaData>,
}

// package.metadata.hermes field
#[derive(serde::Deserialize)]
pub struct HermesMetaData {
    #[serde(default)]
    pub icon_android: Option<String>,
    #[serde(default)]
    pub icon_iphone: Option<String>,
    #[serde(default)]
    pub icon_web: Option<String>,
    #[serde(default)]
    pub require_webgpu: Option<bool>,
}

// struct for the toml and fields we need
#[derive(serde::Deserialize)]
pub struct WorkspaceToml {
    pub package: Package,
}
