extern crate byteorder;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate genet_sdk;

#[macro_use]
extern crate serde_derive;

use byteorder::{LittleEndian, WriteBytesExt};
use genet_sdk::{
    io::{Writer, WriterWorker},
    prelude::*,
};

use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Deserialize)]
struct Arg {
    file: String,
}

#[derive(Clone)]
struct PcapFileWriter {}

impl Writer for PcapFileWriter {
    fn new_worker(&self, _ctx: &Context, arg: &str) -> Result<Box<WriterWorker>> {
        let arg: Arg = serde_json::from_str(arg)?;
        let file = File::create(&arg.file)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&[0x4d, 0x3c, 0xb2, 0xa1])?;
        Ok(Box::new(PcapFileWriterWorker {
            writer,
            header: false,
        }))
    }

    fn id(&self) -> &str {
        "pcap-file"
    }
}

struct PcapFileWriterWorker {
    writer: BufWriter<File>,
    header: bool,
}

impl PcapFileWriterWorker {
    fn write_header(&mut self, snaplen: u32, network: u32) -> Result<()> {
        if !self.header {
            self.header = true;
            let var_major = 2u16;
            let var_minor = 4u16;
            let thiszone = 0i32;
            let sigfigs = 0u32;
            self.writer.write_u16::<LittleEndian>(var_major)?;
            self.writer.write_u16::<LittleEndian>(var_minor)?;
            self.writer.write_i32::<LittleEndian>(thiszone)?;
            self.writer.write_u32::<LittleEndian>(sigfigs)?;
            self.writer.write_u32::<LittleEndian>(snaplen)?;
            self.writer.write_u32::<LittleEndian>(network)?;
        }
        Ok(())
    }
}

impl WriterWorker for PcapFileWriterWorker {
    fn write(&mut self, _index: u32, stack: &LayerStack) -> Result<()> {
        if let Some(layer) = stack.bottom() {
            let incl_len = layer.data().len();
            let mut orig_len = 0;
            let mut ts_sec = 0;
            let mut ts_usec = 0;
            let mut link = 0;

            if let Some(attr) = layer.attr(token!("link.length")) {
                orig_len = attr.try_get(&layer.data())?.try_into()?;
            }
            if let Some(attr) = layer.attr(token!("link.type")) {
                link = attr.try_get(&layer.data())?.try_into()?;
            }
            if let Some(attr) = layer.attr(token!("link.timestamp.sec")) {
                ts_sec = attr.try_get(&layer.data())?.try_into()?;
            }
            if let Some(attr) = layer.attr(token!("link.timestamp.usec")) {
                ts_usec = attr.try_get(&layer.data())?.try_into()?;
            }

            self.write_header(0, link as u32)?;

            self.writer.write_u32::<LittleEndian>(ts_sec as u32)?;
            self.writer.write_u32::<LittleEndian>(ts_usec as u32)?;
            self.writer.write_u32::<LittleEndian>(incl_len as u32)?;
            self.writer.write_u32::<LittleEndian>(orig_len as u32)?;
            self.writer.write_all(&layer.data())?;
        }
        Ok(())
    }
}

genet_writers!(PcapFileWriter {});
