use crate::{core::ConId, json_2_dict};
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
        let name = "on_recv";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_recv"));
    }
}
impl CallbackRecv<CltOuchMessenger> for PythonProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &SvcOuchMsg) {
        let name = "on_recv";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_recv"));
    }
}
impl CallbackSend<SvcOuchMessenger> for PythonProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<SvcOuchMessenger as Messenger>::SendT) {
        let name = "on_sent";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_sent"));
    }
    fn on_send(&self, con_id: &ConIdRs, msg: &mut <SvcOuchMessenger as Messenger>::SendT) {
        let name = "on_send";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_send"));
    }
    fn on_fail(&self, con_id: &ConIdRs, msg: &<SvcOuchMessenger as Messenger>::SendT, e: &std::io::Error) {
        let name = "on_fail";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()), e.to_string());
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_fail"));
    }
}
impl CallbackSend<CltOuchMessenger> for PythonProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<CltOuchMessenger as Messenger>::SendT) {
        let name = "on_sent";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_sent"));
    }
    fn on_send(&self, con_id: &ConIdRs, msg: &mut <CltOuchMessenger as Messenger>::SendT) {
        let name = "on_send";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_send"));
    }
    fn on_fail(&self, con_id: &ConIdRs, msg: &<CltOuchMessenger as Messenger>::SendT, e: &std::io::Error) {
        let name = "on_fail";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()), e.to_string());
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_fail"));
    }
}
impl CallbackRecvSend<SvcOuchMessenger> for PythonProxyCallback {}
impl CallbackRecvSend<CltOuchMessenger> for PythonProxyCallback {}
