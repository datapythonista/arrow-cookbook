mod create_arrays;
mod low_level_api;

fn call_create_arrays() {
    println!("Int32Array::from(vec![1, 2, 3]): {:?}",
             create_arrays::create_array_with_constructor());
    println!("Int32Array::from(vec![Some(1), None, Some(3)]): {:?}",
             create_arrays::create_array_with_constructor_and_nulls());
    println!("Float64Array::from(vec![1., 1.5, 2.]): {:?}",
             create_arrays::create_float_array());
    println!("Int32Builder::new() ... .append_value(1) ...: {:?}",
             create_arrays::create_array_with_default_builder());
    println!("Int32Builder::with_capacity(2) ... .append_value(1) ...: {:?}",
             create_arrays::create_array_with_builder_and_capacity());
    println!("vec![1, 2, 3].into_iter().collect::<Int32Array>(): {:?}",
             create_arrays::create_array_with_collect());
    println!("StringBuilder::with_capacity(4, 32) ... .append_value(\"foo\") ...: {:?}",
             create_arrays::create_string_array_with_builder());
    println!("vec![\"foo\", \"bar\", \"foobar\"].into_iter().map(Some).collect::<StringArray>(): {:?}",
             create_arrays::create_string_array_with_collect());
}

fn call_low_level_api() {
    low_level_api::introspect_int_array();
    low_level_api::introspect_int_array_capacity();
    low_level_api::introspect_int_array_with_nulls();
    low_level_api::introspect_string_array();
    println!("ArrayData::try_new(...): {:?}",
             low_level_api::create_int_array_from_buffer());
}

fn main() {
    call_create_arrays();
    call_low_level_api();
}
