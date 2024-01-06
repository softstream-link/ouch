use crate::callback::PyProxyCallback;
use crate::core::timeout_selector;
use ouch_connect_nonblocking::prelude::{
    asserted_short_name, ConnectionId, ConnectionStatus, PoolConnectionStatus, SendNonBlocking, SendStatus, Shutdown, SvcOuch as SvcOuchRs, SvcOuchProtocolAuto, SvcOuchProtocolManual, SvcOuchSender, SvcOuchSenderRef, UserName,
};
use ouch_connect_nonblocking::prelude::{Password, SessionId};
use pyo3::{prelude::*, types::PyDict};
use std::io::Error;
use std::io::ErrorKind;
use std::num::NonZeroUsize;
use std::time::Duration;

#[pyclass]
pub struct SvcManual {
    sender: SvcOuchSender<SvcOuchProtocolManual, PyProxyCallback>,
    io_timeout: Option<f64>,
}
#[pymethods]
impl SvcManual {
    fn __repr__(&mut self, _py: Python<'_>) -> String {
        _py.allow_threads(move || {
            let is_connected = self.sender.is_next_connected();
            if !is_connected {
                format!("{}({}, is_connected: {})", asserted_short_name!("SvcManual", Self), self.sender.con_id(), is_connected)
            } else {
                let num = self.sender.len();
                let max = self.sender.max_connections();
                let connections = self.sender.iter().map(|(_, s)| format!("[{}, is_connected: {}]", s.con_id(), s.is_connected())).collect::<Vec<_>>().join(",");

                format!("{}(#{} of max {} {})", asserted_short_name!("SvcManual", Self), num, max, connections)
            }
        })
    }
    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }
    fn __exit__(&mut self, _py: Python<'_>, _exc_type: Option<&PyAny>, _exc_value: Option<&PyAny>, _traceback: Option<&PyAny>) {
        self.sender.shutdown()
    }
    fn __del__(&mut self) {
        self.sender.shutdown()
    }
}
#[pymethods]
impl SvcManual {
    #[new]
    fn new(_py: Python<'_>, host: String, callback: PyObject, max_connections: Option<NonZeroUsize>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let max_connections = max_connections.unwrap_or(NonZeroUsize::new(1).unwrap());
        let callback = PyProxyCallback::new_ref(callback);
        let protocol = SvcOuchProtocolManual::default();
        let sender = _py.allow_threads(move || SvcOuchRs::bind(host.as_str(), max_connections, callback, protocol, name).unwrap().into_sender_with_spawned_recver());
        Self { sender, io_timeout }
    }

    fn send(&mut self, _py: Python<'_>, msg: Py<PyDict>, io_timeout: Option<f64>) -> PyResult<()> {
        let io_timeout = timeout_selector(io_timeout, self.io_timeout);
        let json_module = PyModule::import(_py, "json")?;
        let json: String = json_module.getattr("dumps")?.call1((msg,))?.extract()?;
        let mut msg = serde_json::from_str(json.as_str()).unwrap();

        _py.allow_threads(move || match self.sender.send_busywait_timeout(&mut msg, io_timeout)? {
            SendStatus::Completed => Ok(()),
            SendStatus::WouldBlock => Err(Error::new(ErrorKind::WouldBlock, format!("Message not delivered due timeout: {:?}, msg: {}", io_timeout, json)).into()),
        })
    }
    fn is_connected(&mut self, _py: Python<'_>, io_timeout: Option<f64>) -> bool {
        let io_timeout = timeout_selector(io_timeout, self.io_timeout);
        _py.allow_threads(move || self.sender.is_next_connected_busywait_timeout(io_timeout))
    }
}

#[pyclass]
pub struct SvcAuto {
    sender: SvcOuchSenderRef<SvcOuchProtocolAuto, PyProxyCallback>,
    io_timeout: Option<f64>,
}
#[pymethods]
impl SvcAuto {
    fn __repr__(&mut self, _py: Python<'_>) -> String {
        _py.allow_threads(move || {
            let is_connected = self.sender.is_next_connected();
            if !is_connected {
                format!("{}({}, is_connected: {})", asserted_short_name!("SvcAuto", Self), self.sender.con_id(), is_connected)
            } else {
                let num = self.sender.len();
                let max = self.sender.max_connections();
                let connections = self.sender.iter().map(|(_, s)| format!("[{}, is_connected: {}]", s.con_id(), s.is_connected())).collect::<Vec<_>>().join(",");

                format!("{}(#{} of max {} {})", asserted_short_name!("SvcAuto", Self), num, max, connections)
            }
        })
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }
    fn __exit__(&mut self, _py: Python<'_>, _exc_type: Option<&PyAny>, _exc_value: Option<&PyAny>, _traceback: Option<&PyAny>) {
        self.sender.shutdown()
    }
    fn __del__(&mut self) {
        self.sender.shutdown()
    }
}
#[pymethods]
impl SvcAuto {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(_py: Python<'_>, host: String, callback: PyObject, usr: &str, pwd: &str, session: &str, clt_max_hbeat_interval: f64, svc_max_hbeat_interval: f64, max_connections: Option<NonZeroUsize>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let max_connections = max_connections.unwrap_or(NonZeroUsize::new(1).unwrap());
        let callback = PyProxyCallback::new_ref(callback);
        let protocol = SvcOuchProtocolAuto::new(
            UserName::from(usr),
            Password::from(pwd),
            SessionId::from(session),
            io_timeout.map(Duration::from_secs_f64).unwrap_or(Duration::from_secs(0)),
            Duration::from_secs_f64(clt_max_hbeat_interval),
            Duration::from_secs_f64(svc_max_hbeat_interval),
        );
        let sender = _py.allow_threads(move || SvcOuchRs::bind(host.as_str(), max_connections, callback, protocol, name).unwrap().into_sender_with_spawned_recver_ref());
        Self { sender, io_timeout }
    }

    fn send(&mut self, _py: Python<'_>, msg: Py<PyDict>, io_timeout: Option<f64>) -> PyResult<()> {
        let io_timeout = timeout_selector(io_timeout, self.io_timeout);
        let json_module = PyModule::import(_py, "json")?;
        let json: String = json_module.getattr("dumps")?.call1((msg,))?.extract()?;
        let mut msg = serde_json::from_str(json.as_str()).unwrap();

        _py.allow_threads(move || match self.sender.send_busywait_timeout(&mut msg, io_timeout)? {
            SendStatus::Completed => Ok(()),
            SendStatus::WouldBlock => Err(Error::new(ErrorKind::WouldBlock, format!("Message not delivered due timeout: {:?}, msg: {}", io_timeout, json)).into()),
        })
    }
    fn is_connected(&mut self, _py: Python<'_>, io_timeout: Option<f64>) -> bool {
        let io_timeout = timeout_selector(io_timeout, self.io_timeout);
        _py.allow_threads(move || self.sender.is_next_connected_busywait_timeout(io_timeout))
    }
}
