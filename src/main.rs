extern crate clap; 
extern crate rand;

use clap::{Arg, App};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


fn main() {
    let matches = App::new("findx")
                          .version("1.0")
                          .author("Souissi Mohamed")
                          .about("Small pipe-like program")
                          .arg(Arg::with_name("in_file")
                            .long("in_file")
                            .takes_value(true)
                            .required(true))
                          .arg(Arg::with_name("out_file")
                            .help("set the outputFile")
                            .takes_value(true)
                            .long("out_file"))    
                          .get_matches();
    
    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    if !matches.value_of("out_file").is_none() {
        rand_string = matches.value_of("out_file").unwrap().to_string();
    }

    println!("{}", rand_string);
    println!("{}", matches.value_of("in_file").unwrap().to_string());
}

