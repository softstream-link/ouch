pub mod callback;
pub mod clt;
pub mod core;
pub mod svc;
use crate::callback::{ConId, ConType};
use clt::{CltAuto, CltManual};
use pyo3::prelude::*;
use svc::{SvcAuto, SvcManual};

#[pymodule]
fn ouch_bindings_py(_py: Python, m: &PyModule) -> PyResult<()> {
    // IMPORTANT - default init of py03 logger will cause background threads to block or deadlock
    // as they need to acquire the GIL to log messages, So being very conservative and only allowing
    // lib WARN and above to be logged irrespective of the log level set by the user in the python log config
    // https://docs.rs/pyo3-log/latest/pyo3_log/ LOGGING WILL DEAD LOCK PYTHON
    // pyo3_log::init();
    // TODO add WARN logging on release vs INFO on debug
    use pyo3_log::{Caching, Logger};
    Logger::new(_py, Caching::LoggersAndLevels)?.filter(log::LevelFilter::Warn).install().expect("Someone installed a logger before us :-(");

    m.add_class::<ConId>()?;
    m.add_class::<ConType>()?;
    m.add_class::<CltAuto>()?;
    m.add_class::<CltManual>()?;
    m.add_class::<SvcManual>()?;
    m.add_class::<SvcAuto>()?;
    Ok(())
}
