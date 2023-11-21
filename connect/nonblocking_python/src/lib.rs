pub mod callbacks;
pub mod clt;
pub mod core;
pub mod svc;
use crate::{
    clt::CltOuchSupervised,
    core::{ConId, ConType, SendStatus},
    svc::SvcOuchSupervised,
};
use core::{AcceptStatus, RecvStatus};
use lazy_static::lazy_static;
use ouch_connect_nonblocking::prelude::{PollHandlerDynamic, SpawnedPollHandlerDynamic};

use pyo3::{prelude::*, types::PyDict};
use svc::SvcOuchSender;

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
fn ouch_connect_nonblocking_python(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<ConId>()?;
    m.add_class::<ConType>()?;
    m.add_class::<AcceptStatus>()?;
    m.add_class::<SendStatus>()?;
    m.add_class::<RecvStatus>()?;
    m.add_class::<CltOuchSupervised>()?;
    m.add_class::<SvcOuchSupervised>()?;
    m.add_class::<SvcOuchSender>()?;
    Ok(())
}
