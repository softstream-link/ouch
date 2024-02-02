use links_bindings_python::prelude::*;
use links_nonblocking::prelude::*;
use log::info;
use ouch_connect_nonblocking::prelude::*;
use pyo3::{prelude::*, types::PyDict};
use serde::Serialize;
use std::time::Duration;

use crate::{DEFAULT_CONNECT_TIMEOUT, DEFAULT_IO_TIMEOUT, DEFAULT_MAX_HBEAT_INTERVAL, DEFAULT_RETRY_CONNECT_AFTER, DEFAULT_USR_PWD};

create_callback_for_messenger!(CltOuchProtocolManual, CltOuchProtocolManualCallback);
create_clt_sender!(CltManual, CltOuchSender, CltOuchProtocolManual, CltOuchProtocolManualCallback, "ouch_connect");

#[derive(Serialize)]
struct CltManualConfig {
    pub connect_timeout: f64,
    pub retry_connect_after: f64,
    pub io_timeout: f64,
    pub name: String,
}
impl Default for CltManualConfig {
    fn default() -> Self {
        Self {
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            retry_connect_after: DEFAULT_RETRY_CONNECT_AFTER,
            io_timeout: DEFAULT_IO_TIMEOUT,
            name: asserted_short_name!("CltManual", CltManual).to_owned(),
        }
    }
}
impl From<&PyDict> for CltManualConfig {
    fn from(py_dict: &PyDict) -> Self {
        let default = Self::default();
        let connect_timeout = py_dict.get_item("connect_timeout").unwrap().map_or(default.connect_timeout, |any| any.extract::<f64>().unwrap());
        let retry_connect_after = py_dict.get_item("retry_connect_after").unwrap().map_or(default.retry_connect_after, |any| any.extract::<f64>().unwrap());
        let io_timeout = py_dict.get_item("io_timeout").unwrap().map_or(default.io_timeout, |any| any.extract::<f64>().unwrap());
        let name = py_dict.get_item("name").unwrap().map_or(default.name, |any| any.extract::<String>().unwrap());
        Self {
            connect_timeout,
            retry_connect_after,
            io_timeout,
            name,
        }
    }
}

#[pymethods]
impl CltManual {
    #[new]
    #[pyo3(signature = (host, callback, **kwargs))]
    fn new(_py: Python<'_>, host: &str, callback: PyObject, kwargs: Option<&PyDict>) -> PyResult<Py<Self>> {
        let config = kwargs.map_or(CltManualConfig::default(), CltManualConfig::from);
        info!("{}: effective config: {} with kwargs: {:?}", asserted_short_name!("CltManual", Self), serde_json::to_string(&config).unwrap(), kwargs);
        let sender = {
            let callback = callback.clone();
            let sender = _py
                .allow_threads(move || {
                    let callback = CltOuchProtocolManualCallback::new_ref(callback);
                    let protocol = CltOuchProtocolManual::default();
                    CltOuch::connect(
                        host,
                        Duration::from_secs_f64(config.connect_timeout),
                        Duration::from_secs_f64(config.retry_connect_after),
                        callback,
                        protocol,
                        Some(config.name.as_str()),
                    )
                })?
                .into_sender_with_spawned_recver();
            Py::new(_py, Self { sender, io_timeout: Some(config.io_timeout) })?
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

#[derive(Serialize)]
struct CltAutoConfig {
    pub username: String,
    pub password: String,
    pub session_id: String,
    pub sequence: usize,
    pub clt_max_hbeat_interval: f64,
    pub svc_max_hbeat_interval: f64,
    pub connect_timeout: f64,
    pub retry_connect_after: f64,
    pub io_timeout: f64,
    pub name: String,
}
impl Default for CltAutoConfig {
    fn default() -> Self {
        Self {
            username: DEFAULT_USR_PWD.to_owned(),
            password: DEFAULT_USR_PWD.to_owned(),
            session_id: "".to_owned(),
            sequence: 0,
            clt_max_hbeat_interval: DEFAULT_MAX_HBEAT_INTERVAL,
            svc_max_hbeat_interval: DEFAULT_MAX_HBEAT_INTERVAL,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            retry_connect_after: DEFAULT_RETRY_CONNECT_AFTER,
            io_timeout: DEFAULT_IO_TIMEOUT,
            name: asserted_short_name!("CltAuto", CltAuto).to_owned(),
        }
    }
}
impl From<&PyDict> for CltAutoConfig {
    fn from(value: &PyDict) -> Self {
        let default = Self::default();
        let username = value.get_item("username").unwrap().map_or(default.username, |any| any.extract::<String>().unwrap());
        let password = value.get_item("password").unwrap().map_or(default.password, |any| any.extract::<String>().unwrap());
        let session_id = value.get_item("session_id").unwrap().map_or(default.session_id, |any| any.extract::<String>().unwrap());
        let sequence = value.get_item("sequence").unwrap().map_or(default.sequence, |any| any.extract::<usize>().unwrap());
        let clt_max_hbeat_interval = value.get_item("clt_max_hbeat_interval").unwrap().map_or(default.clt_max_hbeat_interval, |any| any.extract::<f64>().unwrap());
        let svc_max_hbeat_interval = value.get_item("svc_max_hbeat_interval").unwrap().map_or(default.svc_max_hbeat_interval, |any| any.extract::<f64>().unwrap());
        let connect_timeout = value.get_item("connect_timeout").unwrap().map_or(default.connect_timeout, |any| any.extract::<f64>().unwrap());
        let retry_connect_after = value.get_item("retry_connect_after").unwrap().map_or(default.retry_connect_after, |any| any.extract::<f64>().unwrap());
        let io_timeout = value.get_item("io_timeout").unwrap().map_or(default.io_timeout, |any| any.extract::<f64>().unwrap());
        let name = value.get_item("name").unwrap().map_or(default.name, |any| any.extract::<String>().unwrap());
        Self {
            username,
            password,
            session_id,
            sequence,
            clt_max_hbeat_interval,
            svc_max_hbeat_interval,
            connect_timeout,
            retry_connect_after,
            io_timeout,
            name,
        }
    }
}

#[pymethods]
impl CltAuto {
    #[new]
    #[pyo3(signature = (host, callback, **kwargs))]
    fn new(_py: Python<'_>, host: &str, callback: PyObject, kwargs: Option<&PyDict>) -> PyResult<Py<Self>> {
        let config = kwargs.map_or(CltAutoConfig::default(), CltAutoConfig::from);
        info!("{}: effective config: {} with kwargs: {:?}", asserted_short_name!("CltAuto", Self), serde_json::to_string(&config).unwrap(), kwargs);

        let sender = {
            let callback = CltOuchProtocolAutoCallback::new_ref(callback.clone());

            let protocol = CltOuchProtocolAuto::new(
                UserName::from(config.username.as_str()),
                Password::from(config.password.as_str()),
                SessionId::from(config.session_id.as_str()),
                SequenceNumber::from(config.sequence),
                Duration::from_secs_f64(config.io_timeout),
                Duration::from_secs_f64(config.clt_max_hbeat_interval),
                Duration::from_secs_f64(config.svc_max_hbeat_interval),
            );

            let sender = _py
                .allow_threads(move || {
                    CltOuch::connect(
                        host,
                        Duration::from_secs_f64(config.connect_timeout),
                        Duration::from_secs_f64(config.retry_connect_after),
                        callback,
                        protocol,
                        Some(config.name.as_str()),
                    )
                })?
                .into_sender_with_spawned_recver_ref();
            Py::new(_py, Self { sender, io_timeout: Some(config.io_timeout) })?
        };

        patch_callback_if_settable_sender!(_py, sender, callback, asserted_short_name!("CltAuto", Self));
        Ok(sender)
    }
    #[classattr]
    fn msg_samples() -> Vec<String> {
        ouch_connect_nonblocking::prelude::clt_ouch_default_msgs().iter().map(|m| serde_json::to_string(m).unwrap()).collect::<Vec<_>>()
    }
}
