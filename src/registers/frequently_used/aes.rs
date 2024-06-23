use register_rs::*;

const AES_DATA_IN_ADDRESS_BASE: u8 = 0x80;
const AES_DATA_LENGTH: usize = 16;

const AES_DATA_OUT_ADDRESS_BASE: u8 = 0xD4;

/// `AES_KEY_IN` register
pub struct AesKeyIn {
    /// AES engine key input (128 bits)
    pub key: [u8; AES_DATA_LENGTH]
}

/// `AES_DATA_IN` register
pub struct AesDataIn {
    /// AES engine data input (128 bits)
    pub data: [u8; AES_DATA_LENGTH]
}

/// `AES_DATA_OUT` register
pub struct AesDataOut {
    /// AES engine data output (128 bits)
    pub data: [u8; AES_DATA_LENGTH]
}

impl Register<u8> for AesKeyIn {
    const ADDRESS: u8 = 0x70;
    const LENGTH: usize = AES_DATA_LENGTH;
    fn reset_value() -> Self {
        Self {
            key: [0; AES_DATA_LENGTH]
        }
    }
}

impl Register<u8> for AesDataIn {
    const ADDRESS: u8 = 0x80;
    const LENGTH: usize = AES_DATA_LENGTH;

    fn reset_value() -> Self {
        Self {
            data: [0; AES_DATA_LENGTH]
        }
    }
}

impl Register<u8> for AesDataOut {
    const ADDRESS: u8 = 0xD4;
    const LENGTH: usize = AES_DATA_LENGTH;

    fn reset_value() -> Self {
        Self {
            data: [0; AES_DATA_LENGTH]
        }
    }
}

impl WriteableRegister<u8> for AesDataIn {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        Ok(self.data)
    }
}

impl WriteableRegister<u8> for AesKeyIn {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        Ok(self.key)
    }
}

impl ReadableRegister<u8> for AesDataIn {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self {
            data: *buffer
        })
    }
}

impl ReadableRegister<u8> for AesKeyIn {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self {
            key: *buffer
        })
    }
}

impl ReadableRegister<u8> for AesDataOut {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self {
            data: *buffer
        })
    }
}