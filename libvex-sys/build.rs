// build.rs

use std::env::{self, VarError};
use std::error::Error;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::process::Command;

use fs_extra::dir::{copy, CopyOptions};

type Result<T> = std::result::Result<T, Box<dyn Error>>;


fn vex_headers() -> Result<Vec<String>> {
    match env::var("VEX_HEADERS") {
        Ok(paths) => {
            Ok(paths.split(':').map(String::from).collect())
        }
        Err(VarError::NotPresent) => {
            let mut vex = find_vex()?;
            let mut res = Vec::with_capacity(2);
            res.push(vex.to_string_lossy().into_owned());
            vex.pop();
            res.push(vex.to_string_lossy().into_owned());
            Ok(res)
        }
        Err(err) => Err(err.into()),
    }
}

fn apply_patches(valgrind_dir: PathBuf, patch_dir: PathBuf) -> Result<()> {
    let mut patch = Command::new("patch");
    patch
        .arg("-p1")
        .current_dir(valgrind_dir);
    for entry in patch_dir.read_dir()? {
        patch
            .stdin(File::open(entry?.file_name())?)
            .status()?;
    }
    Ok(())
}

fn copy_valgrind(out_dir: &Path) -> Result<()> {
    let mut options = CopyOptions::default();
    options.copy_inside = true;
    copy("valgrind", out_dir, &options)?;
    println!("cargo:rerun-if-changed=valgrind/");
    match env::var("VEX_PATCHES") {
        Ok(path) => apply_patches(out_dir.join("valgrind"), PathBuf::from(path))?,
        Err(VarError::NotUnicode(path)) =>
            apply_patches(out_dir.join("valgrind"), PathBuf::from(path))?,
        Err(VarError::NotPresent) => {}
    }
    Ok(())
}

fn find_vex() -> Result<PathBuf> {
    Ok(match env::var("VEX_SRC") {
        Ok(path) => {
            println!("cargo:rerun-if-changed={}", path);
            PathBuf::from(path)
        }
        // It would be nice to cargo:rerun-if-changed=path here, but can we?...
        Err(VarError::NotUnicode(path)) => PathBuf::from(path),
        Err(_) => {
            let out_dir = PathBuf::from(env::var("OUT_DIR")?);
            let valgrind_dir = out_dir.join("valgrind");
            if !valgrind_dir.exists() {
                copy_valgrind(&out_dir)?;
            }
            if !valgrind_dir.join("configure").exists() {
                Command::new("./autogen.sh")
                    .current_dir(&valgrind_dir)
                    .status()?;
            }
            if !valgrind_dir.join("VEX").join("Makefile").exists() {
                let mut configure = Command::new("./configure");
                configure.current_dir(&valgrind_dir);
                if cfg!(feature = "pic") {
                    configure.arg("CFLAGS=-fPIC");
                }
                configure.status()?;
            }
            valgrind_dir.join("VEX")
        }
    })
}

fn compile_vex() -> Result<PathBuf> {
    let src_dir = find_vex()?;
    Command::new("make")
        .env("MAKEFLAGS", env::var("CARGO_MAKEFLAGS").unwrap())
        .current_dir(&src_dir)
        .status()?;

    Ok(src_dir)
}

/// Ensure that libvex*.so is present on the file system.
///
/// Return its directory.
fn ensure_lib() -> Result<PathBuf> {
    match env::var("VEX_LIBS") {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(VarError::NotUnicode(path)) => Ok(PathBuf::from(path)),
        Err(_) => compile_vex(),
    }
}

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let host = env::var("HOST")?;

    {
        let (arch, platform) = {
            let mut host_parts = host.as_str().split("-");
            let arch = host_parts.next().unwrap();
            let arch = if arch == "x86_64" { "amd64" } else { arch };
            let _ = host_parts.next();
            let platform = host_parts.next().unwrap();

            (arch, platform)
        };

        let vex_dir = ensure_lib()?;

        // Tell rustc to link to libvex
        println!("cargo:rustc-link-search=native={}", vex_dir.display());
        println!("cargo:rustc-link-lib=static=vex-{}-{}", arch, platform);
    }

    {
        // Generate bindings
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .blacklist_type("_IRStmt__bindgen_ty_1__bindgen_ty_1")
            .rustified_enum("*")
            .clang_args(vex_headers()?
                        .into_iter()
                        .map(|dir| format!("-I{}", dir))
            )
            .generate()
            .map_err(|()| "Unable to generate bindings")?;
        bindings.write_to_file(out_dir.join("bindings.rs"))?;
    }

    Ok(())
}
