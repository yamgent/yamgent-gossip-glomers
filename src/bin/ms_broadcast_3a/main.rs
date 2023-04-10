use std::collections::{HashMap, HashSet};

use maelstrom_rs::{
    actor::Actor,
    message::{Request, Response},
    runtime::Runtime,
};
use serde_json::{Map, Value};

fn main() {
    let node = BroadcastActor {
        node_id: None,
        seen: HashSet::new(),
        topology: HashMap::new(),
    };
    let mut runtime = Runtime::new(Box::new(node));
    runtime.start();
}

struct BroadcastActor {
    node_id: Option<String>,
    seen: HashSet<i64>,
    topology: HashMap<String, Vec<String>>,
}

impl Actor for BroadcastActor {
    fn init(
        &mut self,
        node_id: &str,
        _node_ids: Vec<String>,
    ) -> Result<(), maelstrom_rs::error::Error> {
        self.node_id = Some(node_id.to_string());
        eprintln!("node {} initialized", node_id);
        Ok(())
    }

    fn receive(&mut self, request: &Request) -> Result<Vec<Response>, maelstrom_rs::error::Error> {
        match request.message_type.as_str() {
            "broadcast" => {
                self.seen
                    .insert(request.body.get("message").unwrap().as_i64().unwrap());
                Ok(vec![Response::new_from_request(request, Map::new())])
            }
            "read" => {
                let mut body = Map::new();

                body.insert(
                    "messages".to_string(),
                    Value::Array(
                        self.seen
                            .iter()
                            .map(|v| Value::from(*v))
                            .collect::<Vec<_>>(),
                    ),
                );

                Ok(vec![Response::new_from_request(request, body)])
            }
            "topology" => {
                self.topology = request
                    .body
                    .get("topology")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.to_string(),
                            value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v.as_str().unwrap().to_string())
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                Ok(vec![Response::new_from_request(request, Map::new())])
            }
            _ => unimplemented!(),
        }
    }
}
