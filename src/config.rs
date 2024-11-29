#[derive(Debug, schematic::Schematic, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct ProtocPluginConfig {
    pub dist_url: String,
}

impl Default for ProtocPluginConfig {
    fn default() -> Self {
        Self {
            dist_url:
                "https://github.com/protocolbuffers/protobuf/releases/download/v{version}/{file}"
                    .into(),
        }
    }
}
