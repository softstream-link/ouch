use links_bindings_python::prelude::*;
use links_nonblocking::prelude::*;
use ouch_connect_nonblocking::prelude::*;
use pyo3::prelude::*;
use std::time::Duration;

create_callback_for_messenger!(CltOuchProtocolManual, CltOuchProtocolManualCallback);
create_clt_sender!(CltManual, CltOuchSender, CltOuchProtocolManual, CltOuchProtocolManualCallback, "ouch_connect");

#[pymethods]
impl CltManual {
    #[new]
    #[pyo3(signature = (host, callback, connect_timeout = 1.0, io_timeout = 0.5, name = None))]
    fn new(_py: Python<'_>, host: &str, callback: PyObject, connect_timeout: f64, io_timeout: f64, name: Option<&str>) -> PyResult<Py<Self>> {
        let sender = {
            let callback = CltOuchProtocolManualCallback::new_ref(callback.clone());
            let connect_timeout = Duration::from_secs_f64(connect_timeout);
            let protocol = CltOuchProtocolManual::default();
            let sender = _py.allow_threads(move || CltOuch::connect(host, connect_timeout, connect_timeout / 10, callback, protocol, name))?.into_sender_with_spawned_recver();
            Py::new(_py, Self { sender, io_timeout: Some(io_timeout) })?
        };
        patch_callback_if_settable_sender!(_py, sender, callback, asserted_short_name!("CltManual", Self));
        Ok(sender)
    }
    #[classattr]
    fn msg_samples() -> Vec<String> {
        ouch_connect_nonblocking::prelude::clt_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>()
    }
}

create_callback_for_messenger!(CltOuchProtocolAuto, CltOuchProtocolAutoCallback);
create_clt_sender!(CltAuto, CltOuchSenderRef, CltOuchProtocolAuto, CltOuchProtocolAutoCallback, "ouch_connect");

#[pymethods]
impl CltAuto {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        _py: Python<'_>,
        host: &str,
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
    ) -> PyResult<Py<Self>> {
        let sender = {
            let callback = CltOuchProtocolAutoCallback::new_ref(callback.clone());
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

            let sender = _py
                .allow_threads(move || CltOuch::connect(host, connect_timeout, connect_timeout / 10, callback, protocol, name))?
                .into_sender_with_spawned_recver_ref();
            Py::new(_py, Self { sender, io_timeout })?
        };

        patch_callback_if_settable_sender!(_py, sender, callback, asserted_short_name!("CltAuto", Self));
        Ok(sender)
    }
    #[classattr]
    fn msg_samples() -> Vec<String> {
        ouch_connect_nonblocking::prelude::clt_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>()
    }
}
