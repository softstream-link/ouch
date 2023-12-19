use crate::callback::PyProxyCallback;
use crate::core::timeout_selector;
use crate::py_dict_2_json;
use ouch_connect_nonblocking::prelude::{asserted_short_name, ConnectionId, ConnectionStatus, PoolConnectionStatus, SendNonBlocking, SendStatus, SvcOuch as SvcOuchRs, SvcOuchProtocolManual, SvcOuchSender};
use pyo3::{prelude::*, types::PyDict};
use std::io::Error;
use std::io::ErrorKind;
use std::num::NonZeroUsize;

#[pyclass]
pub struct SvcManual {
    sender: SvcOuchSender<SvcOuchProtocolManual, PyProxyCallback>,
    timeout: Option<f64>,
}

#[pymethods]
impl SvcManual {
    #[new]
    fn new(host: String, callback: PyObject, max_connections: Option<NonZeroUsize>, timeout: Option<f64>, name: Option<&str>) -> Self {
        let max_connections = max_connections.unwrap_or(NonZeroUsize::new(1).unwrap());
        let callback = PyProxyCallback::new_ref(callback);
        let protocol = SvcOuchProtocolManual::default();
        let sender = SvcOuchRs::bind(host.as_str(), max_connections, callback, protocol, name).unwrap().into_sender_with_spawned_recver();
        Self { sender, timeout }
    }
    fn __repr__(&mut self) -> String {
        let is_connected = self.sender.is_next_connected();
        if !is_connected {
            return format!("{}({}, is_connected: {})", asserted_short_name!("SvcManual", Self), self.sender.con_id(), is_connected);
        } else {
            let num = self.sender.len();
            let max = self.sender.max_connections();
            let connections = self.sender.iter().map(|(_, s)| format!("[{}, is_connected: {}]", s.con_id(), s.is_connected())).collect::<Vec<_>>().join(",");

            format!("{}(#{} of max {} {})", asserted_short_name!("SvcManual", Self), num, max, connections)
        }
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
    fn is_connected(&mut self, timeout: Option<f64>) -> bool {
        let timeout = timeout_selector(timeout, self.timeout);
        self.sender.is_next_connected_busywait_timeout(timeout)
    }
}
