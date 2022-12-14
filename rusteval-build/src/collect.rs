use rustdoc_json::{BuildError, Builder, Package};
use rustdoc_types::{Crate, Id, Item};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

static SKIP_LIST: [&str; 5] = [
    "core",
    "alloc",
    "std",
    "compiler_builtins",
    "rustc_std_workspace_core",
];

pub struct CrateItems {
    pub items: HashMap<Id, Item>,
}

impl CrateItems {
    pub fn collect(builder: Builder) -> Result<Self, Error> {
        let json_path = builder.clone().build()?;
        let json_path2 = offset_json_path(json_path);

        let file = std::fs::read(dbg!(json_path2))?;

        let crate_: Crate = serde_json::from_slice(&file)?;
        let mut items = crate_.index;

        for external_crate in crate_.external_crates.values() {
            if SKIP_LIST.contains(&external_crate.name.as_str()) {
                continue;
            }

            let name = external_crate.name.replace('_', "-");

            let json_path = match builder.clone().package(Package::Lib(name)).build() {
                Ok(path) => path,
                Err(e) => {
                    dbg!(e);
                    continue;
                }
            };
            let json_path2 = offset_json_path(json_path);
            let file = std::fs::read(dbg!(json_path2))?;
            let crate_: Crate = serde_json::from_slice(&file)?;
            items.extend(crate_.index)
        }
        Ok(CrateItems { items })
    }
}

#[derive(Debug)]
pub enum Error {
    Rustdoc(BuildError),
    FileError(io::Error),
    JsonError(serde_json::Error),
}

impl From<BuildError> for Error {
    fn from(value: BuildError) -> Self {
        Error::Rustdoc(value)
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::FileError(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonError(value)
    }
}

fn offset_json_path(path: PathBuf) -> PathBuf {
    let directory = path.parent().unwrap().parent().unwrap();
    directory
        .join("thumbv7em-none-eabihf")
        .join("doc")
        .join(path.file_name().unwrap())
}

/*
Problems:
* https://github.com/Enselic/cargo-public-api/issues/102
    * Support for .cargo/config.toml #102
    * Some docs go to target/doc some to target/thumbv7em-none-eabihf/doc
* error: There are multiple `bare-metal` packages in your project, and the specification `bare-metal` is ambiguous.
    * cargo pkgid helps to identify duplicates
    * json gets overwritten by default though
* package ID specification `num-enum-derive` did not match any packages Did you mean `num_enum_derive`?
* running from build script dead-locks
    * https://github.com/rust-lang/cargo/issues/6412
*/
