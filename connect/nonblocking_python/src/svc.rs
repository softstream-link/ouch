use std::num::NonZeroUsize;

use ouch_connect_nonblocking::prelude::SvcOuchSupervised as SvcOuchSupervisedRs;
use ouch_connect_nonblocking::prelude::{PoolAcceptCltNonBlocking, SendNonBlocking, SvcOuchMsg};
use pyo3::prelude::*;

use crate::{callbacks::PythonProxyCallback, core::Status};

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
    fn send(&mut self, msg: String) {
        let mut msg: SvcOuchMsg = serde_json::from_str(msg.as_str()).unwrap();
        self.0.send(&mut msg).unwrap();
    }
    fn accept(&mut self) -> PyResult<Status> {
        Ok(self.0.pool_accept()?.into())
    }
}
