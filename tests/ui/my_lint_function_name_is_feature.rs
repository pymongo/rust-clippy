#![warn(clippy::my_lint_function_name_is_feature)]

fn main() {
    let feature = 1;
    fn feature() {

    }
}
