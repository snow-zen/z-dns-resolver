pub trait Deserializable: Sized {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self>;
}

pub struct Deserializer<'d> {
    src: &'d [u8],
    cursor: usize,
}

impl<'d> Deserializer<'d> {
    pub fn new(src: &'d [u8]) -> Self {
        Self { src, cursor: 0 }
    }

    pub fn read(&mut self) -> u8 {
        let result = self.src.get(self.cursor).expect("read out of bound");
        self.cursor += 1;
        *result
    }

    pub fn read_slice2(&mut self, buf: &mut [u8]) {
        let mut len = buf.len();
        if self.cursor + len > self.src.len() {
            len = self.src.len() - self.cursor;
        }
        buf.copy_from_slice(&self.src[self.cursor..(self.cursor + len)]);
        self.cursor += len;
    }

    pub fn read_slice3(&mut self, len: usize) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..len {
            result.push(self.read());
        }
        result
    }

    pub fn read_slice<const N: usize>(&mut self) -> [u8; N] {
        if self.cursor + N > self.src.len() {
            panic!("read out of bound");
        }
        let mut result = [0u8; N];
        for i in 0..N {
            result[i] = self.src[self.cursor];
            self.cursor += 1;
        }
        result
    }

    pub fn reset_cursor(&mut self, cursor: usize) -> usize {
        if self.src.len() <= cursor {
            panic!("cursor out of bound");
        }
        let old_cursor = self.cursor;
        self.cursor = cursor;
        old_cursor
    }

    pub fn peek_bytes(&self) -> &[u8] {
        self.src
    }
}

impl Deserializable for u16 {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self> {
        let mut bytes = [0u8; 2];
        deserializer.read_slice2(&mut bytes);
        Some(u16::from_be_bytes(bytes))
    }
}

impl Deserializable for u32 {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self> {
        let mut bytes = [0u8; 4];
        deserializer.read_slice2(&mut bytes);
        Some(u32::from_be_bytes(bytes))
    }
}
