use maelstrom_rs::{
    actor::Actor,
    message::{Request, Response},
    runtime::Runtime,
};
use serde_json::{Map, Value};

fn main() {
    let node = UnqiueIdActor {
        node_id: None,
        next: 0,
    };
    let mut runtime = Runtime::new(Box::new(node));
    runtime.start();
}

struct UnqiueIdActor {
    next: i32,
    node_id: Option<String>,
}

impl Actor for UnqiueIdActor {
    fn init(
        &mut self,
        node_id: &str,
        _node_ids: Vec<String>,
    ) -> Result<(), maelstrom_rs::error::Error> {
        self.node_id = Some(node_id.to_string());
        self.next = node_id
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        eprintln!("node {} initialized", node_id);
        Ok(())
    }

    fn receive(&mut self, request: &Request) -> Result<Vec<Response>, maelstrom_rs::error::Error> {
        match request.message_type.as_str() {
            "generate" => {
                let mut body = Map::new();
                body.insert("id".to_string(), Value::from(self.next));

                self.next += 3;

                Ok(vec![Response::new_from_request(request, body)])
            }
            _ => unimplemented!(),
        }
    }
}
