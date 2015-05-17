extern crate rustc_serialize;
extern crate docopt;

static USAGE: &'static str = "
Usage:
    echo [-n] <args>...

Options:
  -n    no newline at the end
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_args: Vec<String>,
    flag_n: bool,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    if args.flag_n {
        print!("{}", args.arg_args.connect(" "));
    } else {
        println!("{}", args.arg_args.connect(" "));
    }
}
