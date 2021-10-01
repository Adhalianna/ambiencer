extern crate clap;
use clap::{App, Arg, SubCommand};

mod project;

fn main() {
    let app_matches = App::new("Ambiencer")
                            .version("0.1")
                            .author("Natalia K. G. <natka.goc@gmail.com>")
                            .about("Ambience track composer and player")
                            .subcommand(SubCommand::with_name("play")
                                .about("Plays a specified Ambiencer project")
                                .arg(Arg::with_name("INPUT")
                                    .required(true)
                                    .index(1)))
                            .subcommand(SubCommand::with_name("edit")
                                .about("Edits a chosen Ambiencer project")
                                .arg(Arg::with_name("INPUT")
                                    .required(true)
                                    .index(1)))
                            .subcommand(SubCommand::with_name("new")
                                .about("Creates a new Ambiencer project")
                                .arg(Arg::with_name("NAME")
                                    .required(true)
                                    .index(1)))
                            .get_matches();

}
