//! Arrow Cookbook: Low-level API
//!
//! Most Arrow users do not need to get into the details of how and where
//! memory is allocated. But in some cases it may be useful to be able to
//! check the details, or create Arrow arrays directly from buffers of data.
//!
//! This chapter illustrates this lower level API, and it can be useful to
//! better understand the Arrow internal representation.

use arrow::array::{UInt8Array,UInt8Builder,
                   StringArray,
                   ArrayData,Array};
use arrow::buffer::Buffer;
use arrow::datatypes::DataType;

/// Introspect a simple integer array.
///
/// An Arrow array, in its simplest case is a location in the memory, with
/// know data type and lenght. In this example we create a simple array of
/// unsigned 8-bit integers, and we check its length, its data type, the
/// address (pointer) in memory of its buffer, and the stored data.
///
/// Arrow arrays are often more complex, containing multiple buffers. Those
/// will be shown in later examples.
pub fn introspect_int_array() {
    let array = UInt8Array::from(vec![1, 2, 3]);

    let array_data = array.data();
    println!("array len: {:?}", array_data.len());
    println!("array data_type: {:?}", array_data.data_type());

    let buffer = &array_data.buffers()[0];
    println!("data buffer pointer: {:?}", buffer.as_ptr());
    println!("data buffer len: {:?}", buffer.len());
    println!("data buffer data: {:?}", buffer.as_slice());
}

/// Introspect an integer array capacity.
///
/// An array capacity (allocated memory) can be different than an array
/// length (used memory). In this example we create an array with bigger
/// capacity than needed, and we compare how both quantities differ.
///
/// We can also see how Arrow is aware of the array length, and data in
/// the allocated memory of the array in the unused are is ignored by
/// functions such as `as_slice`, as one would expect.
pub fn introspect_int_array_capacity() {
    let mut builder = UInt8Builder::with_capacity(128);
    builder.append_value(1);
    builder.append_value(2);
    builder.append_value(3);
    let array = builder.finish();
    println!("introspect int array capacity: {:?}", array);

    let array_data = array.data();
    println!("array len: {:?}", array_data.len());

    let buffer = &array_data.buffers()[0];
    println!("data buffer len: {:?}", buffer.len());
    println!("data buffer capacity: {:?}", buffer.capacity());
    println!("data buffer data: {:?}", buffer.as_slice());
}

/// Introspect an integer array containing null values
///
/// In this example, a more complex array is created, containing null
/// values. When null values exist, Arrow automatically creates a
/// buffer to store a bitmap with the values that are null.
///
/// The next code shows an example of how Arrow represents an array with
/// null values:
///
/// ```rust
/// let array_to_represent = [Some(1), None, Some(3)];
///
/// let internal_data = [1, 0, 3];
/// let internal_null_bitmap = [1, 0, 1];
/// ```
///
/// As we can see, `internal_null_bitmap` contains the positions of the
/// array that are not null as ones. So, the second element is considered
/// a null value. Note that `internal_data` still has a value for it (`0`
/// in the example), but the value is not relevant, as Arrow will consider
/// the value as null.
pub fn introspect_int_array_with_nulls() {
    // Create a sample array
    let mut builder = UInt8Builder::new();
    builder.append_value(1);
    builder.append_null();
    builder.append_value(3);
    let array = builder.finish();
    println!("introspect int array with nulls: {:?}", array);

    // Basic array information
    let array_data = array.data();
    println!("array len: {:?}", array_data.len());
    println!("array data_type: {:?}", array_data.data_type());

    let data_buffer = &array_data.buffers()[0];
    println!("data buffer pointer: {:?}", data_buffer.as_ptr());
    println!("data buffer len: {:?}", data_buffer.len());
    println!("data buffer capacity: {:?}", data_buffer.capacity());
    println!("data buffer data: {:?}", data_buffer.as_slice());

    // Null buffer information
    match array_data.null_buffer() {
        Some(null_buffer) => {
            println!("null buffer pointer: {:?}", null_buffer.as_ptr());
            println!("null buffer capacity: {:?}", null_buffer.capacity());
            println!("null buffer bits: {:08b}", null_buffer.as_slice()[0]);

            let null_bitmap = array_data.null_bitmap().unwrap();
            println!("null bitmap bit_len: {:?}", null_bitmap.bit_len());
        }
        None => println!("null buffer does not exist")
    }
}

// TODO
pub fn introspect_string_array() {
    let array = StringArray::from(vec!["this", "is", "an", "array"]);
    println!("introspect string array: {:?}", array);

    // Basic array information
    let array_data = array.data();
    println!("array len: {:?}", array_data.len());
    println!("array data_type: {:?}", array_data.data_type());

    // Offset buffer information
    let offset_buffer = &array_data.buffers()[0];
    println!("offset buffer pointer: {:?}", offset_buffer.as_ptr());
    println!("offset buffer len: {:?}", offset_buffer.len());
    println!("offset buffer capacity: {:?}", offset_buffer.capacity());
    println!("offset buffer data: {:?}", offset_buffer.as_slice()); // TODO better representation

    // Data buffer information
    let data_buffer = &array_data.buffers()[1];
    println!("data buffer pointer: {:?}", data_buffer.as_ptr());
    println!("data buffer len: {:?}", data_buffer.len());
    println!("data buffer capacity: {:?}", data_buffer.capacity());
    println!("data buffer data: {:?}", data_buffer.as_slice()); // TODO better representation

}

pub fn create_int_array_from_buffer() -> UInt8Array {
    let data = Buffer::from([1, 2, 3]);
    // let nulls = Buffer::from([0b00000010]);
    let array_data = ArrayData::try_new(DataType::UInt8,
                                        3, // length of the array
                                        None, // bit mask with nulls
                                        0, // offset
                                        vec![data], // vector of buffers
                                        vec![]).unwrap(); // child_data
                                        
    UInt8Array::from(array_data)
}
