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

/// Create an integer array with the default builder.
///
/// This is useful to append values one at a time, for example
/// if the data needs to be iterated when read from disk.
pub fn create_array_with_default_builder() -> Int32Array {
    let mut builder = Int32Builder::new();
    builder.append_value(1);
    builder.append_null();
    builder.append_value(3);
    builder.append_slice(&(4..6).collect::<Vec<i32>>());
    builder.append_option(None);
    builder.append_option(Some(8));
    builder.finish()
}

/// Create an integer array with the builder with the initial capacity.
///
/// When creating a new array, Arrow needs to allocate an initial amount
/// of memory for it. By default, memory for 1024 items is usually allocated.
/// This may not be ideal. If for example our data is only a couple items, we
/// would be wasting most of the allocated memory, which could be used by other
/// processes in the operating system instead. If our data contains milions
/// of items, many more allocations will be needed, and each new allocation
/// requires a significant amount of time, so our program will become slower.
///
/// For this reason, when creating a builder, we can specify the capacity,
/// based on the information we have about our data. This can make our
/// program to minimize allocations, run as fast as possible, and avoid
/// using unnecessary memory.
pub fn create_array_with_builder_and_capacity() -> Int32Array {
    let mut builder = Int32Builder::with_capacity(2);
    builder.append_value(1);
    builder.append_null();
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

/// Create a string array with the builder and the initial capacity.
///
/// When creating a string array with the builder, we can use the default
/// builder `new`, or we can specify the capacity, as seen in previous
/// examples. In the case of strings, we need to specify the number of items
/// as well as the size of the data.
///
/// An array of strings is represented with two different buffers, one for
/// the data, and one with the initial and final position of each item in
/// the data. For example:
///
/// ```rust
/// let data = "thisisanarray   ";
/// let offsets = [0, 4, 6, 8, 13];
///
/// let array = ["this", "is", "an", "array"];
/// ```
///
/// Note that the data can have more memory allocated than actually used.
/// And note that the offsets contain one more item than the array, since
/// it contains the position of each element, and one extra item to know
/// where the last item finishes.
pub fn create_string_array_with_builder() -> StringArray {
    let mut builder = StringBuilder::with_capacity(4, 32);
    builder.append_value("foo");
    builder.append_value("bar".to_string());
    builder.append_null();
    builder.append_option(Some("foobar"));
    builder.finish()
}

/// Create a string array with the `collect` method of an iterator.
///
/// Arrow currently does not implement collecting from a string vector
/// directly, but from a vector of `Option`. To collect from a vector of
/// strings, we can simply map it to `Some`.
pub fn create_string_array_with_collect() -> StringArray {
    vec!["foo", "bar", "foobar"].into_iter().map(Some).collect::<StringArray>()
}
