use std::sync::{Arc, Mutex};
use serde_json::Value;
use tiny_http::{Server, Response};

use crate::state::State;
use crate::vm::Vm;

pub async fn start_rpc(addr: &str, state: Arc<Mutex<State>>) {
    let server = Server::http(addr).unwrap();
    println!("[RPC] Server attivo su {}", addr);

    for request in server.incoming_requests() {
        let mut body = String::new();
        request.as_reader().read_to_string(&mut body).unwrap();

        let json: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
        let method = json["method"].as_str().unwrap_or("");
        let params = json["params"].clone();

        let result = match method {
            "vm_deploy" => {
                let name = params["contract"].as_str().unwrap_or("");
                let code = params["code"].as_str().unwrap_or("");
                let mut vm = state.lock().unwrap().vm.lock().unwrap();
                vm.deploy(name, code)
            }

            "vm_call" => {
                let name = params["contract"].as_str().unwrap_or("");
                let func = params["function"].as_str().unwrap_or("");
                let args = params["args"].clone();
                let mut vm = state.lock().unwrap().vm.lock().unwrap();
                vm.call(name, func, args)
            }

            "vm_exec_zlang" => {
                let code = params["code"].as_str().unwrap_or("");
                let mut vm = state.lock().unwrap().vm.lock().unwrap();
                vm.execute_zlang(code)
            }

            _ => Err("unknown method".into()),
        };

        let response = match result {
            Ok(v) => json!({"status": "ok", "result": v}),
            Err(e) => json!({"status": "error", "message": e}),
        };

        let resp = Response::from_string(response.to_string());
        request.respond(resp).unwrap();
    }
}
