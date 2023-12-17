use crate::core::timeout_selector;
use crate::core::AcceptStatus;
use crate::core::RecvStatus;
use crate::dict_2_json;
use crate::json_2_dict;
use crate::POLL_HANDLER;
use crate::{callbacks::PythonProxyCallback, core::SendStatus};
use log::warn;
use ouch_connect_nonblocking::prelude::{asserted_short_name, PoolAcceptCltNonBlocking, RecvNonBlocking, SendNonBlocking, SvcOuchSendersPool};
use ouch_connect_nonblocking::prelude::{RecvStatus as RecvStatusRs, SvcOuchManual as SvcOuchSupervisedRs};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::io::ErrorKind;
use std::num::NonZeroUsize;

#[pyclass]
pub struct SvcOuchSender {
    sender: SvcOuchSendersPool<PythonProxyCallback>,
    timeout: Option<f64>,
}
#[pymethods]
impl SvcOuchSender {
    fn __repr__(&self) -> String {
        self.sender.to_string()
    }
    #[pyo3(signature = (msg, timeout = None))]
    fn send(&mut self, msg: Py<PyDict>, timeout: Option<f64>) -> PyResult<SendStatus> {
        let json = dict_2_json(msg);
        let mut msg = serde_json::from_str(json.as_str()).unwrap();
        let timeout = timeout_selector(timeout, self.timeout);
        Ok(self.sender.send_busywait_timeout(&mut msg, timeout)?.into())
    }
}

#[pyclass]
pub struct SvcOuchSupervised {
    svc: Option<SvcOuchSupervisedRs<PythonProxyCallback>>,
    timeout: Option<f64>,
}

#[pymethods]
impl SvcOuchSupervised {
    #[new]
    #[pyo3(signature = (host, callback, max_connections = NonZeroUsize::new(1).unwrap(), timeout = None, name = asserted_short_name!("SvcOuchSupervised", Self)))]
    fn new(host: String, callback: PyObject, max_connections: NonZeroUsize, timeout: Option<f64>, name: Option<&str>) -> Self {
        let callback = PythonProxyCallback::new_ref(callback);
        let svc = SvcOuchSupervisedRs::bind(host.as_str(), callback, max_connections, name).unwrap();

        Self { svc: Some(svc), timeout }
    }
    fn into_sender(&mut self) -> PyResult<SvcOuchSender> {
        let timeout = self.timeout;
        let (acceptor, recver, sender) = self.svc.take().expect("SvcOuchSupervised moved").into_split();
        (*POLL_HANDLER).add_acceptor(acceptor.into());
        
        // (*POLL_HANDLER).add_recver(recver.into());
        // yield_now();
        Python::with_gil(|py| py.run("sleep(0.0001)", None, None)).unwrap();
        Ok(SvcOuchSender { sender, timeout })
    }
    // #[pyo3(signature = (host, callback, max_connections = NonZeroUsize::new(1).unwrap(), timeout = None, name = asserted_short_name!("SvcOuchSupervised", Self)))]
    // fn new(host: String, callback: PyObject, max_connections: NonZeroUsize, timeout: Option<f64>, name: Option<&str>) -> Self {
    //     let callback = PythonProxyCallback::new_ref(callback);
    //     let svc = SvcOuchSupervisedRs::bind(host.as_str(), callback, max_connections, name).unwrap();
    //     let (acceptor, _, sender) = svc.into_split();
    //     (*POLL_HANDLER).add_acceptor(acceptor.into());
    //     // yield to poll thread hack so that recver is ready
    //     Python::with_gil(|py| py.run("sleep(0.0001)", None, None)).unwrap();

    //     Self { sender, timeout }
    // }

    fn __repr__(&self) -> String {
        self.svc.as_ref().expect("SvcOuchSupervised moved").to_string()
    }

    #[pyo3(signature = (msg, timeout = None))]
    fn send(&mut self, msg: Py<PyDict>, timeout: Option<f64>) -> PyResult<SendStatus> {
        self.pool_accept(timeout)?;
        let json = dict_2_json(msg);
        let mut msg = serde_json::from_str(json.as_str()).unwrap();
        let timeout = timeout_selector(timeout, self.timeout);
        Ok(self.svc.as_mut().expect("SvcOuchSupervised moved").send_busywait_timeout(&mut msg, timeout)?.into())
    }

    #[pyo3(signature = (timeout = None))]
    fn pool_accept(&mut self, timeout: Option<f64>) -> PyResult<AcceptStatus> {
        let timeout = timeout_selector(self.timeout, timeout);
        match self.svc.as_mut().expect("SvcOuchSupervised moved").pool_accept_busywait_timeout(timeout) {
            Ok(status) => Ok(status.into()),
            Err(e) if e.kind() == ErrorKind::OutOfMemory => {
                warn!("{}", e);
                Ok(AcceptStatus::WouldBlock)
            }
            Err(e) => Err(e.into()),
        }
    }
    #[pyo3(signature = (timeout = None))]
    fn recv(&mut self, timeout: Option<f64>) -> PyResult<RecvStatus> {
        self.pool_accept(timeout)?;
        let timeout = timeout_selector(timeout, self.timeout);
        let status = self.svc.as_mut().expect("SvcOuchSupervised moved").recv_busywait_timeout(timeout)?;
        match status {
            RecvStatusRs::Completed(Some(msg)) => {
                let json = serde_json::to_string(&msg).unwrap(); // TODO raise PyError
                Ok(RecvStatus(Some(json_2_dict(json.as_str()))))
            }
            RecvStatusRs::WouldBlock => Ok(RecvStatus(None)),
            RecvStatusRs::Completed(None) => Ok(RecvStatus(Some(Python::with_gil(|py| PyDict::new(py).into())))),
        }
    }
}
