pub mod clt;
pub mod svc;

use clt::{CltAuto, CltManual};
use links_bindings_python::prelude::{create_register_atexit, ConId, ConType};
use pyo3::prelude::*;
use svc::{SvcAuto, SvcManual};

create_register_atexit!();

/// This is a Python extension-module over the `ouch_connect_nonblocking` library. Please refer to readme for more information.
#[pymodule]
fn ouch_connect(_py: Python, m: &PyModule) -> PyResult<()> {

    register_atexit()?;
    // IMPORTANT - py03 logger can cause background threads to block or deadlock as they need to acquire the GIL to log messages in python.
    // IMPORTANT - py03_log::init() will enable all logging including debug to be passed to python, even if PYTHON only logs INFO.
    // hence being conservative and only allowing WARN & above to be logged in release mode
    // https://docs.rs/pyo3-log/latest/pyo3_log/ LOGGING WILL DEAD LOCK PYTHON
    #[cfg(debug_assertions)]
    {
        // pyo3_log::init();
        let log = pyo3_log::try_init();
        if log.is_err() {
            log::info!("Looks like someone initialized logging prior to pyo3_log::try_init() -> {}", log.unwrap_err());
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
