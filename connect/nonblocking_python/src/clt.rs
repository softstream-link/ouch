use std::time::Duration;

use ouch_connect_nonblocking::prelude::CltOuchSupervised as CltOuchSupervisedRs;
use ouch_connect_nonblocking::prelude::{CltOuchMsg, SendNonBlocking};

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::callbacks::PythonProxyCallback;
use crate::core::Status;
use crate::dict_2_json;

#[pyclass]
pub struct CltOuchSupervised(CltOuchSupervisedRs<PythonProxyCallback>);

#[pymethods]
impl CltOuchSupervised {
    #[new]
    fn new(host: String, callback: PyObject, name: Option<&str>) -> Self {

        let callback = PythonProxyCallback::new_ref(callback);

        let clt = CltOuchSupervisedRs::connect(host.as_str(), Duration::from_secs(1), Duration::from_millis(100), callback, name).unwrap();

        Self(clt)
    }
    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
    fn send(&mut self, msg_any: &PyAny) -> PyResult<Status>{
        let json = {
            if let Ok(msg_dict) = msg_any.downcast::<PyDict>() {
                dict_2_json(msg_dict)
            } else if let Ok(msg_str) = msg_any.extract::<String>() {
                msg_str
            } else {
                panic!("msg is not a dict or str")
            }
        };
        let mut msg: CltOuchMsg = serde_json::from_str(json.as_str()).unwrap(); //TODO serde to PyError
        Ok(self.0.send(&mut msg)?.into())
    }
}

#[cfg(test)]
mod test {

    use crate::ouch_connect_nonblocking_python;
    use pyo3::{append_to_inittab, prepare_freethreaded_python, Python};

    #[test]
    fn test_clt() {
        append_to_inittab!(ouch_connect_nonblocking_python);
        prepare_freethreaded_python();

        let code = format!(
            r#"
import logging; logging.basicConfig(level=logging.DEBUG)
from {crate_name} import *

clt = CltOuchSupervised("127.0.0.1:8080", , "test")
        "#,
            crate_name = stringify!(ouch_connect_nonblocking_python)
        );

        Python::with_gil(|py| Python::run(py, code.as_str(), None, None)).unwrap();
    }
}
