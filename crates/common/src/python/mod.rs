// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Python bindings from [PyO3](https://pyo3.rs).

pub mod clock;
pub mod custom;
pub mod enums;
pub mod handler;
pub mod listener;
pub mod logging;
pub mod msgbus;
pub mod signal;
pub mod timer;
pub mod xrate;

use std::rc::Rc;

use handler::PythonMessageHandler;
use indexmap::IndexMap;
use nautilus_core::{UUID4, UnixNanos};
use nautilus_model::{data::DataType, identifiers::ClientId};
use pyo3::prelude::*;

use crate::{
    messages::data::{DataCommand, RequestCommand, RequestCustomData},
    msgbus::{
        get_message_bus, handler::{MessageHandler, ShareableMessageHandler}, register, register_response_handler, MStr
    },
};

#[pyfunction(name = "register_response_handler")]
pub fn py_register_response_handler(correlation_id: &str, handler: PyObject) {
    let handler = PythonMessageHandler::new(correlation_id, handler);
    let handler = ShareableMessageHandler::from(Rc::new(handler) as Rc<dyn MessageHandler>);
    let correlation_id = UUID4::from(correlation_id);
    register_response_handler(&correlation_id, handler);
}

#[pymethods]
impl RequestCustomData {
    /// Creates a new [`RequestCustomData`] instance.
    #[new]
    pub fn py_new(
        client_id: &str,
        data_type: DataType,
        request_id: &str,
        ts_init: u64,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        let client_id = ClientId::new(client_id);
        let request_id = UUID4::from(request_id);
        let ts_init = UnixNanos::new(ts_init);
        Self {
            client_id,
            data_type,
            request_id,
            ts_init,
            params,
        }
    }
}

#[pyfunction]
/// Sends the `message` to the `endpoint`.
pub fn send_request(endpoint: &str, message: RequestCustomData) {
    // TODO: This should return a Result (in case endpoint doesn't exist)
    let endpoint = MStr::from(endpoint);
    let cmd = DataCommand::Request(RequestCommand::Data(message));
    let handler = get_message_bus().borrow().get_endpoint(endpoint).cloned();
    if let Some(handler) = handler {
        handler.0.handle(&cmd);
    }
}

#[pyfunction(name = "register")]
pub fn py_register(endpoint: &str, handler: PyObject) {
    let endpoint = MStr::from(endpoint);
    let handler = PythonMessageHandler::new(&endpoint, handler);
    let handler = ShareableMessageHandler::from(Rc::new(handler) as Rc<dyn MessageHandler>);
    register(endpoint, handler);
}

/// Loaded as nautilus_pyo3.common
///
/// # Errors
///
/// Returns a `PyErr` if registering any module components fails.
#[pymodule]
pub fn common(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::custom::CustomData>()?;
    m.add_class::<crate::messages::data::RequestCustomData>()?;
    m.add_class::<crate::signal::Signal>()?;
    m.add_class::<crate::python::clock::TestClock_Py>()?;
    m.add_class::<crate::python::clock::LiveClock_Py>()?;
    m.add_class::<crate::msgbus::BusMessage>()?;
    m.add_class::<crate::msgbus::MessageBus>()?;
    m.add_class::<crate::msgbus::listener::MessageBusListener>()?;
    m.add_class::<crate::python::handler::PythonMessageHandler>()?;
    m.add_class::<crate::enums::ComponentState>()?;
    m.add_class::<crate::enums::ComponentTrigger>()?;
    m.add_class::<crate::enums::LogColor>()?;
    m.add_class::<crate::enums::LogLevel>()?;
    m.add_class::<crate::enums::LogFormat>()?;
    m.add_class::<crate::logging::logger::LoggerConfig>()?;
    m.add_class::<crate::logging::logger::LogGuard>()?;
    m.add_class::<crate::logging::writer::FileWriterConfig>()?;
    m.add_function(wrap_pyfunction!(py_register_response_handler, m)?)?;
    m.add_function(wrap_pyfunction!(send_request, m)?)?;
    m.add_function(wrap_pyfunction!(py_register, m)?)?;
    m.add_function(wrap_pyfunction!(logging::py_init_tracing, m)?)?;
    m.add_function(wrap_pyfunction!(logging::py_init_logging, m)?)?;
    m.add_function(wrap_pyfunction!(logging::py_logger_flush, m)?)?;
    m.add_function(wrap_pyfunction!(logging::py_logger_log, m)?)?;
    m.add_function(wrap_pyfunction!(logging::py_log_header, m)?)?;
    m.add_function(wrap_pyfunction!(logging::py_log_sysinfo, m)?)?;
    m.add_function(wrap_pyfunction!(
        logging::py_logging_clock_set_static_mode,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        logging::py_logging_clock_set_realtime_mode,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        logging::py_logging_clock_set_static_time,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(xrate::py_get_exchange_rate, m)?)?;

    Ok(())
}
