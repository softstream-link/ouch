use crate::{core::ConId, json_2_dict};
use ouch_connect_nonblocking::prelude::{asserted_short_name, CallbackRecv, CallbackRecvSend, CallbackSend, CltOuchProtocolAuto, Messenger, SvcOuchMessenger, SvcOuchProtocolAuto, SvcOuchProtocolManual};
use ouch_connect_nonblocking::prelude::{CltOuchProtocolManual, ConId as ConIdRs};
use pyo3::{prelude::*, types::PyDict};
use serde::Serialize;
use serde_json::to_string;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

enum Method {
    OnRecv,
    OnSent,
}
#[derive(Debug)]
pub struct PyProxyCallback(PyObject);
impl PyProxyCallback {
    pub fn new_ref(callback: PyObject) -> Arc<Self> {
        // TODO add assert to ensure that callback has correct methods or instance of Callback
        Arc::new(Self(callback))
    }

    fn issue_callback<O: Serialize>(&self, method: Method, con_id: &ConIdRs, msg: &O) {
        let name = match method {
            Method::OnRecv => "on_recv",
            Method::OnSent => "on_sent",
        };
        // convert msg to str
        let json_str = to_string(msg).unwrap();
        let msg = json_str.as_str();
        let con_id = ConId::from(con_id);

        Python::with_gil(|py| {
            // convert str to dict
            let locals = Some(PyDict::new(py));
            locals.unwrap().set_item("msg", msg).unwrap();
            let res = py.eval("loads(msg)", None, locals).unwrap();
            let py_dic = res.extract::<Py<PyDict>>().unwrap();

            // issue callback
            let args = (con_id, py_dic);
            let kwargs = None;
            self.0.call_method(py, name, args, kwargs).expect(format!("Failed to call {}", name).as_str());
        })
    }
}
impl Display for PyProxyCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", asserted_short_name!("PyProxyCallback", Self))
    }
}
// CltManual
impl CallbackRecv<CltOuchProtocolManual> for PyProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &<CltOuchProtocolManual as Messenger>::RecvT) {
        self.issue_callback(Method::OnRecv, con_id, msg)
    }
}
impl CallbackSend<CltOuchProtocolManual> for PyProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<CltOuchProtocolManual as Messenger>::SendT) {
        self.issue_callback(Method::OnSent, con_id, msg);
    }
}
impl CallbackRecvSend<CltOuchProtocolManual> for PyProxyCallback {}
// CltAuto
impl CallbackRecv<CltOuchProtocolAuto> for PyProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &<CltOuchProtocolAuto as Messenger>::RecvT) {
        self.issue_callback(Method::OnRecv, con_id, msg)
    }
}
impl CallbackSend<CltOuchProtocolAuto> for PyProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<CltOuchProtocolAuto as Messenger>::SendT) {
        self.issue_callback(Method::OnSent, con_id, msg);
    }
}
impl CallbackRecvSend<CltOuchProtocolAuto> for PyProxyCallback {}
// SvcManual
impl CallbackRecv<SvcOuchProtocolManual> for PyProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &<SvcOuchProtocolManual as Messenger>::RecvT) {
        self.issue_callback(Method::OnRecv, con_id, msg)
    }
}
impl CallbackSend<SvcOuchProtocolManual> for PyProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<SvcOuchProtocolManual as Messenger>::SendT) {
        self.issue_callback(Method::OnSent, con_id, msg);
    }
}
impl CallbackRecvSend<SvcOuchProtocolManual> for PyProxyCallback {}
// SvcAuto
impl CallbackRecv<SvcOuchProtocolAuto> for PyProxyCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &<SvcOuchProtocolAuto as Messenger>::RecvT) {
        self.issue_callback(Method::OnRecv, con_id, msg)
    }
}
impl CallbackSend<SvcOuchProtocolAuto> for PyProxyCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<SvcOuchProtocolAuto as Messenger>::SendT) {
        self.issue_callback(Method::OnSent, con_id, msg);
    }
}
impl CallbackRecvSend<SvcOuchProtocolAuto> for PyProxyCallback {}

#[derive(Debug)]
pub struct PythonProxySvcCallback(PyObject);
impl PythonProxySvcCallback {
    pub fn new_ref(callback: PyObject) -> Arc<Self> {
        // TODO add assert to ensure that callback has correct methods or instance of Callback
        Arc::new(Self(callback))
    }
}
impl Display for PythonProxySvcCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", asserted_short_name!("Callback", Self))
    }
}
impl CallbackRecvSend<SvcOuchMessenger> for PythonProxySvcCallback {}
impl CallbackRecv<SvcOuchMessenger> for PythonProxySvcCallback {
    fn on_recv(&self, con_id: &ConIdRs, msg: &<SvcOuchMessenger as Messenger>::RecvT) {
        let name = "on_recv";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_recv"));
    }
}
impl CallbackSend<SvcOuchMessenger> for PythonProxySvcCallback {
    fn on_sent(&self, con_id: &ConIdRs, msg: &<SvcOuchMessenger as Messenger>::SendT) {
        let name = "on_sent";
        let args = (ConId::from(con_id), json_2_dict(to_string(msg).unwrap().as_str()));
        let kwargs = None;
        Python::with_gil(|py| self.0.call_method(py, name, args, kwargs).expect("Failed to call on_sent"));
    }
}
