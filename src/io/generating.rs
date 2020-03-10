use crate::{
    configs::generator,
    io::{MapFileExt, SupportingFileExts, SupportingMapFileExts},
    network::Graph,
};
use log::info;

pub struct Generator;

impl Generator {
    pub fn generate(graph: &Graph, cfg: &generator::Config) -> Result<(), String> {
        info!("START Generate file from graph");
        match Generator::from_path(&cfg.map_file)? {
            MapFileExt::FMI => {
                fmi::Generator::new().generate(graph, cfg)?;
                info!("FINISHED");
                Ok(())
            }
            MapFileExt::PBF => Err(String::from("No support for generating pbf-files.")),
        }
    }
}

impl SupportingMapFileExts for Generator {}
impl SupportingFileExts for Generator {
    fn supported_exts<'a>() -> &'a [&'a str] {
        // &["fmi"] // TODO :3
        &[]
    }
}

trait Generating {
    fn generate(&self, graph: &Graph, cfg: &generator::Config) -> Result<(), String>;
}

pub mod fmi {
    use crate::{configs::generator, helpers, network::Graph};
    use std::io::{BufWriter, Write};

    pub struct Generator;

    impl Generator {
        pub fn new() -> Generator {
            Generator {}
        }
    }

    impl super::Generating for Generator {
        fn generate(&self, graph: &Graph, cfg: &generator::Config) -> Result<(), String> {
            let output_file = helpers::open_new_file(&cfg.map_file)?;
            let mut writer = BufWriter::new(output_file);

            // write graph to file
            writeln!(writer, "{}", graph.nodes().count()).expect("TODO");

            // return success
            Ok(())
        }
    }
}
