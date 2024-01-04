fn main() {
    pyo3_build_config::add_extension_module_link_args();

    let python_lib_rpath = get_rpath();
    println!("cargo:warning=build.rs: setting adding linker -rpath to libpython3.x.[so|dylib|dll] as '{}'", python_lib_rpath);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", python_lib_rpath);
}

fn get_rpath() -> String {
    match std::env::var_os("CONDA_PREFIX") {
        Some(path) => {
            println!("cargo:warning=build.rs: using CONDA_PREFIX={:?}", path);
            let rpath = std::path::PathBuf::from(path).join("lib");
            let rpath = rpath.canonicalize().expect(format!("Expected $CONDA_PREFIX/lib to be valid path").as_str());
            rpath.into_os_string().into_string().unwrap()
        }
        None => panic!("build.rs: Failed. CONDA_PREFIX, it is necessary to find correct libpython3.x.[so|dylib|dll] for py03 module"),
    }
}
