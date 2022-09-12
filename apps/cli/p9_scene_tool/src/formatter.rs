use serde::ser::Serialize;
use serde_json::Result as JsonResult;
use serde_json::ser::{Formatter, PrettyFormatter, Serializer};
use std::io;

#[derive(Default)]
pub struct P9Formatter<'a> {
    base: PrettyFormatter<'a>,
    primitive_array_context: PrimitiveArrayContext,
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
    dummy_writer: DummyWriter
}

#[derive(Default)]
struct DummyWriter;

impl io::Write for DummyWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// TODO: Support more types
enum PrimitiveArrayContext {
    Start,
    F32(Vec<f32>),
    End
}

impl Default for PrimitiveArrayContext {
    fn default() -> Self {
        PrimitiveArrayContext::End
    }
}

impl PrimitiveArrayContext {
    fn reset(&mut self) {
        *self = PrimitiveArrayContext::End
    }

    fn begin(&mut self) {
        *self = PrimitiveArrayContext::Start
    }

    fn at_start(&self) -> bool {
        match self {
            PrimitiveArrayContext::Start => true,
            _ => false
        }
    }

    fn in_progress(&self) -> bool {
        match self {
            PrimitiveArrayContext::End => false,
            _ => true
        }
    }

    fn add_f32(&mut self, value: f32) {
        if !self.in_progress() {
            panic!("Working stream has already ended")
        }

        // Init stream
        if self.at_start() {
            self.init_f32_stream();
        }

        if let Some(stream) = self.get_mut_f32_array() {
            stream.push(value)
        } else {
            panic!("Primitive type doesn't match add")
        }
    }

    fn init_f32_stream(&mut self) {
        if !self.at_start() {
            panic!("Can't init stream at already in progress")
        }

        *self = PrimitiveArrayContext::F32(Vec::new())
    }

    fn get_mut_f32_array<'a>(&'a mut self) -> Option<&'a mut Vec<f32>> {
        match self {
            PrimitiveArrayContext::F32(stream) => Some(stream),
            _ => None
        }
    }
}

impl<'a> P9Formatter<'a> {
    pub fn new() -> Self {
        P9Formatter {
            base: PrettyFormatter::new(),
            indent: b"  ",
            ..P9Formatter::default()
        }
    }

    fn indent<W: ?Sized + io::Write>(&self, writer: &mut W) -> io::Result<()> {
        for _ in 0..self.current_indent {
            writer.write_all(self.indent)?;
        }

        Ok(())
    }

    fn increment_indent(&mut self) {
        self.current_indent += 1;
        self.has_value = false;
        self.base.begin_array(&mut self.dummy_writer).unwrap();
    }

    fn decrement_indent(&mut self) {
        self.current_indent -= 1;
        self.base.end_array(&mut self.dummy_writer).unwrap();
    }

    fn cancel_primitive_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if self.primitive_array_context.in_progress() {
            self.primitive_array_context.reset();
            self.base.begin_array_value(writer, !self.has_value)
        } else {
            Ok(())
        }
    }
}

impl<'a> Formatter for P9Formatter<'a> {
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_null(writer)
    }

    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_bool(writer, value)
    }

    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_i8(writer, value)
    }

    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_i16(writer, value)
    }

    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_i32(writer, value)
    }

    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_i64(writer, value)
    }

    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_u8(writer, value)
    }

    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_u16(writer, value)
    }

    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_u32(writer, value)
    }

    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_u64(writer, value)
    }

    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.primitive_array_context.in_progress() {
            self.primitive_array_context.add_f32(value);
            Ok(())
        } else {
            self.base.write_f32(writer, value)
        }
    }

    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_f64(writer, value)
    }

    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_number_str(writer, value)
    }

    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.begin_string(writer)
    }

    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.base.end_string(writer)
    }

    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_string_fragment(writer, fragment)
    }

    fn write_char_escape<W>(&mut self, writer: &mut W, char_escape: serde_json::ser::CharEscape) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.cancel_primitive_array(writer)?;
        self.base.write_char_escape(writer, char_escape)
    }

    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.primitive_array_context.begin();
        self.increment_indent();

        writer.write_all(b"[")
    }

    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.has_value && self.primitive_array_context.in_progress() && !self.primitive_array_context.at_start() {
            // Write flat array stream
            // TODO: Support other types
            if let Some(stream) = self.primitive_array_context.get_mut_f32_array() {
                while let Some(value) = stream.pop() {
                    writer.write_all(b" ")?;
                    self.base.write_f32(writer, value)?;

                    if !stream.is_empty() {
                        writer.write_all(b",")?;
                    } else {
                        writer.write_all(b" ")?;
                    }
                }
            }

            self.primitive_array_context.reset();
            self.decrement_indent();

            writer.write_all(b"]")
        } else {
            self.current_indent -= 1;
            self.primitive_array_context.reset();
            self.base.end_array(writer)
        }
    }

    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.primitive_array_context.in_progress() {
            Ok(())
        } else {
            self.base.begin_array_value(writer, first)
        }
    }

    fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        self.base.end_array_value(writer)
    }

    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;

        self.cancel_primitive_array(writer)?;
        self.base.begin_object(writer)
    }

    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;
        self.base.end_object(writer)
    }

    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.base.begin_object_key(writer, first)
    }

    fn end_object_key<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.base.end_object_key(writer)
    }

    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.base.begin_object_value(writer)
    }

    fn end_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        self.base.end_object_value(writer)
    }

    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.base.write_raw_fragment(writer, fragment)
    }
}

pub fn to_string<T: ?Sized + Serialize>(value: &T) -> JsonResult<String> {
    let buffer = Vec::new();
    let formatter = P9Formatter::new();

    let mut serializer = Serializer::with_formatter(buffer, formatter);
    value.serialize(&mut serializer)?;

    Ok(String::from_utf8(serializer.into_inner()).unwrap())
}