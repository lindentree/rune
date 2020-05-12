use std::error::Error;

use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};
use std::path::PathBuf;
use structopt::StructOpt;

use image::{Rgba, GenericImageView};

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: PathBuf
}

pub fn load_graph(mut file: std::path::Path) -> Result<(), Box<dyn Error>> {

    let opt = Opt::from_args();

    //First, we load up the graph as a byte array
    let model = include_bytes!("mobilenet_v1_1.0_224_frozen.pb");

    //Then we create a tensorflow graph from the model
    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?;

    let input_image = image::open(&file)?;

    let mut flattened: Vec<f32> = Vec::new();

    for (_x, _y, rgb) in input_image.pixels() {
        flattened.push(rgb[2] as f32);
        flattened.push(rgb[1] as f32);
        flattened.push(rgb[0] as f32);
    }

    //The `input` tensor expects BGR pixel data.
    let input = Tensor::new(&[input_image.height() as u64, input_image.width() as u64, 3])
        .with_values(&flattened)?;

    let thresholds = Tensor::new(&[3]).with_values(&[0.6f32, 0.7f32, 0.7f32])?;
    let factor = Tensor::new(&[]).with_values(&[0.709f32])?;

    let mut args = SessionRunArgs::new();

    //Load default parameters
    args.add_feed(&graph.operation_by_name_required("thresholds")?,0, &thresholds);
    args.add_feed(&graph.operation_by_name_required("factor")?, 0, &factor);

    //Request the following outputs after the session runs
    let prob = args.request_fetch(&graph.operation_by_name_required("prob")?, 0);

    let mut session = Session::new(&SessionOptions::new(), &graph)?;

    session.run(&mut args)?;

     //Our probability
     let prob_res: Tensor<f32> = args.fetch(prob)?;

     println!("{}", prob_res);


    Ok(())
}