use std::env;

fn find_libpython_x_x_so() -> String {
    for path in env::split_paths(&std::env::var_os("PATH").unwrap()) {
        let mut p = path.join("python3");
        if p.is_symlink() {
            p = std::fs::canonicalize(&p).unwrap();
        }
        if p.is_file() {
            // path.push("python");
            // eprintln!("{p:?}");
            // panic!("{p:?}");
            let lib_path = path.join("../lib");
            return lib_path.into_os_string().into_string().unwrap();
            // break;
        }
    }
    panic!("python3 not found in PATH, which was necessary to find libpython3.x.[so|dylib|dll]")
}
fn main() {
    pyo3_build_config::add_extension_module_link_args();

    let python_lib_so_path = find_libpython_x_x_so();
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", python_lib_so_path);
}
