require_relative 'spec_helper'

RSpec.describe ByteBuffer do

  after(:each) { @buffer.free }
  
  describe '#new' do
    it 'creates a new buffer' do
      @buffer = described_class.new
      expect(@buffer.class).to eql(ByteBuffer)
    end
  end

  describe '#length' do
    it 'returns the length of the buffer' do
      @buffer = described_class.new
      expect(@buffer.length).to eql(0)
    end
  end

  describe '#reset_cursors' do
    it 'resets the write cursor' do
      @buffer = described_class.new
      @buffer.resize(100)
      @buffer.set_write_position(24)
      expect(@buffer.get_write_position).to eql(24)
      @buffer.reset_cursors
      expect(@buffer.get_write_position).to eql(0)
    end

    it 'resets the read cursor' do
      @buffer = described_class.new
      @buffer.resize(100)
      @buffer.set_read_position(56)
      expect(@buffer.get_read_position).to eql(56)
      @buffer.reset_cursors
      expect(@buffer.get_read_position).to eql(0)
    end
  end

  describe '#clear' do
    it 'clears the buffer' do
      @buffer = described_class.new
      @buffer.resize(25)
      expect(@buffer.length).to eql(25)
      @buffer.clear
      expect(@buffer.length).to eql(0)
    end
  end

  describe '#is_empty?' do
    it 'returns true if the Buffer is empty' do
      @buffer = described_class.new
      expect(@buffer.is_empty?).to eql(true)
    end
  end

  describe '#resize' do
    it 'resizes the Buffer' do
      @buffer = described_class.new
      @buffer.resize(25)
      expect(@buffer.length).to eql(25)
    end
  end

  describe '#set_read_position' do
    it 'sets the read position' do
      @buffer = described_class.new
      @buffer.resize(100)
      @buffer.set_read_position(10)
      expect(@buffer.get_read_position).to eql(10)
    end
  end

  describe '#set_write_position' do
    it 'sets the write position' do
      @buffer = described_class.new
      @buffer.resize(100)
      @buffer.set_write_position(10)
      expect(@buffer.get_write_position).to eql(10)
    end
  end

  describe '#get_read_position' do
    it 'returns the read position' do
      @buffer = described_class.new
      @buffer.resize(100)
      @buffer.set_read_position(10)
      expect(@buffer.get_read_position).to eql(10)
    end
  end

  describe '#get_write_position' do
    it 'returns the write position' do
      @buffer = described_class.new
      @buffer.resize(100)
      @buffer.set_write_position(10)
      expect(@buffer.get_write_position).to eql(10)
    end
  end

  describe '#write_bit' do
    it 'writes a single boolean value to a single bit in the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_bit(true)
      expect(@buffer.length).to eql(1)
    end
  end

  describe '#write_u8' do
    it 'writes an unsigned 8-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u8(0xff)
      expect(@buffer.length).to eql(1)
    end
  end

  describe '#write_i8' do
    it 'writes an signed 8-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i8(-200)
      expect(@buffer.length).to eql(1)
    end
  end

  describe '#write_u16' do
    it 'writes an unsigned 16-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u16(0xffff)
      expect(@buffer.length).to eql(2)
    end
  end

  describe '#write_i16' do
    it 'writes an signed 16-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i16(-32000)
      expect(@buffer.length).to eql(2)
    end
  end

  describe '#write_u32' do
    it 'writes an unsigned 32-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u32(0xffffff)
      expect(@buffer.length).to eql(4)
    end
  end

  describe '#write_i32' do
    it 'writes an signed 32-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i32(-2000000)
      expect(@buffer.length).to eql(4)
    end
  end

  describe '#write_u64' do
    it 'writes an unsigned 64-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u64(0xffffffff)
      expect(@buffer.length).to eql(8)
    end
  end

  describe '#write_i64' do
    it 'writes an signed 64-bit integer to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i64(-4000000000)
      expect(@buffer.length).to eql(8)
    end
  end

  describe '#write_f32' do
    it 'writes a single 32-bit float to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_f32(1.234567)
      @buffer.write_f32(3.456789)
      @buffer.write_f32(4.567891)
      @buffer.write_f32(5.678910)
      expect(@buffer.length).to eql(16)
    end
  end

  describe '#write_f64' do
    it 'writes a single 64-bit float to the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_f64(1.234567)
      @buffer.write_f64(3.456789)
      @buffer.write_f64(4.567891)
      @buffer.write_f64(5.678910)

      expect(@buffer.length).to eql(32)
    end
  end

  describe '#read_bit' do
    it 'reads a single boolean value from a single bit in the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_bit(true)
      @buffer.write_bit(false)
      @buffer.write_bit(true)
      @buffer.write_bit(true)

      expect(@buffer.read_bit).to eql(true)
      expect(@buffer.read_bit).to eql(false)
      expect(@buffer.read_bit).to eql(true)
      expect(@buffer.read_bit).to eql(true)
    end
  end

  describe '#read_u8' do
    it 'reads a single unsigned 8-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u8(123)
      @buffer.write_u8(231)
      @buffer.write_u8(123)
      @buffer.write_u8(231)

      expect(@buffer.read_u8).to eql(123)
      expect(@buffer.read_u8).to eql(231)
      expect(@buffer.read_u8).to eql(123)
      expect(@buffer.read_u8).to eql(231)
    end
  end

  describe '#read_i8' do
    it 'reads a single signed 8-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i8(-123)
      @buffer.write_i8(-116)
      @buffer.write_i8(-64)
      @buffer.write_i8(-99)

      expect(@buffer.read_i8).to eql(-123)
      expect(@buffer.read_i8).to eql(-116)
      expect(@buffer.read_i8).to eql(-64)
      expect(@buffer.read_i8).to eql(-99)
    end
  end

  describe '#read_u16' do
    it 'reads a single unsigned 16-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u16(12_300)
      @buffer.write_u16(23_100)
      @buffer.write_u16(12_300)
      @buffer.write_u16(23_100)

      expect(@buffer.read_u16).to eql(12_300)
      expect(@buffer.read_u16).to eql(23_100)
      expect(@buffer.read_u16).to eql(12_300)
      expect(@buffer.read_u16).to eql(23_100)
    end
  end

  describe '#read_i16' do
    it 'reads a single signed 16-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i16(-12_300)
      @buffer.write_i16(-11_600)
      @buffer.write_i16(-640)
      @buffer.write_i16(-990)

      expect(@buffer.read_i16).to eql(-12_300)
      expect(@buffer.read_i16).to eql(-11_600)
      expect(@buffer.read_i16).to eql(-640)
      expect(@buffer.read_i16).to eql(-990)
    end
  end

  describe '#read_u32' do
    it 'reads a single unsigned 32-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u32(123_999)
      @buffer.write_u32(231_941)
      @buffer.write_u32(123_927)
      @buffer.write_u32(231_178)

      expect(@buffer.read_u32).to eql(123_999)
      expect(@buffer.read_u32).to eql(231_941)
      expect(@buffer.read_u32).to eql(123_927)
      expect(@buffer.read_u32).to eql(231_178)
    end
  end

  describe '#read_i32' do
    it 'reads a single signed 32-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i32(-123_999)
      @buffer.write_i32(-116_981)
      @buffer.write_i32(-64_892)
      @buffer.write_i32(-99_982)

      expect(@buffer.read_i32).to eql(-123_999)
      expect(@buffer.read_i32).to eql(-116_981)
      expect(@buffer.read_i32).to eql(-64_892)
      expect(@buffer.read_i32).to eql(-99_982)
    end
  end

  describe '#read_u64' do
    it 'reads a single unsigned 64-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_u64(12_399_928_999)
      @buffer.write_u64(23_192_892_200)
      @buffer.write_u64(12_923_928_777)
      @buffer.write_u64(23_282_900_172)

      expect(@buffer.read_u64).to eql(12_399_928_999)
      expect(@buffer.read_u64).to eql(23_192_892_200)
      expect(@buffer.read_u64).to eql(12_923_928_777)
      expect(@buffer.read_u64).to eql(23_282_900_172)
    end
  end

  describe '#read_i64' do
    it 'reads a single signed 8-bit integer from the ByteBuffer' do
      @buffer = described_class.new
      @buffer.write_i64(-12_399_928_999)
      @buffer.write_i64(-23_192_892_200)
      @buffer.write_i64(-12_923_928_777)
      @buffer.write_i64(-23_282_900_172)

      expect(@buffer.read_i64).to eql(-12_399_928_999)
      expect(@buffer.read_i64).to eql(-23_192_892_200)
      expect(@buffer.read_i64).to eql(-12_923_928_777)
      expect(@buffer.read_i64).to eql(-23_282_900_172)
    end
  end

  describe '#read_f32' do
    it 'reads a single 32-bit float from the ByteBuffer accurate to the 6th decimal place' do
      @buffer = described_class.new
      @buffer.write_f32(1.234567)
      @buffer.write_f32(3.456789)
      @buffer.write_f32(4.567891)
      @buffer.write_f32(5.678910)

      expect(@buffer.read_f32.round(6)).to eql(1.234567)
      expect(@buffer.read_f32.round(6)).to eql(3.456789)
      expect(@buffer.read_f32.round(6)).to eql(4.567891)
      expect(@buffer.read_f32.round(6)).to eql(5.678910)
    end
  end

  describe '#read_f64' do
    it 'reads a single 64-bit float from the ByteBuffer accurate to the 6th decimal place' do
      @buffer = described_class.new
      @buffer.write_f64(1.234567)
      @buffer.write_f64(3.456789)
      @buffer.write_f64(4.567891)
      @buffer.write_f64(5.678910)

      expect(@buffer.read_f64.round(6)).to eql(1.234567)
      expect(@buffer.read_f64.round(6)).to eql(3.456789)
      expect(@buffer.read_f64.round(6)).to eql(4.567891)
      expect(@buffer.read_f64.round(6)).to eql(5.678910)
    end
  end
end