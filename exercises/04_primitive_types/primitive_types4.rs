// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    // in slice the last element you add after .. is exclusive
    let nice_slice = &a[1..4]; //* passing the array as reference and taking slice  */

    assert_eq!([2, 3, 4], nice_slice)
}
