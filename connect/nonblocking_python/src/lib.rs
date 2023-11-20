pub mod callbacks;
pub mod clt;
pub mod core;
pub mod svc;
use core::{AcceptStatus, RecvStatus};

use crate::{
    clt::CltOuchSupervised,
    core::{ConId, ConType, SendStatus},
    svc::SvcOuchSupervised,
};

use pyo3::{prelude::*, types::PyDict};

pub(crate) fn dict_2_json(msg: &PyDict) -> String {
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
    Ok(())
}
