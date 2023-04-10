use maelstrom_rs::{
    actor::Actor,
    message::{Request, Response},
    runtime::Runtime,
};
use serde_json::{Map, Value};

fn main() {
    let node = EchoActor { node_id: None };
    let mut runtime = Runtime::new(Box::new(node));
    runtime.start();
}

struct EchoActor {
    node_id: Option<String>,
}

impl Actor for EchoActor {
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
            "echo" => {
                let mut body = Map::new();
                body.insert(
                    "echo".to_string(),
                    Value::from(request.body.get("echo").unwrap().as_str().unwrap()),
                );
                Ok(vec![Response::new_from_request(request, body)])
            }
            _ => unimplemented!(),
        }
    }
}
