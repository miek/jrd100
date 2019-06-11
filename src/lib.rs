extern crate byteorder;

use byteorder::{WriteBytesExt, BigEndian};

const FRAME_HEADER: u8 = 0xBB;
const FRAME_END: u8 = 0x7E;

enum FrameType {
    Command = 0,
    Response = 1,
    Notification = 2,
}

pub struct JRD100 {

}

impl JRD100 {
    pub fn new() -> Self {
        JRD100{

        }
    }
    pub fn send_command(&self, command: u8, parameter: &[u8]) -> Result<(), Error> {
        let frame = JRD100::build_frame(FrameType::Command, command, parameter)?;
        Ok(())
    }

    fn build_frame(frame_type: FrameType, command: u8, parameter: &[u8]) -> Result<Vec<u8>, Error> {
        let mut packet = Vec::with_capacity(7 + parameter.len());
        packet.push(0); // placeholder for header 
        packet.push(frame_type as u8);
        packet.push(command);
        packet.write_u16::<BigEndian>(parameter.len() as u16)?;
        packet.extend_from_slice(&parameter);

        let checksum = packet.iter().sum(); 
        packet.push(checksum);
        packet.push(FRAME_END);

        packet[0] = FRAME_HEADER;
        Ok(packet)
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_frame() {
        let f = JRD100::build_frame(FrameType::Command, 0x07, &[0x01]).unwrap();
        assert_eq!(f, vec![0xBB, 0x00, 0x07, 0x00, 0x01, 0x01, 0x09, 0x7E]);
    }

    #[test]
    fn test_build_frame_empty() {
        let f = JRD100::build_frame(FrameType::Command, 0x22, &[]).unwrap();
        assert_eq!(f, vec![0xBB, 0x00, 0x22, 0x00, 0x00, 0x22, 0x7E]);
    }
}
