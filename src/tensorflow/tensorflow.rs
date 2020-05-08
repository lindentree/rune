use std::error::Error;

use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};


fn load_graph() -> Result<(), Box<dyn Error>> {

    //First, we load up the graph as a byte array
    let model = include_bytes!("mobilenet_v1_1.0_224_frozen.pb");

    //Then we create a tensorflow graph from the model
    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?

    Ok(())
}