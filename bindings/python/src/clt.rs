use links_bindings_python::prelude::*;
use ouch_connect_nonblocking::prelude::CltOuch as CltOuchRs;
use ouch_connect_nonblocking::prelude::{CltOuchProtocolAuto, CltOuchProtocolManual, CltOuchSender, CltOuchSenderRef, ConnectionId, ConnectionStatus, Password, SendNonBlocking, SendStatus, SequenceNumber, SessionId, Shutdown, UserName};
use pyo3::prelude::*;
use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

create_callback_for_messenger!(CltOuchProtocolManualCallback, CltOuchProtocolManual);
create_clt_sender!(CltManual, CltOuchSender, CltOuchProtocolManual, CltOuchProtocolManualCallback);

#[pymethods]
impl CltManual {
    #[new]
    fn new(_py: Python<'_>, host: String, callback: PyObject, connect_timeout: Option<f64>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let callback = CltOuchProtocolManualCallback::new_ref(callback);
        let connect_timeout = timeout_selector(connect_timeout, Some(1.0));
        let protocol = CltOuchProtocolManual::default();
        let sender = _py.allow_threads(move || CltOuchRs::connect(host.as_str(), connect_timeout, connect_timeout / 10, callback, protocol, name).unwrap().into_sender_with_spawned_recver());
        Self { sender, io_timeout }
    }
    #[classattr]
    fn msg_samples() -> String {
        let msgs = ouch_connect_nonblocking::prelude::clt_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>().join("\t\n");
        format!("Valid Json Messages:\n{}", msgs)
    }
}

create_callback_for_messenger!(CltOuchProtocolAutoCallback, CltOuchProtocolAuto);
create_clt_sender!(CltAuto, CltOuchSenderRef, CltOuchProtocolAuto, CltOuchProtocolAutoCallback);

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
        let callback = CltOuchProtocolAutoCallback::new_ref(callback);
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
    #[classattr]
    fn msg_samples() -> String {
        let msgs = ouch_connect_nonblocking::prelude::clt_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>().join("\t\n");
        format!("Valid Json Messages:\n{}", msgs)
    }
}
