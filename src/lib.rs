extern crate quick_xml;

use quick_xml::*;
use quick_xml::events::*;

use std::path::*;
use std::io::*;
use std::fs::*;

#[derive(Debug)]
pub enum Error {
    Xml(quick_xml::Error),
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_dir(dir: &AsRef<Path>, target: &AsRef<Path>) -> Result<()> {
    let target = File::create(target).map_err(Error::Io)?;
    let mut writer = Writer::new(target);
    write_start(&mut writer).map_err(Error::Xml)?;
    for entry in read_dir(dir).map_err(Error::Io)? {
        let dir = entry.map_err(Error::Io)?;
        if !dir.path().is_file() {
            continue;
        }
        let mut reader = Reader::from_file(dir.path()).map_err(Error::Xml)?;
        reader.trim_text(true);
        let mut buf = Vec::new();
        write_symbol_start(&mut writer, dir.path().file_name().unwrap_or_default().to_str().unwrap_or_default()).map_err(Error::Xml)?;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) => {
                    if e.name() != b"?xml" {
                        // writes the event to the writer
                        writer.write_event(Event::Start(e)).map_err(Error::Xml)?;
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name() != b"?xml" {
                        writer.write_event(Event::End(e)).map_err(Error::Xml)?;
                    }
                }
                Ok(Event::Eof) => break,
                // you can use either `e` or `&e` if you don't want to move the event
                Ok(e) => {
                    writer.write_event(&e).map_err(Error::Xml)?;
                },
                Err(e) => return Err(Error::Xml(e)),
            }
            buf.clear();
        }
        write_symbol_end(&mut writer).map_err(Error::Xml)?;
    }
    write_end(&mut writer).map_err(Error::Xml)?;
    Ok(())
}

fn write_start<T: Write>(writer: &mut Writer<T>) -> quick_xml::Result<usize> {
    let mut elem = BytesStart::borrowed_name(b"svg");
    elem.push_attribute(("xmlns", "http://www.w3.org/2000/svg"));
    writer.write_event(Event::Start(elem))?;
    writer.write_event(Event::Start(BytesStart::borrowed_name(b"defs")))
}

fn write_end<T: Write>(writer: &mut Writer<T>) -> quick_xml::Result<usize> {
    writer.write_event(Event::End(BytesEnd::borrowed(b"defs")))?;
    writer.write_event(Event::End(BytesEnd::borrowed(b"svg")))
}

fn write_symbol_start<T: Write>(writer: &mut Writer<T>, id: &str) -> quick_xml::Result<usize> {
    let mut elem = BytesStart::borrowed_name(b"symbol");
    elem.push_attribute(("id", id));
    writer.write_event(Event::Start(elem))
}

fn write_symbol_end<T: Write>(writer: &mut Writer<T>) -> quick_xml::Result<usize> {
    writer.write_event(Event::End(BytesEnd::borrowed(b"symbol")))
}

#[test]
fn test_() {
    parse_dir(&"./svgs", &"defs.svg").unwrap();
}