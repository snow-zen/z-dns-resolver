pub trait Serializable {
    fn serialize(&self, serializer: &mut Serializer);
}

pub struct Serializer {
    serial_byte: Vec<u8>,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            serial_byte: Vec::new(),
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.serial_byte.push(byte)
    }

    pub fn extend<I: IntoIterator<Item = u8>>(&mut self, bytes: I) {
        self.serial_byte.extend(bytes)
    }

    pub fn to_owned_bytes(&self) -> Vec<u8> {
        self.serial_byte.to_owned()
    }
}
