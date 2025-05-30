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

use std::{cell::RefCell, ffi::CString, net::SocketAddr, pin::Pin, rc::Rc};

use data_client::MockDataClient;
use futures::{Stream, stream::SelectAll};
use http_server::start_positive_stream_http_server;
use nautilus_common::{
    cache::Cache,
    clock::{Clock, LiveClock},
    messages::data::{DataCommand, DataResponse},
    msgbus::{
        self, get_message_bus,
        handler::{ShareableMessageHandler, TypedMessageHandler},
        register,
    },
    testing::init_logger_for_testing,
};
use nautilus_core::python::IntoPyObjectNautilusExt;
use nautilus_data::{
    client::{DataClient, DataClientAdapter},
    engine::DataEngine,
};
use nautilus_model::identifiers::Venue;
use pyo3::prelude::*;
use tokio_stream::StreamExt;
use websocket_server::NegativeStreamServer;

pub mod big_brain_actor;
pub mod data_client;
pub mod http_server;
pub mod websocket_server;

pub async fn init_data_engine(
    http_address: SocketAddr,
    websocket_address: SocketAddr,
) -> (
    Pin<Box<dyn Stream<Item = DataResponse>>>,
    Pin<Box<dyn Stream<Item = i32>>>,
) {
    let (client, http_stream, websocket_stream) =
        MockDataClient::start(http_address, websocket_address).await;
    let client: Box<dyn DataClient> = Box::new(client);
    let clock: Rc<RefCell<dyn Clock>> = Rc::new(RefCell::new(LiveClock::new()));

    let adapter = DataClientAdapter::new(
        client.client_id(),
        Some(Venue::from_str_unchecked("DEMO")),
        false,
        false,
        client,
    );
    let cache = Rc::new(RefCell::new(Cache::new(None, None)));

    let mut data_engine = DataEngine::new(clock, cache, None);
    data_engine.register_client(adapter, None);
    let data_engine = Rc::new(RefCell::new(data_engine));

    let data_engine_clone = data_engine;
    let handler = ShareableMessageHandler(Rc::new(TypedMessageHandler::from(
        move |cmd: &DataCommand| data_engine_clone.borrow_mut().execute(cmd),
    )));
    register("data_engine".into(), handler);

    (http_stream, websocket_stream)
}

#[derive(Default)]
pub struct LiveRunner {
    data_response_stream: SelectAll<Pin<Box<dyn Stream<Item = DataResponse>>>>,
    message_stream: SelectAll<Pin<Box<dyn Stream<Item = i32>>>>,
}

impl LiveRunner {
    pub fn new_add_data_response_stream(
        &mut self,
        stream: Pin<Box<dyn Stream<Item = DataResponse>>>,
    ) {
        self.data_response_stream.push(stream);
    }

    pub fn new_message_stream(&mut self, stream: Pin<Box<dyn Stream<Item = i32>>>) {
        self.message_stream.push(stream);
    }

    pub async fn run(&mut self) {
        let endpoint = "negative_stream".into();

        loop {
            // TODO: push decoding logic into data client
            tokio::select! {
                data_response = self.data_response_stream.next() => {
                    if let Some(DataResponse::Data(custom_data_response)) = data_response {
                            log::debug!("Received custom data response: {custom_data_response:?}");
                            let value = custom_data_response.data.downcast_ref::<i32>().copied().unwrap();
                            let value = Python::with_gil(|py| value.into_py_any_unwrap(py));
                            msgbus::response(&custom_data_response.correlation_id, &value);
                    }
                }
                message = self.message_stream.next() => {
                    if let Some(message) = message {
                        let value = Python::with_gil(|py| message.into_py_any_unwrap(py));
                        log::debug!("Received message: {message}");
                        msgbus::send(endpoint, &value);
                    }
                }
            }
        }
    }
}

async fn run_engine() {
    let http_address = start_positive_stream_http_server().await.unwrap();
    let websocket_server = NegativeStreamServer::setup().await;

    // Initialize data client with http and websocket streams
    let (http_stream, websocket_stream) =
        init_data_engine(http_address, websocket_server.address).await;

    let mut runner = LiveRunner::default();
    runner.new_add_data_response_stream(http_stream);
    runner.new_message_stream(websocket_stream);

    let msgbus = get_message_bus();
    println!(
        "Is registered: {}",
        msgbus.borrow().is_registered("negative_stream")
    );
    println!("endpoints: {:?}", msgbus.borrow().endpoints());
    println!(
        "handler: {:?}",
        msgbus.borrow().get_endpoint("negative_stream".into())
    );

    runner.run().await;
}

#[pyfunction]
pub fn main(file_path: String) {
    init_logger_for_testing(None).unwrap();
    let message_bus = get_message_bus();

    // Initialize big brain actor
    // let big_brain_actor = BigBrainActor::new();
    // let big_brain_actor = Rc::new(UnsafeCell::new(big_brain_actor));
    // register_actor(big_brain_actor);
    // BigBrainActor::register_message_handlers();

    // Initialize python actor
    let code = std::fs::read_to_string(file_path).unwrap();
    let code = CString::new(code).unwrap();
    let filename = CString::new("big_brain_actor".to_string()).unwrap();
    let module = CString::new("big_brain_actor".to_string()).unwrap();

    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let pymod = PyModule::from_code(py, &code, &filename, &module).unwrap();

        let test_class = pymod.getattr("BigBrainActor").unwrap();
        let test_instance = test_class.call0().unwrap();
        let do_method = test_instance.getattr("register_handlers").unwrap();
        do_method.call0().unwrap();
        println!(
            "Is registered: {}",
            message_bus.borrow().is_registered("negative_stream")
        );
    });

    println!("Endpoints: {:?}", message_bus.borrow().endpoints());

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(run_engine());
}
