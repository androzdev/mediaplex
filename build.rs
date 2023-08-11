// based on https://github.com/Lakelezz/audiopus_sys

extern crate napi_build;

use bindgen;
use std::path::PathBuf;
use std::{env, fmt::Display, path::Path};

const fn rustc_linking_word(is_static_link: bool) -> &'static str {
  if is_static_link {
    "static"
  } else {
    "dylib"
  }
}

fn generate_binding() {
  const ALLOW_UNCONVENTIONALS: &'static str = "#![allow(non_upper_case_globals)]\n\
                                                 #![allow(non_camel_case_types)]\n\
                                                 #![allow(non_snake_case)]\n";

  let bindings = bindgen::Builder::default()
    .header("src/opus/libopus.h")
    .raw_line(ALLOW_UNCONVENTIONALS)
    .generate()
    .expect("Unable to generate binding");

  let binding_target_path = PathBuf::new().join("src").join("opus").join("lib.rs");

  bindings
    .write_to_file(binding_target_path)
    .expect("Could not write binding to the file at `src/opus/lib.rs`");

  println!("cargo:info=Successfully generated binding.");
}

fn build_opus(is_static: bool) {
  let opus_path = Path::new("deps/libopus");

  println!(
    "cargo:info=Opus source path used: {:?}.",
    opus_path
      .canonicalize()
      .expect("Could not canonicalise to absolute path")
  );

  println!("cargo:info=Building Opus via CMake.");

  let mut cmake_config = cmake::Config::new(opus_path);

  if let Ok(android_target) = env::var("CARGO_NDK_ANDROID_TARGET") {
    cmake_config.define("ANDROID_ABI", android_target);

    if let Ok(toolchain_file) = env::var("CARGO_NDK_CMAKE_TOOLCHAIN_PATH") {
      cmake_config.define("CMAKE_TOOLCHAIN_FILE", toolchain_file);
    }
  }

  if let Ok(value) = env::var("CMAKE_OSX_SYSROOT") {
    cmake_config.configure_arg(format!("-DCMAKE_OSX_SYSROOT={value}"));
  }

  let opus_build_dir = cmake_config.build();
  link_opus(is_static, opus_build_dir.display())
}

fn link_opus(is_static: bool, opus_build_dir: impl Display) {
  let is_static_text = rustc_linking_word(is_static);

  println!(
    "cargo:info=Linking Opus as {} lib: {}",
    is_static_text, opus_build_dir
  );
  println!("cargo:rustc-link-lib={}=opus", is_static_text);
  println!("cargo:rustc-link-search=native={}/lib", opus_build_dir);
}

#[cfg(any(unix, target_env = "gnu"))]
fn find_via_pkg_config(is_static: bool) -> bool {
  use pkg_config;
  pkg_config::Config::new()
    .statik(is_static)
    .probe("opus")
    .is_ok()
}

fn default_library_linking() -> bool {
  #[cfg(any(windows, target_os = "macos", target_env = "musl"))]
  {
    true
  }
  #[cfg(any(target_os = "freebsd", all(unix, target_env = "gnu")))]
  {
    false
  }
}

fn is_static_build() -> bool {
  if cfg!(feature = "static") && cfg!(feature = "dynamic") {
    default_library_linking()
  } else if cfg!(feature = "static")
    || env::var("LIBOPUS_STATIC").is_ok()
    || env::var("OPUS_STATIC").is_ok()
  {
    println!("cargo:info=Static feature or environment variable found.");

    true
  } else if cfg!(feature = "dynamic") {
    println!("cargo:info=Dynamic feature enabled.");

    false
  } else {
    println!("cargo:info=No feature or environment variable found, linking by default.");

    default_library_linking()
  }
}

fn main() {
  generate_binding();

  let is_static = is_static_build();

  #[cfg(any(unix, target_env = "gnu"))]
  {
    if std::env::var("LIBOPUS_NO_PKG").is_ok() || std::env::var("OPUS_NO_PKG").is_ok() {
      println!("cargo:info=Bypassed `pkg-config`.");
    } else if find_via_pkg_config(is_static) {
      println!("cargo:info=Found `Opus` via `pkg_config`.");

      return;
    } else {
      println!("cargo:info=`pkg_config` could not find `Opus`.");
    }
  }

  build_opus(is_static);

  napi_build::setup();
}
