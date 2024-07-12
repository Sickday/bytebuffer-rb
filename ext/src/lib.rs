use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use bytebuffer::{ByteBuffer, Endian};

#[no_mangle]
extern "C" fn new() -> *mut ByteBuffer {
	Box::into_raw(Box::new(ByteBuffer::new()))
}

#[no_mangle]
extern "C" fn from_bytes(bytes: *const u8, len: usize) -> *mut ByteBuffer {
	let slice = unsafe {
		assert!(!bytes.is_null());
		std::slice::from_raw_parts(bytes, len)
	};
	Box::into_raw(Box::new(ByteBuffer::from_bytes(slice)))
}

#[no_mangle]
extern "C" fn drop(ptr: *mut ByteBuffer) {
	let _ = unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
extern "C" fn length(ptr: *mut ByteBuffer) -> usize {
	match unsafe { raw_pointer_to_ref(ptr) } {
		Some(buffer) => buffer.len(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn is_empty(ptr: *mut ByteBuffer) -> bool {
	match unsafe { raw_pointer_to_ref(ptr) } {
		Some(buffer) => buffer.is_empty(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn clear(ptr: *mut ByteBuffer) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.clear(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn reset_cursors(ptr: *mut ByteBuffer) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.reset_cursors(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn reset_bits_cursor(ptr: *mut ByteBuffer) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.reset_bits_cursors(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn resize(ptr: *mut ByteBuffer, size: usize) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.resize(size),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn flush_bits(ptr: *mut ByteBuffer) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.flush_bits(),
		None  => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn get_read_position(ptr: *mut ByteBuffer) -> usize {
	match unsafe { raw_pointer_to_ref(ptr) } {
		Some(buffer) => buffer.get_rpos(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn get_write_position(ptr: *mut ByteBuffer) -> usize {
	match unsafe { raw_pointer_to_ref(ptr) } {
		Some(buffer) => buffer.get_wpos(),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn set_write_position(ptr: *mut ByteBuffer, position: usize) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.set_wpos(position),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn set_read_position(ptr: *mut ByteBuffer, position: usize) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.set_rpos(position),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn set_endian(ptr: *mut ByteBuffer, endian: u8) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			let endian: Endian = endian_for_u8(endian);
			buffer.set_endian(endian);
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn get_endian(ptr: *mut ByteBuffer) -> u8 {
	match unsafe { raw_pointer_to_ref(ptr) } {
		Some(buffer) => {
			let endian: Endian = buffer.endian();
			u8_for_endian(endian)
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_bit(ptr: *mut ByteBuffer, bit: bool) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_bit(bit),
		None => panic!("Unable to locate buffer using ptr")
	}
}

extern "C" fn write_bits(ptr: *mut ByteBuffer, value: u64, bits: u8) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_bits(value, bits),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_u8(ptr: *mut ByteBuffer, val: u8) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_u8(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_i8(ptr: *mut ByteBuffer, val: i8) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_i8(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_u16(ptr: *mut ByteBuffer, val: u16) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_u16(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_i16(ptr: *mut ByteBuffer, val: i16) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_i16(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_u32(ptr: *mut ByteBuffer, val: u32) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_u32(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_i32(ptr: *mut ByteBuffer, val: i32) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_i32(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_u64(ptr: *mut ByteBuffer, val: u64) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_u64(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_i64(ptr: *mut ByteBuffer, val: i64) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_i64(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_f32(ptr: *mut ByteBuffer, val: f32) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_f32(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_f64(ptr: *mut ByteBuffer, val: f64) {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_f64(val),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_bytes(ptr: *mut ByteBuffer, bytes: *const u8, length: usize) {
	let slice = unsafe {
		assert!(!bytes.is_null());
		std::slice::from_raw_parts(bytes, length)
	};

	match unsafe  { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_bytes(slice),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn write_string(ptr: *mut ByteBuffer, c_string: *const c_char) {
	let raw_str: &CStr = unsafe { CStr::from_ptr(c_string) };
	let string: &str = raw_str.to_str().unwrap();

	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => buffer.write_string(string),
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_bits(ptr: *mut ByteBuffer, length: u8) -> u64 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => match buffer.read_bits(length) {
			Ok(value) => value,
			Err(error) => panic!("Unable to read bits from buffer: {}", error),
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_bit(ptr: *mut ByteBuffer) -> bool {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) =>
			match buffer.read_bit() {
				Ok(bit) => bit,
				Err(_) => panic!("Unable to read bit from buffer")
			},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_u8(ptr: *mut ByteBuffer) -> u8 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_u8() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read u8 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_i8(ptr: *mut ByteBuffer) -> i8 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_i8() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read i8 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_u16(ptr: *mut ByteBuffer) -> u16 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_u16() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read u16 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_i16(ptr: *mut ByteBuffer) -> i16 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_i16() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read i16 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_u32(ptr: *mut ByteBuffer) -> u32 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_u32() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read u32 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_i32(ptr: *mut ByteBuffer) -> i32 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_i32() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read i32 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_u64(ptr: *mut ByteBuffer) -> u64 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_u64() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read u64 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_i64(ptr: *mut ByteBuffer) -> i64 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_i64() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read i64 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_f32(ptr: *mut ByteBuffer) -> f32 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_f32() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read f32 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_f64(ptr: *mut ByteBuffer) -> f64 {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			match buffer.read_f64() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read f64 from buffer")
			}
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn read_string(ptr: *mut ByteBuffer) -> *const c_char {
	match unsafe { raw_pointer_to_ref_mut(ptr) } {
		Some(buffer) => {
			let str = match buffer.read_string() {
				Ok(value) => value,
				Err(_) => panic!("Unable to read string from buffer")
			};
			let cstr = CString::new(str).unwrap();
			cstr.into_raw()
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

#[no_mangle]
extern "C" fn to_hex_string(ptr: *mut ByteBuffer) -> *const c_char {
	match unsafe { raw_pointer_to_ref(ptr) } {
		Some(buffer) => {
			let dump: String = buffer.to_hex_dump();
			let c_s = CString::new(dump).unwrap();
			c_s.into_raw()
		},
		None => panic!("Unable to locate buffer using ptr")
	}
}

unsafe fn raw_pointer_to_ref(buffer: *mut ByteBuffer) -> Option<&'static ByteBuffer> {
	if buffer.is_null() {
		None
	} else {
		Some(&*buffer)
	}
}

unsafe fn raw_pointer_to_ref_mut(buffer: *mut ByteBuffer) -> Option<&'static mut ByteBuffer> {
	buffer.as_mut()
}

fn endian_for_u8(value: u8) -> Endian {
	match value {
		0 => Endian::BigEndian,
		1 => Endian::LittleEndian,
		_ => unreachable!(),
	}
}

fn u8_for_endian(endian: Endian) -> u8 {
	match endian {
		Endian::BigEndian => 0,
		Endian::LittleEndian => 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_bytebuffer() {
		let buffer_ref = new();
		assert!(!buffer_ref.is_null());
		drop(buffer_ref);
	}

	#[test]
	fn test_from_bytes() {
		let bytes = [0, 1, 2, 3, 4, 5];
		let buffer = from_bytes(bytes.as_ptr(), bytes.len());
		assert!(!buffer.is_null(), "ByteBuffer from bytes failed.");
		assert_eq!(length(buffer), bytes.len(), "Length of ByteBuffer mismatched with input bytes.");
		drop(buffer);
	}

	#[test]
	fn test_length() {
		let bytes = [0, 1, 2, 3, 4, 5];
		let buffer = from_bytes(bytes.as_ptr(), bytes.len());
		assert_eq!(length(buffer), bytes.len(), "Length of ByteBuffer mismatched with input bytes.");
		drop(buffer);
	}

	#[test]
	fn test_is_empty() {
		let empty_buffer = new();
		assert_eq!(is_empty(empty_buffer), true, "New ByteBuffer expected to be empty.");
		drop(empty_buffer);

		let bytes = [0, 1, 2, 3, 4, 5];
		let non_empty_buffer = from_bytes(bytes.as_ptr(), bytes.len());
		assert_eq!(is_empty(non_empty_buffer), false, "ByteBuffer from bytes should not be empty.");
		drop(non_empty_buffer);
	}

	#[test]
	fn test_clear() {
		let bytes = [0, 1, 2, 3, 4, 5];
		let buffer = from_bytes(bytes.as_ptr(), bytes.len());
		clear(buffer);
		assert_eq!(is_empty(buffer), true, "Cleared ByteBuffer expected to be empty.");
		drop(buffer);
	}

	#[test]
	fn test_drop() {
		let bytes = [0, 1, 2, 3, 4, 5];
		let buffer = from_bytes(bytes.as_ptr(), bytes.len());
		drop(buffer);
	}

	#[test]
	fn test_set_endian() {
		let buffer = new();
		assert_eq!(get_endian(buffer), u8_for_endian(Endian::BigEndian));

		set_endian(buffer, 1);
		assert_eq!(get_endian(buffer), u8_for_endian(Endian::LittleEndian));
	}
	
	#[test]
	fn test_get_endian() {
		let buffer = new();
		assert_eq!(get_endian(buffer), u8_for_endian(Endian::BigEndian))
	}

	#[test]
	fn test_write_u8() {
		let bytes = [0, 1, 2, 3, 4, 5];
		let buffer = from_bytes(bytes.as_ptr(), bytes.len());
		write_u8(buffer, 0);
		assert_eq!(length(buffer), 7, "ByteBuffer expected to have 1 byte written.");
	}

	#[test]
	fn test_write_u16() {
		let buffer = new();
		write_u16(buffer, 808);
		assert_eq!(length(buffer), 2, "ByteBuffer expected to have 2 bytes written.");
	}

	#[test]
	fn test_write_u32() {
		let buffer = new();
		write_u32(buffer, 65580);
		assert_eq!(length(buffer), 4, "ByteBuffer expected to have 4 bytes written.");
	}

	#[test]
	fn test_write_u64() {
		let buffer = new();
		write_u64(buffer, 2147888888);
		assert_eq!(length(buffer), 8, "ByteBuffer expected to have 8 bytes written.");
	}

	#[test]
	fn test_write_i8() {
		let buffer = new();
		write_i8(buffer, -48);
		assert_eq!(length(buffer), 1, "ByteBuffer expected to have 1 byte written.");
	}

	#[test]
	fn test_write_i16() {
		let buffer = new();
		write_i16(buffer, -808);
		assert_eq!(length(buffer), 2, "ByteBuffer expected to have 2 bytes written.");
	}

	#[test]
	fn test_write_i32() {
		let buffer = new();
		write_i32(buffer, -65580);
		assert_eq!(length(buffer), 4, "ByteBuffer expected to have 4 bytes written.");
	}

	#[test]
	fn test_write_i64() {
		let buffer = new();
		write_i64(buffer, -2147888888);
		assert_eq!(length(buffer), 8, "ByteBuffer expected to have 8 bytes written.");
	}

	#[test]
	fn test_write_bit() {
		let buffer = new();
		write_bit(buffer, true);
		write_bit(buffer, false);
		write_bit(buffer, true);
		assert_eq!(length(buffer), 1);
		write_bit(buffer, true);
		assert_eq!(length(buffer), 1);
		write_bit(buffer, false);
		assert_eq!(length(buffer), 1);
		write_bit(buffer, true);
		assert_eq!(length(buffer), 1);
		write_bit(buffer, false);
		assert_eq!(length(buffer), 1);
		write_bit(buffer, true);
		assert_eq!(length(buffer), 1);
		write_bit(buffer, false);
		assert_eq!(length(buffer), 2);
	}

	#[test]
	fn test_write_bits() {
		let buffer = new();
		write_bits(buffer, 96, 8);
		assert_eq!(length(buffer), 1);
	}
	
	#[test]
	fn test_write_f32() {
		let buffer = new();
		write_f32(buffer, -21478888.9848);
		assert_eq!(length(buffer), 4, "ByteBuffer expected to have 8 bytes written.");
	}

	#[test]
	fn test_write_f64() {
		let buffer = new();
		write_f64(buffer, -2147888888.3282);
		assert_eq!(length(buffer), 8, "ByteBuffer expected to have 8 bytes written.");
	}

	#[test]
	fn test_read_u8() {
		let buffer = new();
		write_u8(buffer, 127);
		write_u8(buffer, 100);
		write_u8(buffer, 32);
		assert_eq!(read_u8(buffer), 127, "ByteBuffer expected to have read 127.");
		assert_eq!(read_u8(buffer), 100,  "ByteBuffer expected to have read 100.");
		assert_eq!(read_u8(buffer), 32,  "ByteBuffer expected to have read 32.");
	}

	#[test]
	fn test_read_i8() {
		let buffer = new();
		write_i8(buffer, -127);
		write_i8(buffer, -100);
		write_i8(buffer, -32);
		assert_eq!(read_i8(buffer), -127, "ByteBuffer expected to have read -127.");
		assert_eq!(read_i8(buffer), -100,  "ByteBuffer expected to have read -100.");
		assert_eq!(read_i8(buffer), -32,  "ByteBuffer expected to have read -32.");
	}

	#[test]
	fn test_read_u16() {
		let buffer = new();
		write_u16(buffer, 21411);
		write_u16(buffer, 21499);
		write_u16(buffer, 21488);
		assert_eq!(read_u16(buffer), 21411, "ByteBuffer expected to have read 21411.");
		assert_eq!(read_u16(buffer), 21499, "ByteBuffer expected to have read 21499.");
		assert_eq!(read_u16(buffer), 21488, "ByteBuffer expected to have read 21488.");
	}

	#[test]
	fn test_read_i16() {
		let buffer = new();
		write_i16(buffer, -21411);
		write_i16(buffer, -21499);
		write_i16(buffer, -21488);
		assert_eq!(read_i16(buffer), -21411, "ByteBuffer expected to have read -21411.");
		assert_eq!(read_i16(buffer), -21499, "ByteBuffer expected to have read -21499.");
		assert_eq!(read_i16(buffer), -21488, "ByteBuffer expected to have read -21488.");
	}

	#[test]
	fn test_read_u32() {
		let buffer = new();
		write_u32(buffer, 214000000);
		write_u32(buffer, 214000099);
		write_u32(buffer, 214000098);
		assert_eq!(read_u32(buffer), 214000000, "ByteBuffer expected to have read 214000000.");
		assert_eq!(read_u32(buffer), 214000099, "ByteBuffer expected to have read 214000099.");
		assert_eq!(read_u32(buffer), 214000098, "ByteBuffer expected to have read 214000098.");
	}

	#[test]
	fn test_read_i32() {
		let buffer = new();
		write_i32(buffer, -214000000);
		write_i32(buffer, -214000099);
		write_i32(buffer, -214000098);
		assert_eq!(read_i32(buffer), -214000000, "ByteBuffer expected to have read -214000000.");
		assert_eq!(read_i32(buffer), -214000099, "ByteBuffer expected to have read -214000099.");
		assert_eq!(read_i32(buffer), -214000098, "ByteBuffer expected to have read -214000098.");
	}

	#[test]
	fn test_read_u64() {
		let buffer = new();
		write_u64(buffer, 2149000000);
		write_u64(buffer, 2149000099);
		write_u64(buffer, 2149000098);
		assert_eq!(read_i64(buffer), 2149000000, "ByteBuffer expected to have read 2149000000.");
		assert_eq!(read_i64(buffer), 2149000099, "ByteBuffer expected to have read 2149000099.");
		assert_eq!(read_i64(buffer), 2149000098, "ByteBuffer expected to have read 2149000098.");
	}

	#[test]
	fn test_read_i64() {
		let buffer = new();
		write_i64(buffer, -2149000000);
		write_i64(buffer, -2149000099);
		write_i64(buffer, -2149000098);
		assert_eq!(read_i64(buffer), -2149000000, "ByteBuffer expected to have read -2149000000.");
		assert_eq!(read_i64(buffer), -2149000099, "ByteBuffer expected to have read -2149000099.");
		assert_eq!(read_i64(buffer), -2149000098, "ByteBuffer expected to have read -2149000098.");
	}

	#[test]
	fn test_read_bit() {
		let buffer = new();
		write_bit(buffer, true);
		write_bit(buffer, false);
		write_bit(buffer, true);
		write_bit(buffer, true);

		assert!(read_bit(buffer));
		assert!(!read_bit(buffer));
		assert!(read_bit(buffer));
		assert!(read_bit(buffer));
	}

	#[test]
	fn test_read_bits() {
		let buffer = new();
		write_bits(buffer, 96, 8);
		assert_eq!(read_bits(buffer, 8), 96u64);
	}
}
