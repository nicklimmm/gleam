use std::collections::HashMap;
use std::path::PathBuf;

use crate::requirement::Requirement;
use crate::Result;
use hexpm::version::Version;
use itertools::Itertools;
use smol_str::SmolStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Manifest {
    #[serde(serialize_with = "ordered_map")]
    pub requirements: HashMap<String, Requirement>,
    #[serde(serialize_with = "sorted_vec")]
    pub packages: Vec<ManifestPackage>,
}

impl Manifest {
    // Rather than using the toml library to do serialization we implement it
    // manually so that we can control the formatting.
    // We want to keep entries on a single line each so that they are more
    // resistant to merge conflicts and are easier to fix when it does happen.
    pub fn to_toml(&self) -> String {
        let mut buffer = String::new();
        let Self {
            requirements,
            packages,
        } = self;

        buffer.push_str(
            "# This file was generated by Gleam
# You typically do not need to edit this file

",
        );

        // Packages
        buffer.push_str("packages = [\n");
        for ManifestPackage {
            name,
            source,
            version,
            otp_app,
            build_tools,
            requirements,
        } in packages.iter().sorted_by(|a, b| a.name.cmp(&b.name))
        {
            buffer.push_str(r#"  {"#);
            buffer.push_str(r#" name = ""#);
            buffer.push_str(name);
            buffer.push_str(r#"", version = ""#);
            buffer.push_str(&version.to_string());
            buffer.push_str(r#"", build_tools = ["#);
            for (i, tool) in build_tools.iter().enumerate() {
                if i != 0 {
                    buffer.push_str(", ");
                }
                buffer.push('"');
                buffer.push_str(tool);
                buffer.push('"');
            }

            buffer.push_str("], requirements = [");
            for (i, package) in requirements.iter().enumerate() {
                if i != 0 {
                    buffer.push_str(", ");
                }
                buffer.push('"');
                buffer.push_str(package);
                buffer.push('"');
            }
            buffer.push(']');

            if let Some(app) = otp_app {
                buffer.push_str(", otp_app = \"");
                buffer.push_str(app);
                buffer.push('"');
            }

            match source {
                ManifestPackageSource::Hex { outer_checksum } => {
                    buffer.push_str(r#", source = "hex", outer_checksum = ""#);
                    buffer.push_str(&outer_checksum.to_string());
                    buffer.push('"');
                }
                ManifestPackageSource::Git { repo, commit } => {
                    buffer.push_str(r#", source = "git", repo = ""#);
                    buffer.push_str(repo);
                    buffer.push_str(r#"", commit = ""#);
                    buffer.push_str(commit);
                    buffer.push('"');
                }
                ManifestPackageSource::Local { path } => {
                    buffer.push_str(r#", source = "local", path = ""#);
                    buffer.push_str(path.to_str().expect("local path non utf-8"));
                    buffer.push('"');
                }
            };

            buffer.push_str(" },\n");
        }
        buffer.push_str("]\n\n");

        // Requirements
        buffer.push_str("[requirements]\n");
        for (name, requirement) in requirements.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
            buffer.push_str(name);
            buffer.push_str(" = ");
            buffer.push_str(&requirement.to_toml());
            buffer.push('\n');
        }

        buffer
    }
}

#[test]
fn manifest_toml_format() {
    let mut manifest = Manifest {
        requirements: [
            ("zzz".into(), Requirement::hex("> 0.0.0")),
            ("aaa".into(), Requirement::hex("> 0.0.0")),
            (
                "awsome_local2".into(),
                Requirement::git("https://github.com/gleam-lang/gleam.git"),
            ),
            (
                "awsome_local1".into(),
                Requirement::path("../path/to/package"),
            ),
            ("gleam_stdlib".into(), Requirement::hex("~> 0.17")),
            ("gleeunit".into(), Requirement::hex("~> 0.1")),
        ]
        .into(),
        packages: vec![
            ManifestPackage {
                name: "gleam_stdlib".into(),
                version: Version::new(0, 17, 1),
                build_tools: ["gleam".into()].into(),
                otp_app: None,
                requirements: vec![],
                source: ManifestPackageSource::Hex {
                    outer_checksum: Base16Checksum(vec![1, 22]),
                },
            },
            ManifestPackage {
                name: "aaa".into(),
                version: Version::new(0, 4, 0),
                build_tools: ["rebar3".into(), "make".into()].into(),
                otp_app: Some("aaa_app".into()),
                requirements: vec!["zzz".into(), "gleam_stdlib".into()],
                source: ManifestPackageSource::Hex {
                    outer_checksum: Base16Checksum(vec![3, 22]),
                },
            },
            ManifestPackage {
                name: "zzz".into(),
                version: Version::new(0, 4, 0),
                build_tools: ["mix".into()].into(),
                otp_app: None,
                requirements: vec![],
                source: ManifestPackageSource::Hex {
                    outer_checksum: Base16Checksum(vec![3, 22]),
                },
            },
            ManifestPackage {
                name: "awsome_local2".into(),
                version: Version::new(1, 2, 3),
                build_tools: ["gleam".into()].into(),
                otp_app: None,
                requirements: vec![],
                source: ManifestPackageSource::Git {
                    repo: "https://github.com/gleam-lang/gleam.git".into(),
                    commit: "bd9fe02f72250e6a136967917bcb1bdccaffa3c8".into(),
                },
            },
            ManifestPackage {
                name: "awsome_local1".into(),
                version: Version::new(1, 2, 3),
                build_tools: ["gleam".into()].into(),
                otp_app: None,
                requirements: vec![],
                source: ManifestPackageSource::Local {
                    path: "/home/louis/packages/path/to/package".into(),
                },
            },
            ManifestPackage {
                name: "gleeunit".into(),
                version: Version::new(0, 4, 0),
                build_tools: ["gleam".into()].into(),
                otp_app: None,
                requirements: vec!["gleam_stdlib".into()],
                source: ManifestPackageSource::Hex {
                    outer_checksum: Base16Checksum(vec![3, 46]),
                },
            },
        ],
    };
    let buffer = manifest.to_toml();
    assert_eq!(
        buffer,
        r#"# This file was generated by Gleam
# You typically do not need to edit this file

packages = [
  { name = "aaa", version = "0.4.0", build_tools = ["rebar3", "make"], requirements = ["zzz", "gleam_stdlib"], otp_app = "aaa_app", source = "hex", outer_checksum = "0316" },
  { name = "awsome_local1", version = "1.2.3", build_tools = ["gleam"], requirements = [], source = "local", path = "/home/louis/packages/path/to/package" },
  { name = "awsome_local2", version = "1.2.3", build_tools = ["gleam"], requirements = [], source = "git", repo = "https://github.com/gleam-lang/gleam.git", commit = "bd9fe02f72250e6a136967917bcb1bdccaffa3c8" },
  { name = "gleam_stdlib", version = "0.17.1", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "0116" },
  { name = "gleeunit", version = "0.4.0", build_tools = ["gleam"], requirements = ["gleam_stdlib"], source = "hex", outer_checksum = "032E" },
  { name = "zzz", version = "0.4.0", build_tools = ["mix"], requirements = [], source = "hex", outer_checksum = "0316" },
]

[requirements]
aaa = { version = "> 0.0.0" }
awsome_local1 = { path = "../path/to/package" }
awsome_local2 = { git = "https://github.com/gleam-lang/gleam.git" }
gleam_stdlib = { version = "~> 0.17" }
gleeunit = { version = "~> 0.1" }
zzz = { version = "> 0.0.0" }
"#
    );
    let deserialised: Manifest = toml::from_str(&buffer).unwrap();
    manifest.packages.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(deserialised, manifest);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Base16Checksum(pub Vec<u8>);

impl ToString for Base16Checksum {
    fn to_string(&self) -> String {
        base16::encode_upper(&self.0)
    }
}

impl serde::Serialize for Base16Checksum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base16::encode_upper(&self.0))
    }
}

impl<'de> serde::Deserialize<'de> for Base16Checksum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
        base16::decode(s)
            .map(Base16Checksum)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct ManifestPackage {
    pub name: String,
    pub version: Version,
    pub build_tools: Vec<String>,
    #[serde(default)]
    pub otp_app: Option<String>,
    #[serde(serialize_with = "sorted_vec")]
    pub requirements: Vec<String>,
    #[serde(flatten)]
    pub source: ManifestPackageSource,
}

impl ManifestPackage {
    pub fn with_build_tools(mut self, build_tools: &'static [&'static str]) -> Self {
        self.build_tools = build_tools.iter().map(|s| (*s).to_string()).collect();
        self
    }

    #[inline]
    pub fn is_hex(&self) -> bool {
        matches!(self.source, ManifestPackageSource::Hex { .. })
    }

    #[inline]
    pub fn is_local(&self) -> bool {
        matches!(self.source, ManifestPackageSource::Local { .. })
    }
}

#[cfg(test)]
impl Default for ManifestPackage {
    fn default() -> Self {
        Self {
            name: Default::default(),
            build_tools: Default::default(),
            otp_app: Default::default(),
            requirements: Default::default(),
            version: Version::new(1, 0, 0),
            source: ManifestPackageSource::Hex {
                outer_checksum: Base16Checksum(vec![]),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(tag = "source")]
pub enum ManifestPackageSource {
    #[serde(rename = "hex")]
    Hex { outer_checksum: Base16Checksum },
    #[serde(rename = "git")]
    Git { repo: SmolStr, commit: SmolStr },
    #[serde(rename = "local")]
    Local { path: PathBuf }, // should be the canonical path
}

fn ordered_map<S, K, V>(value: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    K: serde::Serialize + Ord,
    V: serde::Serialize,
{
    use serde::Serialize;
    let ordered: std::collections::BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

fn sorted_vec<S, T>(value: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize + Ord,
{
    use serde::Serialize;
    let mut value: Vec<&T> = value.iter().collect();
    value.sort();
    value.serialize(serializer)
}
