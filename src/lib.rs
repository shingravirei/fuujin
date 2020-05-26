mod template;
mod utils;

use clap::{App, Arg};
use std::error::Error;
use std::fs;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Options {
    pub component_name: String,
    pub custom_path: Option<String>,
}

impl Options {
    pub fn new(component_name: String, custom_path: Option<String>) -> Options {
        Options {
            component_name,
            custom_path,
        }
    }
}

pub fn config() -> Options {
    let matches = App::new("fuujin")
        .version("0.1.0")
        .author("Shingravirei")
        .about("Generate React components")
        .arg(
            Arg::with_name("NAME")
                .about("name of the component")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("custom_path")
                .about("add a custom path for your component")
                .short('p')
                .long("path")
                .value_name("PATH")
                .takes_value(true),
        )
        .get_matches();

    let component_name = matches.value_of("NAME").unwrap().into();
    let custom_path = match matches.value_of("custom_path") {
        Some(value) => Some(value.into()),
        None => None,
    };

    Options::new(component_name, custom_path)
}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let path = utils::get_path(options.custom_path);

    utils::create_component_path(path.clone())?;

    let component_path = format!("{}/{}.js", path, options.component_name);

    let mut file = fs::File::create(component_path)?;

    let writeble_template = template::RFC.replace("<PNAME>", options.component_name.as_ref());

    file.write_all(writeble_template.as_bytes())?;

    Ok(())
}
