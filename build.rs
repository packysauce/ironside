use std::env;
use std::path::PathBuf;
use bindgen::CargoCallbacks;

fn main() {
    cc::Build::new()
        .files(&["klipper/src/command.c"])
        .include("klipper/src")
        .include("klipper/out")
        .include(".")
        .shared_flag(true)
        .compile("commandc");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings")
}
