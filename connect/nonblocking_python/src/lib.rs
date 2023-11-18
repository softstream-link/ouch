pub mod callbacks;
pub mod clt;
pub mod core;
pub mod svc;
use crate::{
    clt::CltOuchSupervised,
    core::{ConId, ConType, Status},
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

#[pymodule]
fn ouch_connect_nonblocking_python(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<ConId>()?;
    m.add_class::<ConType>()?;
    m.add_class::<Status>()?;
    m.add_class::<CltOuchSupervised>()?;
    m.add_class::<SvcOuchSupervised>()?;
    Ok(())
}
