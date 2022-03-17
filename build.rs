use bindgen::CargoCallbacks;
use ironside_build_tools::Dictionary;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const KLIPPER_DICT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/klipper/out/klipper.dict"
));

fn main() {
    // Generate the commands from the klipper data dictionary
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let d: Dictionary = serde_json::from_str(KLIPPER_DICT).unwrap();
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_dir.join("command_gen.rs"))
        .and_then(|mut f| writeln!(f, "{}", d.to_token_stream()))
        .expect("Failed to write generated commands");

    // Pull in some klipper sources as the need arises
    let sources = ["klipper/klippy/chelper/msgblock.c", "klipper/src/command.c"];
    cc::Build::new()
        .files(&sources)
        .include("klipper/src")
        .include("klipper/out")
        .include(".")
        .shared_flag(true)
        .compile("libklipper");

    // For all the random C stuff that just needs a little linker fix-up
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
