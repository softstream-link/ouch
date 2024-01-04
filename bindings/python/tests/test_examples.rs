use std::error::Error;

use links_core::unittest::setup;
use log::info;
use pyo3::Python;

#[test]
fn test_python_examples() -> Result<(), Box<dyn Error>> {
    setup::log::configure();
    // append_to_inittab!(ouch_connect);
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or("Expected $CARGO_MANIFEST_DIR environment variable")?;
    let manifest_dir = std::path::PathBuf::from(manifest_dir);
    let examples_dir = manifest_dir.join("examples");
    let mut paths = std::fs::read_dir(&examples_dir)?
        .map(|res| res.unwrap())
        .filter(|dir| dir.path().is_file() && dir.path().extension().unwrap_or_default() == "py")
        .map(|dir| dir.path())
        .collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        let short_name = path.clone().into_os_string().into_string().unwrap().replace(manifest_dir.clone().into_os_string().into_string().unwrap().as_str(), "");
        info!("test_clt2svc_connect_auto: .{}", short_name);
        let example = std::fs::read_to_string(path.clone())?;

        Python::with_gil(|py| Python::run(py, example.as_str(), None, None)).unwrap();
    }

    Ok(())
}
