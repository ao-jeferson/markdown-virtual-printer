//! Local print-spool protocol between a CUPS backend and the logged-in user app.
//! The socket is local-only, bounded, and never accepts a destination path.

use std::io::{self, Read, Write};

pub const MAX_JOB_BYTES: usize = 256 * 1024 * 1024;
const MAGIC: &[u8; 4] = b"MDVP";

#[derive(Debug)]
pub struct JobRequest {
    pub content_type: String,
    pub title: String,
    pub mode: u8,
    pub data: Vec<u8>,
}

pub fn write_request<W: Write>(writer: &mut W, request: &JobRequest) -> io::Result<()> {
    let content_type = request.content_type.as_bytes();
    let title = request.title.as_bytes();
    if request.data.len() > MAX_JOB_BYTES
        || content_type.len() > u16::MAX as usize
        || title.len() > u32::MAX as usize
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "print job exceeds IPC limits",
        ));
    }
    writer.write_all(MAGIC)?;
    writer.write_all(&1_u16.to_be_bytes())?;
    writer.write_all(&(content_type.len() as u16).to_be_bytes())?;
    writer.write_all(&(title.len() as u32).to_be_bytes())?;
    writer.write_all(&[request.mode])?;
    writer.write_all(&(request.data.len() as u64).to_be_bytes())?;
    writer.write_all(content_type)?;
    writer.write_all(title)?;
    writer.write_all(&request.data)?;
    writer.flush()
}

pub fn read_request<R: Read>(reader: &mut R) -> io::Result<JobRequest> {
    let mut magic = [0; 4];
    reader.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid IPC signature",
        ));
    }
    let version = read_u16(reader)?;
    if version != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsupported IPC version",
        ));
    }
    let content_type_len = read_u16(reader)? as usize;
    let title_len = read_u32(reader)? as usize;
    let mut mode = [0];
    reader.read_exact(&mut mode)?;
    let data_len = read_u64(reader)? as usize;
    if data_len > MAX_JOB_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "print job exceeds IPC limit",
        ));
    }
    let mut content_type = vec![0; content_type_len];
    let mut title = vec![0; title_len];
    let mut data = vec![0; data_len];
    reader.read_exact(&mut content_type)?;
    reader.read_exact(&mut title)?;
    reader.read_exact(&mut data)?;
    Ok(JobRequest {
        content_type: String::from_utf8(content_type)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "content type is not UTF-8"))?,
        title: String::from_utf8_lossy(&title).into_owned(),
        mode: mode[0],
        data,
    })
}

pub fn write_response<W: Write>(writer: &mut W, result: io::Result<()>) -> io::Result<()> {
    match result {
        Ok(()) => writer.write_all(&[0]),
        Err(error) => {
            let message = error.to_string();
            let bytes = message.as_bytes();
            writer.write_all(&[1])?;
            writer.write_all(&(bytes.len().min(u32::MAX as usize) as u32).to_be_bytes())?;
            writer.write_all(&bytes[..bytes.len().min(u32::MAX as usize)])
        }
    }
}

pub fn read_response<R: Read>(reader: &mut R) -> io::Result<()> {
    let mut status = [0];
    reader.read_exact(&mut status)?;
    if status[0] == 0 {
        return Ok(());
    }
    let len = read_u32(reader)? as usize;
    if len > 16 * 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "IPC error response too large",
        ));
    }
    let mut message = vec![0; len];
    reader.read_exact(&mut message)?;
    Err(io::Error::other(
        String::from_utf8_lossy(&message).into_owned(),
    ))
}

fn read_u16<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut b = [0; 2];
    reader.read_exact(&mut b)?;
    Ok(u16::from_be_bytes(b))
}
fn read_u32<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut b = [0; 4];
    reader.read_exact(&mut b)?;
    Ok(u32::from_be_bytes(b))
}
fn read_u64<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut b = [0; 8];
    reader.read_exact(&mut b)?;
    Ok(u64::from_be_bytes(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn request_round_trips() {
        let input = JobRequest {
            content_type: "application/pdf".into(),
            title: "Job 1".into(),
            mode: 1,
            data: b"%PDF-data".to_vec(),
        };
        let mut buffer = Vec::new();
        write_request(&mut buffer, &input).unwrap();
        let output = read_request(&mut Cursor::new(buffer)).unwrap();
        assert_eq!(output.content_type, input.content_type);
        assert_eq!(output.title, input.title);
        assert_eq!(output.mode, input.mode);
        assert_eq!(output.data, input.data);
    }
}
