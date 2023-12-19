pub mod callbacks;
pub mod clt;
pub mod core;
pub mod svc;
use crate::core::{ConId, ConType, SendStatus};
use clt::{CltAuto, CltManual};
use lazy_static::lazy_static;
use log::LevelFilter;
use ouch_connect_nonblocking::prelude::{PollHandlerDynamic, SpawnedPollHandlerDynamic};
use svc::SvcManual;

use pyo3::{prelude::*, types::PyDict};

pub(crate) fn dict_2_json(msg: Py<PyDict>) -> String {
    Python::with_gil(|py| {
        let locals = Some(PyDict::new(py));
        locals.unwrap().set_item("msg", msg).unwrap();
        let res = py.eval("dumps(msg)", None, locals).unwrap();
        let json = res.extract::<String>().unwrap();
        json
    })
}
pub(crate) fn json_2_dict(msg: &str) -> Py<PyDict> {
    Python::with_gil(|py| {
        let locals = Some(PyDict::new(py));
        locals.unwrap().set_item("msg", msg).unwrap();
        let res = py.eval("loads(msg)", None, locals).unwrap();
        res.extract::<Py<PyDict>>().unwrap()
    })
}

lazy_static! {
    pub(crate) static ref POLL_HANDLER: SpawnedPollHandlerDynamic = PollHandlerDynamic::default().into_spawned_handler("Poll-Thread");
}

// static spawned_poll_handler: SpawnedPollHandlerDynamic = PollHandlerDynamic::default().into_spawned_handler("Poll-Thread");

#[pymodule]
fn ouch_connect_nonblocking_python(py: Python, m: &PyModule) -> PyResult<()> {
    // IMPORTANT - default init of py03 logger will cause background threads to block or deadlock
    // as they need to acquire the GIL to log messages, So being very conservative and only allowing
    // lib WARN and above to be logged irrespective of the log level set by the user in the python log config
    // https://docs.rs/pyo3-log/latest/pyo3_log/ LOGGING WILL DEAD LOCK PYTHON
    // pyo3_log::init();
    use pyo3_log::{Caching, Logger};
    Logger::new(py, Caching::LoggersAndLevels)?.filter(LevelFilter::Warn).install().expect("Someone installed a logger before us :-(");

    m.add_class::<ConId>()?;
    m.add_class::<ConType>()?;
    m.add_class::<SendStatus>()?;
    m.add_class::<CltAuto>()?;
    m.add_class::<CltManual>()?;
    m.add_class::<SvcManual>()?;
    Ok(())
}
