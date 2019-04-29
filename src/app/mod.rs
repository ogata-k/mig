extern crate clap;

use std::path::PathBuf;

// macros
use clap::{crate_authors, crate_description, crate_name, crate_version};
// no macros
use clap::{App, Arg, ArgMatches, Error};

use converter::convert_to_migration_file;
use framework::to_framework_type;
use helper::file_helper::{
    get_extension_for_framework,
    get_file_name_for_framework,
    is_extension,
    with_timestamp,
};
use helper::io_helper::confirm;

mod converter;
mod framework;
mod helper;

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
                .help("specify the name of the target FrameWork."),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("O")
                .value_name("output-file")
                .required(true)
                .help("specify the name of the output file."),
        );
}

pub fn get_matches_safe<'a>(mig_app: App<'a, '_>) -> Result<ArgMatches<'a>, Error> {
    return mig_app.get_matches_safe();
}

pub fn action_controller(matches: ArgMatches) -> Result<&str, &str> {
    println!("checking condition..");

    let input_file_opt = matches.value_of("INPUT");
    if input_file_opt.is_none() {
        return Err("input file is not specified. Why?");
    }
    // input_file_opt is not none!!!
    let input_file_opt_str = input_file_opt.unwrap();
    let input_file_path = PathBuf::from(input_file_opt_str);

    let framework_opt = matches.value_of("TARGET_FW");
    if framework_opt.is_none() {
        return Err("target frame work is not specified. Why?");
    }
    // framework_opt is not none!!
    let framework_type = to_framework_type(framework_opt.unwrap());
    if framework_type.is_none() {
        return Err(" cannot analyze the framework or the framework is not supported.");
    }

    let output_file_opt = matches.value_of("OUTPUT");
    if output_file_opt.is_none() {
        return Err("out put file is not specified. Why?");
    }
    let output_file_path_string =
        with_timestamp(&output_file_opt.unwrap(), &framework_type.unwrap());
    let output_file_path = PathBuf::from(output_file_path_string.as_str());

    // check extension
    if !is_extension(&input_file_path, "mig") {
        return Err("input file is not a mig-file.");
    }
    // error is never realize!!
    if !is_extension(
        &output_file_path,
        get_extension_for_framework(&framework_type.unwrap()).as_str(),
    ) {
        return Err("output file is not a file for the framework");
    }

    // check these files is file
    if !input_file_path.is_file() {
        return Err("input file is not file");
    }

    // check these files is existing
    if !input_file_path.exists() {
        return Err("input file doesn't exist");
    }
    // TODO もし今後似たようなファイルが存在するとき作成するか聞くような仕様にするなら以下を実装
    /*
    if output_file_path.exists_with_ignore_timestamp() {
        // 確認
        if !confirm("remake the output file?") {
            return Err("output file exist");
        }
    }
    */

    println!("finish checking condition");

    let target_framework = framework_type.unwrap().clone();
    return convert_to_migration_file(&input_file_path, &output_file_path, target_framework);
}
