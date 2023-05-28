use crate::functions::print_labeled_measurement;
use crate::functions::y_value;
use crate::functions::plus_one;
use crate::control_flow::is_three;
use crate::control_flow::let_if_in_let_expression;
use crate::control_flow::values_from_loop;
use crate::control_flow::labeled_loop;
use crate::control_flow::loop_with_while;
use crate::control_flow::loop_with_for;
use crate::ownership::clone_data;
use crate::ownership::ownership_and_function;
use crate::ownership::ownership_return_and_scope;
use crate::references_and_borrowing::reference_to_an_object;
use crate::references_and_borrowing::mutable_references;

mod functions;
mod control_flow;
mod ownership;
mod references_and_borrowing;


fn main() {

    // print_labeled_measurement(5, 'h');
    // y_value();
    // let plus_one_var = plus_one(6);
    // println!("The value of plus_one_var is: {plus_one_var}");
    // is_three(4);
    // let_if_in_let_expression();
    // values_from_loop();
    // labeled_loop();
    // loop_with_while();
    // loop_with_for();
    // clone_data();
    // ownership_and_function();
    // ownership_return_and_scope();
    // reference_to_an_object();
    // mutable_references();
}
