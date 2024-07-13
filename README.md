# ByteBuffer
The `ByteBuffer` Ruby gem provides a powerful and flexible interface for buffered IO data, wrapping the [bytebuffer](https://crates.io/crates/bytebuffer) crate using FFI.

## Overview
ByteBuffer offers an interface to the compiled rust library for manipulating buffered IO, e.g. reading and writing bits, bytes, integers of various sizes, and floats.
## Getting Started
#### Dependencies
The following packages are required to build and use the gem:
* `ruby` >= 3.0
* `rustc`
* `cargo`

Ensure the following toolchains are installed via rustup:
* `x86_64-unknown-linux-gnu`
* 

#### Usage
A basic example of the ByteBuffer in action can be found below:
```ruby
require 'bytebuffer'

bb = ByteBuffer.new
bb.write_i32(-128)
puts bb.read_i32 # Output: -128
bb.free
```