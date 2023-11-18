use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use ouch_connect_nonblocking::prelude::{asserted_short_name, CallbackRecv, CallbackRecvSend, CallbackSend, ConId, SvcOuchMessenger, CltOuchMsg, CltOuchMessenger, SvcOuchMsg};
use pyo3::prelude::*;
use serde_json::to_string;

#[derive(Debug)]
pub struct Callback {
    callback: PyObject,
}
impl Callback {
    pub fn new_ref(callback: PyObject) -> Arc<Self> {
        Arc::new(Self { callback })
    }
}
impl Display for Callback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", asserted_short_name!("Callback", Self))
    }
}
impl CallbackRecv<SvcOuchMessenger> for Callback {
    fn on_recv(&self, con_id: &ConId, msg: &CltOuchMsg) {
        Python::with_gil(|py| {
            let msg = to_string(msg).unwrap();
            self.callback.call_method1(py, "on_recv", (con_id.name(), msg)).expect("failed to call on_recv")
        });
    }
}
impl CallbackRecv<CltOuchMessenger> for Callback {
    fn on_recv(&self, con_id: &ConId, msg: &SvcOuchMsg) {
        Python::with_gil(|py| {
            let json = to_string(msg).unwrap();
            self.callback.call_method1(py, "on_recv", (con_id.name(), json)).unwrap()
        });
    }
}
impl CallbackSend<SvcOuchMessenger> for Callback {
    fn on_sent(&self, con_id: &ConId, msg: &SvcOuchMsg) {
        Python::with_gil(|py| {
            let json = to_string(msg).unwrap();
            self.callback.call_method1(py, "on_sent", (con_id.name(), json)).unwrap()
        });
    }
}
impl CallbackSend<CltOuchMessenger> for Callback {
    fn on_sent(&self, con_id: &ConId, msg: &CltOuchMsg) {
        Python::with_gil(|py| {
            let msg = to_string(msg).unwrap();
            self.callback.call_method1(py, "on_sent", (con_id.name(), msg)).unwrap()
        });
    }
}
impl CallbackRecvSend<SvcOuchMessenger> for Callback {}
impl CallbackRecvSend<CltOuchMessenger> for Callback {}
