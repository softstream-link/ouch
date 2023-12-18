use ouch_connect_nonblocking::prelude::{ConId as ConIdRs, SendStatus as SendStatusRs};
use pyo3::prelude::*;
use std::{fmt::Display, time::Duration};

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
impl Display for ConId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.con_type {
            ConType::Initiator => write!(f, "{}@{}->{}", self.name, self.local, self.peer),
            ConType::Acceptor => write!(f, "{}@{}<-{}", self.name, self.local, self.peer),
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

pub fn timeout_selector(priority_1: Option<f64>, priority_2: Option<f64>) -> Duration {
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
