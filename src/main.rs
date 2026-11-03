use capsule_calc_interpreter::capsule_calc::collection_of_const_funcs::DIV;
use capsule_calc_interpreter::capsule_calc::{Environment, c_int, t_application, t_const};
use capsule_calc_interpreter::cli::start_cli;

fn main() -> std::io::Result<()> {
    let term = Box::new(t_application(
        t_application(t_const(DIV()), t_const(c_int(5))),
        t_const(c_int(0)),
    ));
    let environment = Environment::new();

    start_cli(term, environment)
}

// fn main() {
//     println!("{:?}", lex("(x:=3)"));
// }

// TODO: test Landin's knot
// TODO: make thing for CLI that does n iterations
// TODO: garbage collection (make the step function takes a parameter that garbage collects)
// TODO: parser
