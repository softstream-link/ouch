use crate::callback::PyProxyCallback;
use crate::core::timeout_selector;
use ouch_connect_nonblocking::prelude::{asserted_short_name, CltOuchProtocolAuto, CltOuchProtocolManual, CltOuchSender, CltOuchSenderRef, ConnectionId, ConnectionStatus, Password, SendNonBlocking, SequenceNumber, SessionId, Shutdown, UserName};
use ouch_connect_nonblocking::prelude::{CltOuch as CltOuchRs, SendStatus};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::io::{Error, ErrorKind};
use std::time::Duration;

// TODO add context manager support
#[pyclass]
pub struct CltManual {
    sender: CltOuchSender<CltOuchProtocolManual, PyProxyCallback>,
    io_timeout: Option<f64>,
}
#[pymethods]
impl CltManual {
    fn __repr__(&self, _py: Python<'_>) -> String {
        _py.allow_threads(move || {
            let is_connected = self.sender.is_connected();
            format!("{}({}, is_connected: {})", asserted_short_name!("CltManual", Self), self.sender.con_id(), is_connected)
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
impl CltManual {
    #[new]
    fn new(_py: Python<'_>, host: String, callback: PyObject, connect_timeout: Option<f64>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let callback = PyProxyCallback::new_ref(callback);
        let connect_timeout = timeout_selector(connect_timeout, Some(1.0));
        let protocol = CltOuchProtocolManual::default();
        let sender = _py.allow_threads(move || CltOuchRs::connect(host.as_str(), connect_timeout, connect_timeout / 10, callback, protocol, name).unwrap().into_sender_with_spawned_recver());
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

    fn is_connected(&self, _py: Python<'_>, io_timeout: Option<f64>) -> bool {
        let io_timeout = timeout_selector(io_timeout, self.io_timeout);
        _py.allow_threads(move || self.sender.is_connected_busywait_timeout(io_timeout))
    }
}

#[pyclass]
pub struct CltAuto {
    sender: CltOuchSenderRef<CltOuchProtocolAuto, PyProxyCallback>,
    io_timeout: Option<f64>,
}
#[pymethods]
impl CltAuto {
    fn __repr__(&self, _py: Python<'_>) -> String {
        _py.allow_threads(move || {
            let is_connected = self.sender.is_connected();
            format!("{}({}, is_connected: {})", asserted_short_name!("CltAuto", Self), self.sender.con_id(), is_connected)
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
impl CltAuto {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        _py: Python<'_>,
        host: String,
        callback: PyObject,
        usr: &str,
        pwd: &str,
        session: &str,
        sequence: usize,
        clt_max_hbeat_interval: f64,
        svc_max_hbeat_interval: f64,
        connect_timeout: Option<f64>,
        io_timeout: Option<f64>,
        name: Option<&str>,
    ) -> Self {
        let callback = PyProxyCallback::new_ref(callback);
        let connect_timeout = timeout_selector(connect_timeout, Some(1.0));

        let protocol = CltOuchProtocolAuto::new(
            UserName::from(usr),
            Password::from(pwd),
            SessionId::from(session),
            SequenceNumber::from(sequence),
            io_timeout.map(Duration::from_secs_f64).unwrap_or(Duration::from_secs(0)),
            Duration::from_secs_f64(clt_max_hbeat_interval),
            Duration::from_secs_f64(svc_max_hbeat_interval),
        );

        let sender = _py.allow_threads(move || CltOuchRs::connect(host.as_str(), connect_timeout, connect_timeout / 10, callback, protocol, name).unwrap().into_sender_with_spawned_recver_ref());

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
    fn is_connected(&self, _py: Python<'_>, io_timeout: Option<f64>) -> bool {
        let io_timeout = timeout_selector(io_timeout, self.io_timeout);
        _py.allow_threads(move || self.sender.is_connected_busywait_timeout(io_timeout))
    }
}

// #[cfg(test)]
// mod test {
//     use crate::ouch_connect_nonblocking;
//     use pyo3::{append_to_inittab, prepare_freethreaded_python, Python};

//     #[test]
//     fn test_clt() {
//         append_to_inittab!(ouch_connect_nonblocking);
//         prepare_freethreaded_python();

//         let code = format!(
//             r#"
// import logging; logging.basicConfig(level=logging.DEBUG)
// from {crate_name} import *

// clt = CltOuchManual("127.0.0.1:8080", , "test")
//         "#,
//             crate_name = stringify!(ouch_connect_nonblocking_python)
//         );

//         Python::with_gil(|py| Python::run(py, code.as_str(), None, None)).unwrap();
//     }
// }
