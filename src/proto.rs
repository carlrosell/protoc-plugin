// Import required dependencies
use crate::config::ProtocPluginConfig;
use crate::version::{from_protoc_version, to_protoc_version};
use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;
use std::collections::HashMap;

// Define external host function for executing commands
#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

// Name of the tool/plugin
static NAME: &str = "Protoc";

// Register the tool/plugin and provide metadata about its capabilities
#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine, // Specify this is a command line tool
        config_schema: Some(SchemaBuilder::build_root::<ProtocPluginConfig>()), // Configuration schema
        minimum_proto_version: Some(Version::new(0, 42, 0)), // Minimum proto version required
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(), // Current plugin version
        ..ToolMetadataOutput::default()
    }))
}

// Load available versions from Protobuf's GitHub repository
#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    // Fetch git tags and process them to get version numbers
    // filter all tags with more than 1 dot and remove the v prefix
    let tags = load_git_tags("https://github.com/protocolbuffers/protobuf")?
        .iter()
        .filter(|tag| tag.split('.').count() == 2)
        .map(|tag| from_protoc_version(tag.strip_prefix('v').unwrap()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

// Download prebuilt binaries for the specified version
#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    // Get the host environment information
    let env = get_host_environment()?;

    // Verify the current OS and architecture are supported
    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::X86],
        ],
    )?;

    let version = &input.context.version;

    if version.is_canary() {
        return Err(plugin_err!(PluginError::UnsupportedCanary {
            tool: NAME.into()
        }));
    }

    let version = to_protoc_version(version);

    // match both platform and architecture
    let target = match (env.os, env.arch) {
        (HostOS::Linux, HostArch::Arm64) => format!("protoc-{version}-linux-aarch_64"),
        (HostOS::Linux, HostArch::X64) => format!("protoc-{version}-linux-x86_64"),
        (HostOS::MacOS, HostArch::Arm64) => format!("protoc-{version}-osx-aarch_64"),
        (HostOS::MacOS, HostArch::X64) => format!("protoc-{version}-osx-x86_64"),
        (HostOS::Windows, HostArch::X64) => format!("protoc-{version}-win64"),
        (HostOS::Windows, HostArch::X86) => format!("protoc-{version}-win32"),
        _ => unreachable!(),
    };

    let filename = format!("{target}.zip");

    let host = get_tool_config::<ProtocPluginConfig>()?.dist_url;

    Ok(Json(DownloadPrebuiltOutput {
        download_url: host
            .replace("{version}", &version)
            .replace("{file}", &filename),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

// Locate executable files and directories for the tool
#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    let exes_dir = "bin";

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "protoc".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("bin/protoc")),
        )]),
        exes_dir: Some(exes_dir.into()),
        globals_lookup_dirs: vec![format!("$TOOL_DIR/{exes_dir}"), "$HOME/.local/bin".into()],
        ..LocateExecutablesOutput::default()
    }))
}
