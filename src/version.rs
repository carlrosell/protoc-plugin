use proto_pdk::VersionSpec;

pub fn from_protoc_version(version: &str) -> String {
    // Zero releases don't end in ".0",
    // so we must fix manually...
    let suffix = match version.matches('.').count() {
        1 => ".0",
        0 => ".0.0",
        _ => "",
    };

    // Handle prerelease versions like rc3
    if let Some((base, pre)) = version.split_once('-') {
        return format!("{base}{suffix}-{pre}");
    }

    format!("{version}{suffix}")
}

pub fn to_protoc_version(spec: &VersionSpec) -> String {
    match spec {
        VersionSpec::Alias(alias) => alias.into(),
        VersionSpec::Semantic(version) => {
            if version.pre.is_empty() {
                format!("{}.{}", version.major, version.minor)
            } else {
                format!("{}.{}-{}", version.major, version.minor, version.pre)
            }
        }
        _ => spec.as_version().unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_from() {
        assert_eq!(from_protoc_version("1.0"), "1.0.0");
        assert_eq!(from_protoc_version("1.2"), "1.2.0");
        assert_eq!(from_protoc_version("1.2-rc3"), "1.2.0-rc3");
    }

    #[test]
    fn formats_to() {
        assert_eq!(
            to_protoc_version(&VersionSpec::parse("1.0.0").unwrap()),
            "1.0"
        );
        assert_eq!(
            to_protoc_version(&VersionSpec::parse("1.2.0").unwrap()),
            "1.2"
        );
        assert_eq!(
            to_protoc_version(&VersionSpec::parse("1.2.3").unwrap()),
            "1.2"
        );
        assert_eq!(
            to_protoc_version(&VersionSpec::parse("1.2.0-rc3").unwrap()),
            "1.2-rc3"
        );
    }
}
