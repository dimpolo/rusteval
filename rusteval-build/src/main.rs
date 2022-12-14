use rustdoc_json::{Builder, Package};
use rusteval_build::collect::{CrateItems, Error};

fn main() -> Result<(), Error> {
    let builder = Builder::default().package(Package::Bin("embedded_example"));
    CrateItems::collect(builder)?;
    Ok(())
}
