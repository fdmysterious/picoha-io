// ============================================================================

use core::cmp;

/// Simple structure handling buffer index
pub struct UsbBufferIndex<const CAPACITY: usize> {
    idx: usize,
}

// ============================================================================

impl<const CAPACITY: usize> UsbBufferIndex<CAPACITY> {
    pub fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn shift(&mut self, nbytes: usize, is_end: bool) -> Result<(), ()> {
        self.idx += nbytes;

        match (is_end && (self.idx > CAPACITY)) || (!is_end && (self.idx >= CAPACITY)) {
            true  => Ok(()),
            false => Err(())
        }
    }

    pub fn value(&self) -> usize {
        self.idx
    }
}

// ============================================================================
