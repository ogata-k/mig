extern crate clap;

// macros
use clap::{crate_authors, crate_description, crate_name, crate_version};
// no macros
use clap::{App, Arg, ArgMatches};

pub fn mig_app<'a, 'b>() -> App<'a, 'b> {
    return App::new(crate_name!())
        .about(crate_description!())
        .version(concat!("'s version: ", crate_version!()))
        .author(concat!(
            "If you have questions, contact to ",
            crate_authors!()
        ))
        .arg(Arg::with_name("INPUT")
            .short("I")
            .value_name("input-file")
            .required(true)
            .help("specify a path to the input file.")
        )
        .arg(Arg::with_name("TARGET_FW")
            .short("T")
            .long("target")
            .value_name("target-FW")
            .required(true)
            .help("specify a name of the target Frame-Work.")
        )
        .arg(Arg::with_name("OUTPUT")
            .short("O")
            .value_name("output-file")
            .help("specify a path to the output file.")
        );
}

pub fn action_controller(matches: ArgMatches) {
    // TODO control sequence with matches to do action.
    println!("in action controller");
}