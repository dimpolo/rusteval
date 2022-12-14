use rusteval_build::collect::CrateItems;
use rusteval_build::rustdoc_json::{Builder, Package};
use std::fmt::Debug;

fn main() {
    /*   return;
    if let Err(e) = try_main() {
        panic!("{e:?}")
    }*/
}

fn _try_main() -> Result<(), Box<dyn Debug>> {
    let builder = Builder::default().package(Package::Bin("embedded_example"));
    let _crate_items = CrateItems::collect(builder).map_err(_map_debug)?;
    Ok(())
}

fn _map_debug<T: Debug + 'static>(t: T) -> Box<dyn Debug> {
    Box::new(t)
}
