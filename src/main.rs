use capsule_calc_interpreter::capsule_calc::some_funcs::{INCR, IS_5};
use capsule_calc_interpreter::capsule_calc::{
    Environment, c_int, t_application, t_assignment, t_composition, t_const, t_repeat_until, t_var,
    variable,
};
use capsule_calc_interpreter::cli::start_cli;

fn main() -> std::io::Result<()> {
    // let term = Box::new(t_composition(
    //     t_assignment(variable("x"), t_const(c_int(0))),
    //     t_repeat_until(
    //         t_assignment(
    //             variable("x"),
    //             t_application(t_const(INCR()), t_var(variable("x"))),
    //         ),
    //         t_application(t_const(IS_5()), t_var(variable("x"))),
    //     ),
    // ));
    // let environment = Environment::new();

    let term = Box::new(t_composition(
        t_assignment(variable("x"), t_const(c_int(0))),
        t_repeat_until(
            t_assignment(
                variable("x"),
                t_application(t_const(INCR()), t_var(variable("x"))),
            ),
            t_application(t_const(IS_5()), t_var(variable("x"))),
        ),
    ));
    let environment = Environment::new();

    start_cli(term, environment)
}

// TODO: test Landin's knot
// TODO: make thing for CLI that does n iterations
// TODO: garbage collection
