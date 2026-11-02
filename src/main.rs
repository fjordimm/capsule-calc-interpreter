use capsule_calc_interpreter::capsule_calc::{Environment, c_int, t_application, t_const};
use capsule_calc_interpreter::cli::start_cli;
use capsule_calc_interpreter::some_funcs::INCR;

fn main() -> std::io::Result<()> {
    let term = Box::new(t_application(t_const(INCR()), t_const(c_int(3))));
    let environment = Environment::new();

    start_cli(term, environment)
}

// TODO: test Landin's knot
// TODO: make thing for CLI that does n iterations
// TODO: garbage collection (make the step function takes a parameter that garbage collects)
// TODO: parser
