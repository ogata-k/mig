extern crate clap;

use std::path::PathBuf;

// macros
use clap::{crate_authors, crate_description, crate_name, crate_version};
// no macros
use clap::{App, Arg, ArgMatches, Error};

use convert::convert_to_migration_file;
use file_helper::with_timestamp;
use framework::to_framework_type;

mod convert;
mod file_helper;
mod framework;

pub fn mig_app<'a, 'b>() -> App<'a, 'b> {
    return App::new(crate_name!())
        .about(crate_description!())
        .version(concat!("'s version: ", crate_version!()))
        .author(concat!(
            "If you have questions, contact to ",
            crate_authors!()
        ))
        .arg(
            Arg::with_name("INPUT")
                .short("I")
                .value_name("input-file")
                .required(true)
                .help("specify a path to the input file."),
        )
        .arg(
            Arg::with_name("TARGET_FW")
                .short("T")
                .long("target")
                .value_name("target-FW")
                .required(true)
                .help("specify a name of the target FrameWork."),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("O")
                .value_name("output-file")
                .required(true)
                .help("specify a path to the output file."),
        );
}

pub fn get_matches_safe<'a>(mig_app: App<'a, '_>) -> Result<ArgMatches<'a>, Error> {
    return mig_app.get_matches_safe();
}

pub fn action_controller(matches: ArgMatches) -> Result<&str, &str> {
    println!("checking for converter...");

    let input_file_opt = matches.value_of("INPUT");
    if input_file_opt.is_none() {
        return Err("input file is not specified. Why?");
    }
    // input_file_opt is not none!!!
    let input_file_path = PathBuf::from(input_file_opt.unwrap()); // TODO now, Provisional implementation

    // TODO get framework type by framework's name
    let framework_opt = matches.value_of("TARGET_FW");
    if framework_opt.is_none() {
        return Err("target frame work is not specified. Why?");
    }
    // framework_opt is not none!!
    let framework_type = to_framework_type(framework_opt.unwrap());
    if framework_type.is_none() {
        return Err("the framework is not supported.");
    }

    // TODO get output file path by using framework type
    let output_file_opt = matches.value_of("OUTPUT");
    if output_file_opt.is_none() {
        return Err("out put file is not specified. Why?");
    }
    let output_file_path_ = PathBuf::from(output_file_opt.unwrap()); // TODO now, Provisional implementation
    let output_file_path = with_timestamp(&output_file_path_, &framework_type.unwrap());

    // TODO input/output files existing and isFile check

    println!("finish checking for converter");

    let target_framework = framework_type.unwrap().clone();
    return convert_to_migration_file(&input_file_path, &output_file_path, target_framework);
}
