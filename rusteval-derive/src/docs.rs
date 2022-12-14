use once_cell::sync::Lazy;
use rustdoc_json::{BuildError, Builder};

#[derive(Debug)]
enum Error {
    Rustdoc(BuildError),
    FileError(std::io::Error),
    JsonError(serde_json::Error),
}

impl From<BuildError> for Error {
    fn from(value: BuildError) -> Self {
        Error::Rustdoc(value)
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::FileError(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonError(value)
    }
}

fn generate_docs(builder: Builder) -> Result<Crate, Error> {
    let json_path = builder.build()?;

    let file = std::fs::read(json_path)?;

    Ok(serde_json::from_slice(&file)?)
}

static DOCS: Lazy<Crate> = Lazy::new(|| {
    let builder = Builder::default()
        .toolchain("nightly".to_owned())
        .manifest_path("Cargo.toml")
        .document_private_items(true);
    generate_docs(builder).unwrap()
});
