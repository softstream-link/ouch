use std::io::{Error, ErrorKind};
use std::time::Duration;

use crate::callback::PyProxyCallback;
use crate::core::timeout_selector;

use crate::py_dict_2_json;

use ouch_connect_nonblocking::prelude::{asserted_short_name, CltOuchProtocolAuto, CltOuchProtocolManual, CltOuchSender, CltOuchSenderRef, ConnectionId, ConnectionStatus, SendNonBlocking};
use ouch_connect_nonblocking::prelude::{CltOuch as CltOuchRs, SendStatus};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct CltManual {
    sender: CltOuchSender<CltOuchProtocolManual, PyProxyCallback>,
    timeout: Option<f64>,
}

#[pymethods]
impl CltManual {
    #[new]
    fn new(host: String, callback: PyObject, timeout: Option<f64>, name: Option<&str>) -> Self {
        let callback = PyProxyCallback::new_ref(callback);
        let connect_timeout = timeout_selector(timeout, Some(1.0));
        let retry_after = connect_timeout / 10;
        let protocol = CltOuchProtocolManual::default();
        let sender = CltOuchRs::connect(host.as_str(), connect_timeout, retry_after, callback, protocol, name).unwrap().into_sender_with_spawned_recver();
        Self { sender, timeout }
    }
    fn __repr__(&self) -> String {
        let is_connected = self.sender.is_connected();
        format!("{}({}, is_connected: {})", asserted_short_name!("CltManual", Self), self.sender.con_id(), is_connected)
    }
    fn send(&mut self, msg: Py<PyDict>, timeout: Option<f64>) -> PyResult<()> {
        let timeout = timeout_selector(timeout, self.timeout);
        let json = py_dict_2_json(msg)?;
        let mut msg = serde_json::from_str(json.as_str()).unwrap();
        match self.sender.send_busywait_timeout(&mut msg, timeout)? {
            SendStatus::Completed => Ok(()),
            SendStatus::WouldBlock => Err(Error::new(ErrorKind::WouldBlock, format!("Message not delivered due timeout: {:?}, msg: {}", timeout, json)).into()),
        }
    }
    fn is_connected(&self, timeout: Option<f64>) -> bool {
        let timeout = timeout_selector(timeout, self.timeout);
        self.sender.is_connected_busywait_timeout(timeout)
    }
}

#[pyclass]
pub struct CltAuto {
    sender: CltOuchSenderRef<CltOuchProtocolAuto, PyProxyCallback>,
    timeout: Option<f64>,
}
#[pymethods]
impl CltAuto {
    #[new]
    #[pyo3(signature = (host, callback, usr, pwd, session, sequence, max_hbeat_interval_send, max_hbeat_interval_recv, timeout = None, name = asserted_short_name!("CltAuto", Self)))]
    fn new(host: String, callback: PyObject, usr: &str, pwd: &str, session: &str, sequence: &str, max_hbeat_interval_send: f64, max_hbeat_interval_recv: f64, timeout: Option<f64>, name: Option<&str>) -> Self {
        let callback = PyProxyCallback::new_ref(callback);
        let connect_timeout = timeout_selector(timeout, Some(1.0));
        let retry_after = connect_timeout / 10;
        let protocol = CltOuchProtocolAuto::new(
            usr.as_bytes().into(),
            pwd.as_bytes().into(),
            session.as_bytes().into(),
            sequence.as_bytes().into(),
            connect_timeout,
            Duration::from_secs_f64(max_hbeat_interval_send),
            Duration::from_secs_f64(max_hbeat_interval_recv),
        );

        let sender = CltOuchRs::connect(host.as_str(), connect_timeout, retry_after, callback, protocol, name).unwrap().into_sender_with_spawned_recver_ref();

        Self { sender, timeout }
    }
    fn __repr__(&self) -> String {
        format!("{}({})", asserted_short_name!("CltManual", Self), self.sender.con_id())
    }
    #[pyo3(signature = (msg, timeout = None))]
    fn send(&mut self, msg: Py<PyDict>, timeout: Option<f64>) -> PyResult<()> {
        let json = py_dict_2_json(msg)?;
        let mut msg = serde_json::from_str(json.as_str()).unwrap();
        let timeout = timeout_selector(timeout, self.timeout);
        match self.sender.send_busywait_timeout(&mut msg, timeout)? {
            SendStatus::Completed => Ok(()),
            SendStatus::WouldBlock => Err(Error::new(ErrorKind::WouldBlock, format!("Message not delivered due timeout: {:?}, msg: {}", timeout, json)).into()),
        }
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

clt = CltOuchManual("127.0.0.1:8080", , "test")
        "#,
            crate_name = stringify!(ouch_connect_nonblocking_python)
        );

        Python::with_gil(|py| Python::run(py, code.as_str(), None, None)).unwrap();
    }
}
