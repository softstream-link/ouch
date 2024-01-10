use links_bindings_python::prelude::*;
use ouch_connect_nonblocking::prelude::{
    ConnectionId, ConnectionStatus, Password, PoolConnectionStatus, SendNonBlocking, SendStatus, SessionId, Shutdown, SvcOuch as SvcOuchRs, SvcOuchProtocolAuto, SvcOuchProtocolManual, SvcOuchSender, SvcOuchSenderRef, UserName,
};
use pyo3::prelude::*;
use std::{
    io::{Error, ErrorKind},
    num::NonZeroUsize,
    time::Duration,
};
// TODO mvoe some where
// #[classattr]
// fn __doc__() -> String {
//     let msgs = ouch_connect_nonblocking::prelude::svc_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>().join("\t\n");
//     format!("Valid Json Messages:\n{}", msgs)
// }

create_callback_for_messenger!(SvcOuchProtocolManualCallback, SvcOuchProtocolManual);
create_svc_sender!(SvcManual, SvcOuchSender, SvcOuchProtocolManual, SvcOuchProtocolManualCallback);

#[pymethods]
impl SvcManual {
    #[new]
    fn new(_py: Python<'_>, host: String, callback: PyObject, max_connections: Option<NonZeroUsize>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let max_connections = max_connections.unwrap_or(NonZeroUsize::new(1).unwrap());
        let callback = SvcOuchProtocolManualCallback::new_ref(callback);
        let protocol = SvcOuchProtocolManual::default();
        let sender = _py.allow_threads(move || SvcOuchRs::bind(host.as_str(), max_connections, callback, protocol, name).unwrap().into_sender_with_spawned_recver());
        Self { sender, io_timeout }
    }
    #[classattr]
    fn msg_samples() -> String {
        let msgs = ouch_connect_nonblocking::prelude::svc_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>().join("\t\n");
        format!("Valid Json Messages:\n{}", msgs)
    }
}

create_callback_for_messenger!(SvcOuchProtocolAutoCallback, SvcOuchProtocolAuto);
create_svc_sender!(SvcAuto, SvcOuchSenderRef, SvcOuchProtocolAuto, SvcOuchProtocolAutoCallback);

#[pymethods]
impl SvcAuto {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(_py: Python<'_>, host: String, callback: PyObject, usr: &str, pwd: &str, session: &str, clt_max_hbeat_interval: f64, svc_max_hbeat_interval: f64, max_connections: Option<NonZeroUsize>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let max_connections = max_connections.unwrap_or(NonZeroUsize::new(1).unwrap());
        let callback = SvcOuchProtocolAutoCallback::new_ref(callback);
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

    #[classattr]
    fn msg_samples() -> String {
        let msgs = ouch_connect_nonblocking::prelude::svc_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>().join("\t\n");
        format!("Valid Json Messages:\n{}", msgs)
    }
}
