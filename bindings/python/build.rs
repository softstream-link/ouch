use std::env;

fn get_rpath() -> String {
    for path in env::split_paths(&std::env::var_os("PATH").unwrap()) {
        let p = path.join("python3").canonicalize().unwrap();
        if p.is_file() {
            println!("cargo:warning=build.rs: Found {}", p.display());
            let lib_rpath = path.join("../lib").canonicalize().unwrap();
            println!(
                "{}",
                format!(
                    "cargo:warning=build.rs: setting adding linker -rpath to libpython3.x.[so|dylib|dll] as '{}' 
                    cargo:warning=\tPlease make sure this is the same python installation as one that was used to run 'maturin develop'.
                    cargo:warning=\tIf it is not the same it is likely that you cargo test will fail with 'Library not loaded: @rpath/libpython3.x.[so|dylib|dll]''",
                    lib_rpath.display()
                )
            );

            return lib_rpath.into_os_string().into_string().unwrap();
        }
    }
    panic!("build.rs: Failed. `python3` not found in PATH, which was necessary to find libpython3.x.[so|dylib|dll]")
}
fn main() {
    pyo3_build_config::add_extension_module_link_args();

    let python_lib_rpath = get_rpath();

    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", python_lib_rpath);
}
