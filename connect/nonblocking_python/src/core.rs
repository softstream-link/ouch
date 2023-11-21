use std::time::Duration;

use ouch_connect_nonblocking::prelude::{asserted_short_name, ConId as ConIdRs, PoolAcceptStatus, SendStatus as SendStatusRs};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
#[derive(Debug, Clone)]
pub enum ConType {
    Initiator,
    Acceptor,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ConId {
    pub con_type: ConType,
    pub name: String,
    pub local: String,
    pub peer: String,
}
#[pymethods]
impl ConId {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pyclass]
pub enum AcceptStatus {
    Ok,
    WouldBlock,
}
impl From<PoolAcceptStatus> for AcceptStatus {
    fn from(value: PoolAcceptStatus) -> Self {
        match value {
            PoolAcceptStatus::Accepted => Self::Ok,
            PoolAcceptStatus::WouldBlock => Self::WouldBlock,
        }
    }
}

#[pyclass]
pub enum SendStatus {
    Ok,
    WouldBlock,
}
impl From<SendStatusRs> for SendStatus {
    fn from(status: SendStatusRs) -> Self {
        match status {
            SendStatusRs::Completed => Self::Ok,
            SendStatusRs::WouldBlock => Self::WouldBlock,
        }
    }
}

#[pyclass]
pub struct RecvStatus(pub Option<Py<PyDict>>);
#[pymethods]
impl RecvStatus {
    pub fn __eq__(&self, other: &Self) -> bool {
        self.0.is_some() == other.0.is_some()
    }
    pub fn __repr__(&self) -> String {
        match &self.0 {
            None => format!("{}.Busy", asserted_short_name!("RecvStatus", Self)),
            Some(payload) => format!("{}.Ok({})", asserted_short_name!("RecvStatus", Self), payload),
        }
    }
    pub fn payload(&self) -> PyResult<Py<PyDict>> {
        match &self.0 {
            Some(msg_dict) => Ok(msg_dict.clone()), // clone is a ref counted
            None => Err(PyTypeError::new_err(format!("Payload only available when variant is {}.Ok", asserted_short_name!("RecvStatus", Self)))),
        }
    }
    #[classattr]
    #[allow(non_snake_case)]
    pub fn WouldBlock() -> Self {
        Self(None)
    }
    #[classattr]
    #[allow(non_snake_case)]
    pub fn Ok() -> Self {
        Self(Some(Python::with_gil(|py| PyDict::new(py).into())))
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

pub fn timeout_selector(priority_1: Option<f64>, priority_2: Option<f64> ) -> Duration {
    match priority_1 {
        Some(timeout) => Duration::from_secs_f64(timeout),
        None => match priority_2 {
            Some(timeout) => Duration::from_secs_f64(timeout),
            None => Duration::from_secs(0),
        },
    }
}

#[cfg(test)]
mod test {
    use links_core::unittest::setup;
    use log::info;
    use pyo3::{append_to_inittab, prepare_freethreaded_python};

    use crate::ouch_connect_nonblocking_python;

    // use crate::core::ouch_connect_nonblocking_python;

    use super::*;

    #[test]
    fn test_conid() {
        setup::log::configure();
        append_to_inittab!(ouch_connect_nonblocking_python);
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
