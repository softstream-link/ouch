use links_bindings_python::prelude::*;
use links_nonblocking::prelude::*;
use ouch_connect_nonblocking::prelude::*;
use pyo3::prelude::*;
use std::{num::NonZeroUsize, time::Duration};

create_callback_for_messenger!(SvcOuchProtocolManual, SvcOuchProtocolManualCallback);
create_svc_sender!(SvcManual, SvcOuchSender, SvcOuchProtocolManual, SvcOuchProtocolManualCallback);

#[pymethods]
impl SvcManual {
    #[new]
    fn new(_py: Python<'_>, host: &str, callback: PyObject, max_connections: Option<NonZeroUsize>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
        let max_connections = max_connections.unwrap_or(NonZeroUsize::new(1).unwrap());
        let callback = SvcOuchProtocolManualCallback::new_ref(callback);
        let protocol = SvcOuchProtocolManual::default();
        let sender = _py.allow_threads(move || SvcOuch::bind(host, max_connections, callback, protocol, name).unwrap().into_sender_with_spawned_recver());
        Self { sender, io_timeout }
    }
    #[classattr]
    fn msg_samples() -> Vec<String> {
        ouch_connect_nonblocking::prelude::svc_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>()
    }
}

create_callback_for_messenger!(SvcOuchProtocolAuto, SvcOuchProtocolAutoCallback);
create_svc_sender!(SvcAuto, SvcOuchSenderRef, SvcOuchProtocolAuto, SvcOuchProtocolAutoCallback);

#[pymethods]
impl SvcAuto {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(_py: Python<'_>, host: &str, callback: PyObject, usr: &str, pwd: &str, session: &str, clt_max_hbeat_interval: f64, svc_max_hbeat_interval: f64, max_connections: Option<NonZeroUsize>, io_timeout: Option<f64>, name: Option<&str>) -> Self {
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
        let sender = _py.allow_threads(move || SvcOuch::bind(host, max_connections, callback, protocol, name).unwrap().into_sender_with_spawned_recver_ref());
        Self { sender, io_timeout }
    }

    #[classattr]
    fn msg_samples() -> Vec<String> {
        ouch_connect_nonblocking::prelude::svc_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>()
    }
}
