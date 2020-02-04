use std::env;
use std::path::PathBuf;

fn get_cc_defines() -> Vec<(&'static str, Option<&'static str>)> {
    let mut defines = Vec::new();

    if cfg!(feature = "ma_debug") {
        defines.push(("MA_DEBUG_OUTPUT", None));
    }

    if cfg!(feature = "ma_flac_decoder") {
        defines.push(("DR_FLAC_IMPLEMENTATION", None));
    }

    if cfg!(feature = "ma_mp3_decoder") {
        defines.push(("DR_MP3_IMPLEMENTATION", None));
    }

    if cfg!(feature = "ma_vorbis_decoder") {
        defines.push(("MA_VORBIS_DECODER", None));
    }

    if cfg!(feature = "ma_wav_decoder") {
        defines.push(("DR_WAV_IMPLEMENTATION", None));
    }

    defines
}

fn cc_files(files: Vec<&str>, defines: Vec<(&str, Option<&str>)>, output: &str) {
    let mut builder = cc::Build::new();

    for &define in &defines {
        builder.define(define.0, define.1);
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    builder
        .warnings(false)
        .files(files)
        .out_dir(out_dir.join("lib"))
        .compile(output);
}

fn bindgen_file(files: Vec<&str>, output: &str) {
    let mut builder = bindgen::Builder::default();

    for &file in &files {
        builder = builder.header(file);
    }

    let bindings = builder
        .derive_debug(false)
        .rustfmt_bindings(true)
        .whitelist_function("ma_.*")
        .whitelist_type("MA_.*")
        .whitelist_var("MA_.*")
        .generate()
        .expect(&format!("Unable to generate bindings for {} to {}",
                         files.join(","), output));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(output))
        .expect(&format!("Couldn't write bindings file: {}", output));
}

fn main() {
    cc_files(vec!["gen/wrapper/miniaudio.c"], get_cc_defines(), "libminiaudio.a");
    bindgen_file(vec!["vendor/miniaudio.h"], "bindings.rs");

    println!("cargo:rerun-if-changed=gen/build.rs");
    println!("cargo:rerun-if-changed=gen/wrapper/miniaudio.c");

    println!("cargo:rerun-if-changed=vendor/extras/dr_flac.h");
    println!("cargo:rerun-if-changed=vendor/extras/dr_mp3.h");
    println!("cargo:rerun-if-changed=vendor/extras/dr_wav.h");
    println!("cargo:rerun-if-changed=vendor/extras/stb_vorbis.c");
    println!("cargo:rerun-if-changed=vendor/miniaudio.h");
}
