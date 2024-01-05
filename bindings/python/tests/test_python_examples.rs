use links_core::unittest::setup;
use log::info;
use pyo3::Python;
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

#[test]
fn test_python_examples() {
    setup::log::configure();
    // append_to_inittab!(ouch_connect);
    let pkg_name = env!("CARGO_PKG_NAME");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let examples_dir = manifest_dir.join("examples");

    let mut paths = read_dir(&examples_dir)
        .unwrap()
        .map(|res| res.unwrap())
        .filter(|dir| dir.path().is_file() && dir.path().extension().unwrap_or_default() == "py")
        .map(|dir| dir.path())
        .collect::<Vec<_>>();
    paths.sort();

    for path in paths {
        let short_name = path.to_str().unwrap().replace(manifest_dir.to_str().unwrap(), "");
        info!("{} - .{}", pkg_name, short_name);
        let example = read_to_string(path).unwrap();
        // println!("******************************* START *******************************");
        Python::with_gil(|py| Python::run(py, example.as_str(), None, None)).unwrap();
        println!("*******************************  END  *******************************");
    }
}
