use bindgen::CargoCallbacks;
use ironside_build_tools::Dictionary;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::PathBuf;

fn main() {
    let dict_path = [env!("CARGO_MANIFEST_DIR"), "klipper", "out", "klipper.dict"]
        .iter()
        .collect::<PathBuf>();
    cargo_emit::rerun_if_changed!(dict_path.to_string_lossy());

    let dict_reader = File::open(dict_path)
        .map(BufReader::new)
        .expect("failed to reader klipper.dict");

    // Generate the commands from the klipper data dictionary
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let d: Dictionary = serde_json::from_reader(dict_reader).expect("failed to parse klipper.dict");
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

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings")
}
