use crate::core::ConId;
use ouch_connect_nonblocking::prelude::{asserted_short_name, CallbackRecv, CallbackRecvSend, CallbackSend, CltOuchMessenger, CltOuchMsg, ConId as ConIdRs, Messenger, SvcOuchMessenger, SvcOuchMsg};
use pyo3::prelude::*;
use serde_json::to_string;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

#[derive(Debug)]
pub struct PythonProxyCallback(PyObject);
impl PythonProxyCallback {
    pub fn new_ref(callback: PyObject) -> Arc<Self> {
        Arc::new(Self(callback))
    }
}
impl Display for PythonProxyCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", asserted_short_name!("Callback", Self))
    }
}
impl CallbackRecv<SvcOuchMessenger> for PythonProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &CltOuchMsg) {
        Python::with_gil(|py| {
            let name = "on_recv";
            let args = (ConId::from(con_id), to_string(msg).unwrap());
            let kwargs = None;
            self.0.call_method(py, name, args, kwargs).expect("Failed to call on_recv")
        });
    }
}
impl CallbackRecv<CltOuchMessenger> for PythonProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &SvcOuchMsg) {
        Python::with_gil(|py| {
            let name = "on_recv";
            let args = (ConId::from(con_id), to_string(msg).unwrap());
            let kwargs = None;
            self.0.call_method(py, name, args, kwargs).expect("Failed to call on_recv")
        });
    }
}
impl CallbackSend<SvcOuchMessenger> for PythonProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &SvcOuchMsg) {
        Python::with_gil(|py| {
            let name = "on_sent";
            let args = (ConId::from(con_id), to_string(msg).unwrap());
            let kwargs = None;
            self.0.call_method(py, name, args, kwargs).expect("Failed to call on_sent")
        });
    }
}
impl CallbackSend<CltOuchMessenger> for PythonProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &CltOuchMsg) {
        Python::with_gil(|py| {
            let name = "on_sent";
            let args = (ConId::from(con_id), to_string(msg).unwrap());
            let kwargs = None;
            self.0.call_method(py, name, args, kwargs).expect("Failed to call on_sent")
        });
    }
    fn on_fail(&self, con_id: &ConIdRs, msg: &CltOuchMsg, e: &std::io::Error) {
        Python::with_gil(|py| {
            let name = "on_fail";
            let args = (ConId::from(con_id), to_string(msg).unwrap(), e.to_string());
            let kwargs = None;
            self.0.call_method(py, name, args, kwargs).expect("Failed to call on_fail")
        });
    }
}
impl CallbackRecvSend<SvcOuchMessenger> for PythonProxyCallback {}
impl CallbackRecvSend<CltOuchMessenger> for PythonProxyCallback {}
