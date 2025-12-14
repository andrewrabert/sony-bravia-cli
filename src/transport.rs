use std::io::{Read, Write};
use std::time::Duration;
use thiserror::Error;

use crate::protocol::{
    Command, ProtocolError, RESPONSE_HEADER, ResponseCode, build_control_packet,
    build_query_packet, checksum,
};

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("serial port error: {0}")]
    Serial(#[from] serialport::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("timeout reading response")]
    Timeout,
    #[error("query not supported for this command")]
    QueryNotSupported,
    #[error("protocol error: {0}")]
    Protocol(#[from] ProtocolError),
    #[error("invalid checksum in response")]
    InvalidChecksum,
    #[error("unexpected response header: {0:#04x}")]
    UnexpectedHeader(u8),
    #[error("command rejected: {0:?}")]
    CommandRejected(ResponseCode),
}

pub struct Transport {
    port: Box<dyn serialport::SerialPort + Send>,
}

impl Transport {
    pub fn new(device_path: &str) -> Result<Self, TransportError> {
        let port = serialport::new(device_path, 9600)
            .timeout(Duration::from_millis(500))
            .open()?;
        Ok(Self { port })
    }

    pub fn execute<C: Command>(&mut self, action: &C::Action) -> Result<(), TransportError> {
        let data = C::build_bytes(action);
        let packet = build_control_packet(C::FUNCTION_CODE, &data);
        self.write_and_validate(&packet)?;
        Ok(())
    }

    pub fn query<C: Command>(&mut self) -> Result<C::Response, TransportError> {
        if !C::supports_query() {
            return Err(TransportError::QueryNotSupported);
        }
        let packet = build_query_packet(C::FUNCTION_CODE);
        let response_data = self.write_and_read(&packet)?;
        let response = C::parse_response(&response_data)?;
        Ok(response)
    }

    fn write_and_validate(&mut self, packet: &[u8]) -> Result<(), TransportError> {
        self.port.write_all(packet)?;

        let mut header_buf = [0u8; 3];
        self.port.read_exact(&mut header_buf).map_err(|e| {
            if e.kind() == std::io::ErrorKind::TimedOut {
                TransportError::Timeout
            } else {
                TransportError::Io(e)
            }
        })?;

        if header_buf[0] != RESPONSE_HEADER {
            return Err(TransportError::UnexpectedHeader(header_buf[0]));
        }

        let response_code = ResponseCode::try_from(header_buf[1])?;
        if response_code != ResponseCode::Success {
            return Err(TransportError::CommandRejected(response_code));
        }

        let checksum_byte = header_buf[2];
        let expected_checksum = checksum(&header_buf[..2]);
        if checksum_byte != expected_checksum {
            return Err(TransportError::InvalidChecksum);
        }

        Ok(())
    }

    fn write_and_read(&mut self, packet: &[u8]) -> Result<Vec<u8>, TransportError> {
        self.port.write_all(packet)?;

        let mut header_buf = [0u8; 3];
        self.port.read_exact(&mut header_buf).map_err(|e| {
            if e.kind() == std::io::ErrorKind::TimedOut {
                TransportError::Timeout
            } else {
                TransportError::Io(e)
            }
        })?;

        if header_buf[0] != RESPONSE_HEADER {
            return Err(TransportError::UnexpectedHeader(header_buf[0]));
        }

        let response_code = ResponseCode::try_from(header_buf[1])?;
        if response_code != ResponseCode::Success {
            return Err(TransportError::CommandRejected(response_code));
        }

        let data_length = header_buf[2] as usize;
        let mut data_and_checksum = vec![0u8; data_length];
        self.port.read_exact(&mut data_and_checksum).map_err(|e| {
            if e.kind() == std::io::ErrorKind::TimedOut {
                TransportError::Timeout
            } else {
                TransportError::Io(e)
            }
        })?;

        let checksum_byte = data_and_checksum.pop().unwrap();
        let mut full_response = header_buf.to_vec();
        full_response.extend_from_slice(&data_and_checksum);
        let expected_checksum = checksum(&full_response);

        if checksum_byte != expected_checksum {
            return Err(TransportError::InvalidChecksum);
        }

        Ok(data_and_checksum)
    }
}
