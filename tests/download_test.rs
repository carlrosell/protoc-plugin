use proto_pdk_test_utils::*;

generate_download_install_tests!("protoc-test", "28.3.0");

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::Linux, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.41.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            download_name: Some("protoc-1.41-linux-aarch_64.zip".into()),
            download_url: "https://github.com/protocolbuffers/protobuf/releases/download/v1.41/protoc-1.41-linux-aarch_64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::Linux, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            download_name: Some("protoc-1.2-linux-x86_64.zip".into()),
            download_url: "https://github.com/protocolbuffers/protobuf/releases/download/v1.2/protoc-1.2-linux-x86_64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_macos_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::MacOS, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            download_name: Some("protoc-1.2-osx-aarch_64.zip".into()),
            download_url: "https://github.com/protocolbuffers/protobuf/releases/download/v1.2/protoc-1.2-osx-aarch_64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_macos_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::MacOS, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            download_name: Some("protoc-1.2-osx-x86_64.zip".into()),
            download_url: "https://github.com/protocolbuffers/protobuf/releases/download/v1.2/protoc-1.2-osx-x86_64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic(expected = "Unable to install Protoc, unsupported architecture arm64 for windows.")]
async fn doesnt_support_windows_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::Windows, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            download_name: Some("protoc-1.2-win32.zip".into()),
            download_url: "https://github.com/protocolbuffers/protobuf/releases/download/v1.2/protoc-1.2-win32.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_windows_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::Windows, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            download_name: Some("protoc-1.2-win64.zip".into()),
            download_url: "https://github.com/protocolbuffers/protobuf/releases/download/v1.2/protoc-1.2-win64.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn locates_unix_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::Linux, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
            })
            .await
            .exes
            .get("protoc")
            .unwrap()
            .exe_path,
        Some("bin/protoc".into())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("protoc-test", |config| {
            config.host(HostOS::Windows, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("").unwrap(),
                    ..Default::default()
                },
            })
            .await
            .exes
            .get("protoc")
            .unwrap()
            .exe_path,
        Some("bin/protoc.exe".into())
    );
}
