use std::num::NonZeroUsize;

use ouch_connect_nonblocking::prelude::{PoolAcceptCltNonBlocking, RecvNonBlocking, SendNonBlocking, SvcOuchMsg};
use ouch_connect_nonblocking::prelude::{RecvStatus as RecvStatusRs, SvcOuchSupervised as SvcOuchSupervisedRs};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::core::{AcceptStatus, RecvStatus};
use crate::{callbacks::PythonProxyCallback, core::SendStatus};
use crate::{dict_2_json, json_2_dict};

#[pyclass]
pub struct SvcOuchSupervised(SvcOuchSupervisedRs<PythonProxyCallback>);

#[pymethods]
impl SvcOuchSupervised {
    #[new]
    fn new(host: String, callback: PyObject, max_connections: NonZeroUsize, name: Option<&str>) -> Self {
        let callback = PythonProxyCallback::new_ref(callback);
        let svc = SvcOuchSupervisedRs::bind(host.as_str(), callback, max_connections, name).unwrap();
        Self(svc)
    }
    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
    fn send(&mut self, msg_any: &PyAny) -> PyResult<SendStatus> {
        let json = {
            if let Ok(msg_dict) = msg_any.downcast::<PyDict>() {
                dict_2_json(msg_dict)
            } else if let Ok(msg_str) = msg_any.extract::<String>() {
                msg_str
            } else {
                panic!("msg is not a dict or str")
            }
        };
        let mut msg: SvcOuchMsg = serde_json::from_str(json.as_str()).unwrap();
        Ok(self.0.send(&mut msg)?.into())
    }
    fn pool_accept(&mut self) -> PyResult<AcceptStatus> {
        Ok(self.0.pool_accept()?.into())
    }
    fn recv(&mut self) -> PyResult<RecvStatus> {
        let status = self.0.recv()?;

        match status {
            RecvStatusRs::Completed(Some(msg)) => {
                let json = serde_json::to_string(&msg).unwrap(); // TODO raise PyError
                Ok(RecvStatus(Some(json_2_dict(json.as_str()))))
            }
            RecvStatusRs::WouldBlock => Ok(RecvStatus(None)),
            RecvStatusRs::Completed(None) => Err(PyErr::from(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))),
        }
    }
}
