const END: u8     = 0xC0;
const ESC: u8     = 0xDB;
const ESC_END: u8 = 0xDC;
const ESC_ESC: u8 = 0xDD;

#[derive(Debug)]
pub enum DecoderErrorCode {
    BadEsc,
    BufferFull,
}

#[derive(Debug)]
pub struct DecoderError {
    pub pos: usize,
    pub code: DecoderErrorCode,
}

pub struct DecoderBuffer<const CAPACITY: usize> {
    idx: usize,
    buf: [u8; CAPACITY],
}

pub struct Decoder<const CAPACITY: usize> {
    buf: DecoderBuffer<CAPACITY>,
    is_escaping: bool
}

////////////////////////////////////////////

impl<const CAPACITY: usize> DecoderBuffer<CAPACITY> {
    pub fn new() -> Self {
        Self {idx: 0, buf: [0u8; CAPACITY] }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn slice(&self) -> &[u8] {
        &self.buf[..self.idx]
    }

    pub fn put(&mut self, c: u8)-> Result<(), DecoderErrorCode> {
        if self.idx >= CAPACITY {
            Err(DecoderErrorCode::BufferFull)
        }

        else {
            self.buf[self.idx] = c;
            self.idx+=1;
            Ok(())
        }
    }
}

impl<const CAPACITY: usize> Decoder<CAPACITY> {
    pub fn new() -> Self {
        Self {
            buf: DecoderBuffer::new(),
            is_escaping: false
        }
    }

    pub fn reset(&mut self) {
        self.is_escaping = false;
        self.buf.reset();
    }

    pub fn slice(&self) -> &[u8] {
        self.buf.slice()
    }

    pub fn feed(&mut self, input: &[u8]) -> Result<(usize, bool), DecoderError> {
        let mut i = 0;

        while i < input.len() {
            let c = input[i]; // Consume 1 char from input buffer
            i += 1;           // Increment counter
            
            if self.is_escaping {
                self.is_escaping = false;
                match c {
                    ESC_END => {
                        match self.buf.put(END) {
                            Ok(_) => {},
                            Err(code) => {return Err(DecoderError{ pos: i, code: code});}
                        }
                    }

                    ESC_ESC => {
                        match self.buf.put(ESC) {
                            Ok(_) => {},
                            Err(code) => {return Err(DecoderError{ pos: i, code: code});}
                        }
                    }

                    _ => {return Err(DecoderError{ pos: i, code: DecoderErrorCode::BadEsc});}
                }
            }

            else {
                match c {
                    END => {
                        return Ok((i, true));
                    }

                    ESC => {
                        self.is_escaping = true;
                    }

                    // otherwise, put stuff in buffer
                    _ => {
                        match self.buf.put(c) {
                            Ok(_) => {},
                            Err(code) => {return Err(DecoderError{ pos: i, code: code});}
                        }
                    }
                }
            }
        }

        // Input buffer processed, but no packet end yet detected
        Ok((i, false))
    }
}
