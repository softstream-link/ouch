use crate::callbacks::PythonProxyCallback;
use crate::core::{timeout_selector, RecvStatus, SendStatus};
use crate::POLL_HANDLER;
use crate::{dict_2_json, json_2_dict};
use ouch_connect_nonblocking::prelude::{asserted_short_name, RecvNonBlocking, SendNonBlocking};
use ouch_connect_nonblocking::prelude::{CltOuchSender as CltOuchSenderRs, CltOuchSupervised as CltOuchSupervisedRs, RecvStatus as RecvStatusRs};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct CltOuchSender {
    sender: CltOuchSenderRs<PythonProxyCallback>,
    timeout: Option<f64>,
}

#[pyclass]
pub struct CltOuchSupervised {
    clt: Option<CltOuchSupervisedRs<PythonProxyCallback>>,
    timeout: Option<f64>,
}

#[pymethods]
impl CltOuchSupervised {
    #[new]
    #[pyo3(signature = (host, callback, timeout = None, name = asserted_short_name!("CltOuchSupervised", Self)))]
    fn new(host: String, callback: PyObject, timeout: Option<f64>, name: Option<&str>) -> Self {
        let callback = PythonProxyCallback::new_ref(callback);
        let connect_timeout = timeout_selector(timeout, Some(1.0));
        let retry_after = connect_timeout / 10;

        let clt = CltOuchSupervisedRs::connect(host.as_str(), connect_timeout, retry_after, callback, name).unwrap();
        // let (recver, sender) = clt.into_split();
        // (*POLL_HANDLER).add_recver(recver.into());
        // // yield to poll thread hack so that recver is ready
        // Python::with_gil(|py| py.run("sleep(0.0001)", None, None)).unwrap();

        Self { clt: Some(clt), timeout }
    }
    // fn new(host: String, callback: PyObject, timeout: Option<f64>, name: Option<&str>) -> Self {
    //     let callback = PythonProxyCallback::new_ref(callback);
    //     let connect_timeout = timeout_selector(timeout, Some(1.0));
    //     let retry_after = connect_timeout / 10;

    //     let clt = CltOuchSupervisedRs::connect(host.as_str(), connect_timeout, retry_after, callback, name).unwrap();
    //     let (recver, sender) = clt.into_split();
    //     (*POLL_HANDLER).add_recver(recver.into());
    //     // yield to poll thread hack so that recver is ready
    //     Python::with_gil(|py| py.run("sleep(0.0001)", None, None)).unwrap();

    //     Self { sender, timeout }
    // }
    fn __repr__(&self) -> String {
        self.clt.as_ref().expect("CltOuchSupervised moved").to_string()
    }
    #[pyo3(signature = (msg, timeout = None))]
    fn send(&mut self, msg: Py<PyDict>, timeout: Option<f64>) -> PyResult<SendStatus> {
        let json = dict_2_json(msg);
        let mut msg = serde_json::from_str(json.as_str()).unwrap();
        let timeout = timeout_selector(timeout, self.timeout);
        Ok(self.clt.as_mut().expect("CltOuchSupervised moved").send_busywait_timeout(&mut msg, timeout)?.into())
    }
    #[pyo3(signature = (timeout = None))]
    fn recv(&mut self, timeout: Option<f64>) -> PyResult<RecvStatus> {
        let timeout = timeout_selector(timeout, self.timeout);
        let status = self.clt.as_mut().expect("CltOuchSupervised moved").recv_busywait_timeout(timeout)?;
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

#[cfg(test)]
mod test {

    use crate::ouch_connect_nonblocking_python;
    use pyo3::{append_to_inittab, prepare_freethreaded_python, Python};

    #[test]
    fn test_clt() {
        append_to_inittab!(ouch_connect_nonblocking_python);
        prepare_freethreaded_python();

        let code = format!(
            r#"
import logging; logging.basicConfig(level=logging.DEBUG)
from {crate_name} import *

clt = CltOuchSupervised("127.0.0.1:8080", , "test")
        "#,
            crate_name = stringify!(ouch_connect_nonblocking_python)
        );

        Python::with_gil(|py| Python::run(py, code.as_str(), None, None)).unwrap();
    }
}
