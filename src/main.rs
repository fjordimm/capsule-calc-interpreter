use capsule_calc_interpreter::capsule_calc::some_funcs::INCR;
use capsule_calc_interpreter::capsule_calc::{
    Environment, c_int, t_application, t_constant, t_lambda, t_var, varbl,
};
use capsule_calc_interpreter::cli::start_cli;

fn main() -> std::io::Result<()> {
    let mut term = Box::new(t_application(
        t_lambda(varbl("x"), t_application(INCR(), t_var(varbl("x")))),
        t_constant(c_int(100)),
    ));
    let mut env = Environment::new();

    start_cli(&mut term, &mut env)
}
