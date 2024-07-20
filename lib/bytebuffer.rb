require 'ffi'
require_relative 'bytebuffer/version'

# This module extends the FFI::Library to provide functions used to interact with the compiled rust library.
module ByteBufferExtension
  extend FFI::Library

  case FFI::Platform::OS
  when 'linux' then ffi_lib ["#{File.dirname(__FILE__)}/bytebuffer.so"]
  when 'darwin' then ffi_lib ["#{File.dirname(__FILE__)}/bytebuffer.bundle"]
  else ffi_lib ["#{File.dirname(__FILE__)}/bytebuffer.so"]
  end

  attach_function(:new, :new, [], :pointer)
  attach_function(:from_bytes, :from_bytes, [:pointer, :int], :pointer)

  attach_function(:drop, :drop, [:pointer], :void)

  attach_function(:length, :length, [:pointer], :int)

  attach_function(:set_write_position, :set_write_position, [:pointer, :int], :void)
  attach_function(:set_read_position, :set_read_position, [:pointer, :int], :void)
  attach_function(:get_write_position, :get_write_position, [:pointer], :int)
  attach_function(:get_read_position, :get_read_position, [:pointer], :int)

  attach_function(:reset_cursors, :reset_cursors, [:pointer], :void)
  attach_function(:reset_bits_cursor, :reset_bits_cursor, [:pointer], :void)
  attach_function(:resize, :resize, [:pointer, :int], :void)

  attach_function(:clear, :clear, [:pointer], :void)
  attach_function(:is_empty?, :is_empty, [:pointer], :bool)

  attach_function(:read_bit, :read_bit, [:pointer], :bool)
  attach_function(:read_bits, :read_bits, [:pointer, :int], :long)
  attach_function(:read_u8, :read_u8, [:pointer], :int)
  attach_function(:read_i8, :read_i8, [:pointer], :int)
  attach_function(:read_u16, :read_u16, [:pointer], :int)
  attach_function(:read_i16, :read_i16, [:pointer], :int)
  attach_function(:read_u32, :read_u32, [:pointer], :int)
  attach_function(:read_i32, :read_i32, [:pointer], :int)
  attach_function(:read_u64, :read_u64, [:pointer], :long)
  attach_function(:read_i64, :read_i64, [:pointer], :long)
  attach_function(:read_f32, :read_f32, [:pointer], :float)
  attach_function(:read_f64, :read_f64, [:pointer], :double)

  attach_function(:write_bit, :write_bit, [:pointer, :bool], :void)
  attach_function(:write_bits, :write_bits, [:pointer, :int, :int], :void)
  attach_function(:write_u8, :write_u8, [:pointer, :int], :void)
  attach_function(:write_i8, :write_i8, [:pointer, :int], :void)
  attach_function(:write_u16, :write_u16, [:pointer, :int], :void)
  attach_function(:write_i16, :write_i16, [:pointer, :int], :void)
  attach_function(:write_u32, :write_u32, [:pointer, :int], :void)
  attach_function(:write_i32, :write_i32, [:pointer, :int], :void)
  attach_function(:write_u64, :write_u64, [:pointer, :long], :void)
  attach_function(:write_i64, :write_i64, [:pointer, :long], :void)
  attach_function(:write_f32, :write_f32, [:pointer, :float],  :void)
  attach_function(:write_f64, :write_f64, [:pointer,  :double],  :void)
end

# A {ByteBuffer} provides functions for interacting with buffered IO data. This gem wraps the {bytebuffer} rust crate located at https://github.com/terahlunah/bytebuffer using FFI.
class ByteBuffer



  # Constructs a new ByteBuffer.
  def initialize; @ptr = ByteBufferExtension.new; end

  # Returns the length of the ByteBuffer.
  # @return [Integer] the length
  def length; ByteBufferExtension.length(@ptr); end

  # Resets the read and write cursor positions.
  def reset_cursors; ByteBufferExtension.reset_cursors(@ptr); end

  # Resets the bit cursor position.
  def reset_bits_cursor; ByteBufferExtension.reset_bits_cursor(@ptr); end

  # Clear all bytes from the {ByteBuffer}.
  def clear; ByteBufferExtension.clear(@ptr); end

  # Is the {ByteBuffer} empty?
  # @return [Boolean] true if the {ByteBuffer} is empty, false otherwise.
  def is_empty?; ByteBufferExtension.is_empty?(@ptr); end

  # Resizes the {ByteBuffer} to the desired length.
  # @note {ByteBuffer}s can only increase in size. Lower or negative sizes will not work.
  # @param new_size [Integer] the desired length.
  def resize(new_size)
    if new_size.negative?
      raise ArgumentError, "new_size must be positive"
    else
      ByteBufferExtension.resize(@ptr, new_size)
    end
  end

  # Frees the memory allocated for this {ByteBuffer}.
  # @note This function will invoke the drop() macro in rust, passing in the pointer for the {ByteBuffer}. After the call is made, the pointer is set to `nil`.
  def free
    ByteBufferExtension.drop(@ptr)
    @ptr = nil # :gottem:
  end

  # Set the write cursor to the desired position.
  # @param position [Integer] the desired position.
  def set_write_position(position); ByteBufferExtension.set_write_position(@ptr, position); end

  # Set the read cursor to the desired position.
  # @param position [Integer] the desired position.
  def set_read_position(position); ByteBufferExtension.set_read_position(@ptr, position); end

  # Gets the write cursor's current position.
  # @return [Integer] the write cursor's position.
  def get_write_position; ByteBufferExtension.get_write_position(@ptr); end

  # Gets the read cursor's current position.
  # @return [Integer] the read cursor's position.
  def get_read_position; ByteBufferExtension.get_read_position(@ptr); end

  # Write a boolean value as a single bit to the {ByteBuffer}.
  # @param value [Boolean] the value to write.
  def write_bit(value); ByteBufferExtension.write_bit(@ptr, value); end

  # Write a numeric value as a series of bits in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  # @param amount [Integer] the amount of bits to use.
  def write_bits(value, amount); ByteBufferExtension.write_bits(@ptr, value, amount); end

  # Write a numeric value as a 8-bit unsigned integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_u8(value); ByteBufferExtension.write_u8(@ptr, value); end

  # Write a numeric value as a 8-bit signed integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_i8(value); ByteBufferExtension.write_i8(@ptr, value); end

  # Write a numeric value as a 16-bit unsigned integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_u16(value); ByteBufferExtension.write_u16(@ptr, value); end

  # Write a numeric value as a 16-bit signed integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_i16(value); ByteBufferExtension.write_i16(@ptr, value); end

  # Write a numeric value as a 32-bit unsigned integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_u32(value); ByteBufferExtension.write_u32(@ptr, value); end

  # Write a numeric value as a 32-bit signed integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_i32(value); ByteBufferExtension.write_i32(@ptr, value); end

  # Write a numeric value as a 64-bit unsigned integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_u64(value); ByteBufferExtension.write_u64(@ptr, value); end

  # Write a numeric value as a 64-bit signed integer in the {ByteBuffer}.
  # @param value [Integer] the value to write.
  def write_i64(value); ByteBufferExtension.write_i64(@ptr, value); end

  # Write a float value as a 32-bit signed float in the {ByteBuffer}.
  # @note The C value used is accurate to the nearest millionth decimal place.
  # @param value [Float] the value to write.
  def write_f32(value); ByteBufferExtension.write_f32(@ptr, value); end

  # Write a float value as a 64-bit signed float in the {ByteBuffer}.
  # @note The C value used is accurate to the nearest millionth decimal place.
  # @param value [Float] the value to write.
  def write_f64(value); ByteBufferExtension.write_f64(@ptr, value); end

  # Read a single bit from the {ByteBuffer} as a boolean value.
  # @return [Boolean] the read value.
  def read_bit; ByteBufferExtension.read_bit(@ptr); end

  # Read a series of bits from the {ByteBuffer} as a single unsigned 64-bit integer.
  # @return [Integer] the read value.
  def read_bits(amount); ByteBufferExtension.read_bits(@ptr, amount); end

  # Read a single 8-bit unsigned integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_u8; ByteBufferExtension.read_u8(@ptr); end

  # Read a single 8-bit signed integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_i8; ByteBufferExtension.read_i8(@ptr); end

  # Read a single 16-bit unsigned integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_u16; ByteBufferExtension.read_u16(@ptr); end

  # Read a single 16-bit signed integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_i16; ByteBufferExtension.read_i16(@ptr); end

  # Read a single 32-bit unsigned integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_u32; ByteBufferExtension.read_u32(@ptr); end

  # Read a single 32-bit signed integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_i32; ByteBufferExtension.read_i32(@ptr); end

  # Read a single 64-bit unsigned integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_u64; ByteBufferExtension.read_u64(@ptr); end

  # Read a single 64-bit signed integer from the {ByteBuffer}.
  # @return [Integer] the read value.
  def read_i64; ByteBufferExtension.read_i64(@ptr); end

  # Read a single 32-bit signed float from the {ByteBuffer}.
  # @note Float values read from this buffer are accurate to the millionth decimal place.
  # @return [Float] the read value.
  def read_f32; ByteBufferExtension.read_f32(@ptr); end

  # Read a single 32-bit signed float from the {ByteBuffer}.
  # @note Float values read from this buffer are accurate to the millionth decimal place.
  # @return [Float] the read value.
  def read_f64; ByteBufferExtension.read_f64(@ptr); end

  alias_method :drop, :free
  alias_method :destroy, :free
end
