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

use std::{cell::UnsafeCell, rc::Rc};

use nautilus_common::{
    actor::registry::register_actor, msgbus::get_message_bus, testing::init_logger_for_testing,
};
use nautilus_demo::{
    LiveRunner, big_brain_actor::BigBrainActor, http_server::start_positive_stream_http_server,
    init_data_engine, websocket_server::NegativeStreamServer,
};
use pyo3::{BoundObject, prelude::*};

async fn run_engine() {
    let http_address = start_positive_stream_http_server().await.unwrap();
    let websocket_server = NegativeStreamServer::setup().await;

    // Initialize data client with http and websocket streams
    let (http_stream, websocket_stream) =
        init_data_engine(http_address, websocket_server.address).await;

    let mut runner = LiveRunner::default();
    runner.new_add_data_response_stream(http_stream);
    runner.new_message_stream(websocket_stream);
    runner.run().await;
}

pub fn main() {
    init_logger_for_testing(None).unwrap();
    let message_bus = get_message_bus();

    // Initialize big brain actor
    let big_brain_actor = BigBrainActor::new();
    let big_brain_actor = Rc::new(UnsafeCell::new(big_brain_actor));
    register_actor(big_brain_actor);
    BigBrainActor::register_message_handlers();

    // Initialize python actor
    // let root = env!("CARGO_MANIFEST_DIR");
    // let code = std::fs::read_to_string(format!("{}/src/big_brain_actor.py", root)).unwrap();
    // let code = CString::new(code).unwrap();
    // let filename = CString::new("big_brain_actor".to_string()).unwrap();
    // let module = CString::new("big_brain_actor".to_string()).unwrap();

    // pyo3::prepare_freethreaded_python();

    // Python::with_gil(|py| {
    //     let pymod = PyModule::from_code(py, &code, &filename, &module).unwrap();
    //     let test_class = pymod.getattr("BigBrainActor").unwrap();
    //     let test_instance = test_class.call0().unwrap();
    //     let do_method = test_instance.getattr("register_handlers").unwrap();
    //     do_method.call0().unwrap();
    // });

    println!("Endpoints: {:?}", message_bus.borrow().endpoints());

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(run_engine());
}
