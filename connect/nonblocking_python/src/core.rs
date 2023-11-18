use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ouch_connect_nonblocking::prelude::{ConId as ConIdRs, PoolAcceptStatus, SendStatus};
use pyo3::prelude::*;

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
pub enum Status {
    Ok,
    Busy,
}
// #[pymethods]
// impl Status {
//     // pub fn __hash__(&self) -> u64 {
//     //     match self {
//     //         Self::Ok => 0,
//     //         Self::Busy => 1,
//     //     }
//     // }
// }
impl From<SendStatus> for Status {
    fn from(status: SendStatus) -> Self {
        match status {
            SendStatus::Completed => Self::Ok,
            SendStatus::WouldBlock => Self::Busy,
        }
    }
}
impl From<PoolAcceptStatus> for Status {
    fn from(value: PoolAcceptStatus) -> Self {
        match value {
            PoolAcceptStatus::Accepted => Self::Ok,
            PoolAcceptStatus::WouldBlock => Self::Busy,
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

// #[pymodule]
// fn ouch_connect_nonblocking_python(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<ConId>()?;
//     Ok(())
// }

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
