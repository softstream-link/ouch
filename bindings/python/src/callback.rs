use ouch_connect_nonblocking::prelude::{asserted_short_name, CallbackRecv, CallbackRecvSend, CallbackSend, CltOuchProtocolAuto, Messenger, SvcOuchProtocolAuto, SvcOuchProtocolManual};
use ouch_connect_nonblocking::prelude::{CltOuchProtocolManual, ConId as ConIdRs};
use pyo3::{prelude::*, types::PyDict};
use serde::Serialize;
use serde_json::to_string;
use std::fmt::Debug;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

#[pyclass]
#[derive(Debug, Clone)]
pub enum ConType {
    Initiator,
    Acceptor,
}
impl Display for ConType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConType::Initiator => write!(f, "Initiator"),
            ConType::Acceptor => write!(f, "Acceptor"),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ConId {
    pub con_type: ConType,
    pub name: String,
    pub local: String,
    pub peer: String,
}
// started
// acceptor
#[pymethods]
impl ConId {
    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }
}
impl Display for ConId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.con_type {
            ConType::Initiator => write!(f, "{}({}@{}->{})", self.con_type, self.name, self.local, self.peer),
            ConType::Acceptor => write!(f, "{}({}@{}<-{})", self.con_type,self.name, self.local, self.peer),
        }
    }
}

impl From<&ConIdRs> for ConId {
    fn from(value: &ConIdRs) -> Self {
        use ConIdRs::*;
        match value {
            Initiator { name, local, peer } => Self {
                con_type: ConType::Initiator,
                name: name.to_owned(),
                local: match local {
                    Some(local) => local.to_string(),
                    None => "pending".to_owned(),
                },
                peer: peer.to_string(),
            },

            Acceptor { name, local, peer } => Self {
                con_type: ConType::Acceptor,
                name: name.to_owned(),
                local: local.to_string(),
                peer: match peer {
                    Some(peer) => peer.to_string(),
                    None => "pending".to_owned(),
                },
            },
        }
    }
}

impl From<ConIdRs> for ConId {
    fn from(value: ConIdRs) -> Self {
        Self::from(&value)
    }
}

const ON_RECV: &str = "on_recv";
const ON_SENT: &str = "on_sent";

enum Method {
    OnRecv,
    OnSent,
}
impl Method {
    fn as_str(&self) -> &'static str {
        match self {
            Method::OnRecv => ON_RECV,
            Method::OnSent => ON_SENT,
        }
    }
}

#[derive(Debug)]
pub struct PyProxyCallback(PyObject);
impl PyProxyCallback {
    pub fn new_ref(callback: PyObject) -> Arc<Self> {
        Python::with_gil(|py| {
            callback.getattr(py, ON_RECV).expect(format!("callback must have {} method", ON_RECV).as_str());
            callback.getattr(py, ON_SENT).expect(format!("callback must have {} method", ON_SENT).as_str());
        });

        Arc::new(Self(callback))
    }

    fn issue_callback<O: Serialize + Debug>(&self, method: Method, con_id: &ConIdRs, msg: &O) {
        let name = method.as_str();
        // convert msg to str
        let json = to_string(msg).expect(format!("serde_json::to_string failed to convert msg: {:?}", msg).as_str());
        let con_id = ConId::from(con_id);
        fn py_callback(obj: &PyObject, name: &str, con_id: &ConId, json: &String) -> PyResult<()> {
            Python::with_gil(|py| {
                let json_module = PyModule::import(py, "json")?;
                let dict = json_module.getattr("loads")?.call1((json,))?.extract::<Py<PyDict>>()?;

                let args = (con_id.clone(), dict);
                let kwargs = None;
                obj.call_method(py, name, args, kwargs)?;
                Ok(())
            })
        }

        match py_callback(&self.0, name, &con_id, &json) {
            Ok(_) => {}
            Err(err) => {
                log::error!("{} failed '{}' on {} msg: {} err: {}", asserted_short_name!("PyProxyCallback", Self), name, con_id, json, err);
            }
        }
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

#[cfg(test)]
mod test {
    use links_core::unittest::setup;
    use log::info;
    use pyo3::{append_to_inittab, prepare_freethreaded_python};

    use crate::ouch_connect_nonblocking;

    use super::*;

    #[test]
    fn test_con_id() {
        setup::log::configure();
        append_to_inittab!(ouch_connect_nonblocking);
        prepare_freethreaded_python();

        let code = r#"
from ouch_connect_nonblocking_python import *;
# con_id = ConId("initiator", "name", "local", "peer")
con_ty = ConType.Initiator
print(con_ty)
        "#;

        let con_id_rs = ConIdRs::clt(Some("test"), None, "127.0.0.1:80");
        let con_id = ConId::from(con_id_rs.clone());
        info!("{:?}", con_id);
        assert_eq!(con_id.local, "pending");
        assert_eq!(con_id.peer, con_id_rs.get_peer().unwrap().to_string());
        Python::with_gil(|py| Python::run(py, code, None, None)).unwrap();
    }
}
