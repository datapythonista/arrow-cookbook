mod create_arrays;

fn call_create_arrays() {
    println!("Int32Array::from(vec![1, 2, 3]): {:?}",
             create_arrays::create_array_with_constructor());
    println!("Int32Array::from(vec![Some(1), None, Some(3)]): {:?}",
             create_arrays::create_array_with_constructor_and_nulls());
    println!("Float64Array::from(vec![1., 1.5, 2.]): {:?}",
             create_arrays::create_float_array());
    println!("Int32Builder::with_capacity(6) ... .append_value(1) ...: {:?}",
             create_arrays::create_array_with_builder());
    println!("vec![1, 2, 3].into_iter().collect::<Int32Array>(): {:?}",
             create_arrays::create_array_with_collect());
    println!("StringBuilder::with_capacity(3, 6) ... .append_value(\"foo\") ...: {:?}",
             create_arrays::create_string_array_with_builder());
    println!("vec![\"foo\", \"bar\", \"foobar\"].iter().collect::<StringArray>(): {:?}",
             create_arrays::create_string_array_with_collect());
}

fn main() {
    call_create_arrays();
}
