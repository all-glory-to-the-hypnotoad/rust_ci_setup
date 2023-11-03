
/// This function calls [`my_module::my_submodule::submodule_function`] from another module.
///
/// [`my_module::my_submodule::submodule_function`]: my_module::my_submodule::submodule_function
///
/// This is a private function that adds two numbers.
///
/// # Examples
///
/// ```rust
/// let result = add_numbers(3, 5);
/// assert_eq!(result, 8);
/// ```
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}



pub fn foo() {
    add_numbers(1, 2);
}
