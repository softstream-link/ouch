pub mod callback;
pub mod clt;
pub mod core;
pub mod svc;
use crate::callback::{ConId, ConType};
use clt::{CltAuto, CltManual};
use log::info;
use pyo3::prelude::*;
use svc::{SvcAuto, SvcManual};

#[pymodule]
fn ouch_connect(_py: Python, m: &PyModule) -> PyResult<()> {
    // IMPORTANT - py03 logger can cause background threads to block or deadlock as they need to acquire the GIL to log messages in python.
    // IMPORTANT - py03_log::init() will enable all logging including debug to be passed to python, even if PYTHON only logs INFO.
    // hence being conservative and only allowing WARN & above to be logged in release mode
    // https://docs.rs/pyo3-log/latest/pyo3_log/ LOGGING WILL DEAD LOCK PYTHON
    #[cfg(debug_assertions)]
    {
        // pyo3_log::init();
        let log = pyo3_log::try_init();
        if log.is_err() {
            info!("Looks like someone initialized logging prior to pyo3_log::try_init() -> {}", log.unwrap_err());
        }
    }
    #[cfg(not(debug_assertions))]
    {
        use pyo3_log::{Caching, Logger};
        Logger::new(_py, Caching::LoggersAndLevels)?.filter(log::LevelFilter::Warn).install().expect("Someone installed a logger before us :-(");
    }

    m.add_class::<ConId>()?;
    m.add_class::<ConType>()?;
    m.add_class::<CltAuto>()?;
    m.add_class::<CltManual>()?;
    m.add_class::<SvcManual>()?;
    m.add_class::<SvcAuto>()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::ouch_connect;
    use links_core::unittest::setup;
    use log::info;
    use pyo3::{append_to_inittab, Python};
    use std::error::Error;

    #[test]
    fn test_clt2svc_connect_auto() -> Result<(), Box<dyn Error>> {
        setup::log::configure();
        append_to_inittab!(ouch_connect);
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

        //         let code = r#"
        // import logging
        // logging.basicConfig(
        //     format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
        // )
        // logging.getLogger().setLevel(logging.INFO)

        // from ouch_connect import *;
        // con_ty = ConType.Initiator
        // logging.info(con_ty)
        // logging.info("test")

        //         "#;
        //         Python::with_gil(|py| Python::run(py, code, None, None)).unwrap();

        Ok(())
    }
}
