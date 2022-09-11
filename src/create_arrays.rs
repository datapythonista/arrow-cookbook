//! Arrow Cookbook: Creating arrays
//!
//! There are different ways to create the the Arrow array.
//! The main reason to use one or the other is depending on whether
//! the data is already in memory in a rust collection, or if it needs
//! to be iterated, for example because it is read from disk.

use arrow;
use arrow::array::{Int32Array, Int32Builder,
                   Float64Array,
                   StringArray, StringBuilder};


/// Create an integer array with the `from` constructor.
///
/// This is useful if the data is already allocated in memory,
/// for example as a vector.
pub fn create_array_with_constructor() -> Int32Array {
    Int32Array::from(vec![1, 2, 3])
}

/// Create an integer array with missing values.
///
/// Arrow supports missing values, the constructor can handle
/// a vector of options. Missing values are represented as `null`
/// in Arrow, and rust's `None` values are considered missing values
/// too.
pub fn create_array_with_constructor_and_nulls() -> Int32Array {
    Int32Array::from(vec![Some(1), None, Some(3)])
}

/// Create array of different types.
///
/// Arrow can create arrays of a number of different types.
/// Signed and unsigned integers of 8, 16, 32 or 64 bits,
/// floats, boolean, strings, dates, etc. Every type has an
/// array class that can be used in the same way.
pub fn create_float_array() -> Float64Array {
    Float64Array::from(vec![1., 1.5, 2.])
}

/// Create an integer array with the builder.
///
/// This is useful to append values one at a time, for example
/// if the data needs to be iterated when read from disk.
///
/// We must specify the capacity when creating a builder.
/// The capacity is the initial capacity of the array,
/// not a hard limit. More memory will be automatically
/// allocated if needed (more allocations usually mean slower execution).
pub fn create_array_with_builder() -> Int32Array {
    let mut builder = Int32Builder::with_capacity(8);
    builder.append_value(1);
    builder.append_null();
    builder.append_value(3);
    builder.append_slice(&(4..6).collect::<Vec<i32>>());
    builder.append_option(None);
    builder.append_option(Some(8));
    builder.finish()
}

/// Create an integer array with the `collect` method of an iterator.
///
/// This is an alternative syntax to using the constructor, when the
/// data is already in memory, and the data is an iterator or can be
/// easily converted to it.
pub fn create_array_with_collect() -> Int32Array {
    vec![1, 2, 3].into_iter().collect::<Int32Array>()
}

/// Create a string array with the builder.
///
/// When creating a string array with the builder, we need to specify
/// the capacity not only with the number of elements we expect, but
/// also the total size in bytes to preallocate for the data. Internally,
/// Arrow stores all the strings in a single array for the data, and then
/// it has a separate array to identify in which position (offset) each
/// string starts.
pub fn create_string_array_with_builder() -> StringArray {
    let mut builder = StringBuilder::with_capacity(4, 32);
    builder.append_value("foo");
    builder.append_value("bar".to_string());
    builder.append_null();
    builder.append_option(Some("foobar"));
    builder.finish()
}

pub fn create_string_array_with_collect() -> StringArray {
    // TODO: This is not yet working
    // vec!["foo", "bar", "foobar"].into_iter().collect::<StringArray>()
    StringArray::from(vec!["foo", "bar", "foobar"])
}

