pub mod utilities {
    use byteorder::{ByteOrder, LittleEndian};
    use chrono::*;
    use std::convert::TryInto;
    use std::io::{self, prelude::*, Seek, SeekFrom};

    pub fn convert_to_utc_time(to_convert: [u8; 4]) -> chrono::DateTime<chrono::Utc> {
        //convert unix time to current time
        let timestamp = LittleEndian::read_u32(&to_convert);
        let naive = NaiveDateTime::from_timestamp(timestamp.try_into().unwrap(), 0);

        // Create a normal DateTime from the NaiveDateTime
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        // Format the datetime how you want
        return datetime;
    }

    pub fn seek_read(mut reader: impl Read + Seek, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        reader.seek(SeekFrom::Start(offset))?;
        reader.read_exact(buf)?;
        Ok(())
    }

    pub fn seek_write(
        mut reader: impl Write + Seek,
        offset: u128,
        buf: &mut [u8],
    ) -> io::Result<()> {
        reader.seek(SeekFrom::Start(offset.try_into().unwrap()))?;
        reader.write(buf)?;
        Ok(())
    }

    pub fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
