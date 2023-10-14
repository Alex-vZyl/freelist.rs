use std::mem::transmute;

/// Used to describe an open block in the freelist.
/// A block can consist of many elements, if they are contiguous.
// This struct does not use `usize` since I want to force it to
// be 8 bytes (64 bits).
// I want to use `Option` in this type, but it uses 8 bytes instead of 4.
// Instead, using an API to get this functionality.  Is there a significant
// overhead?  Or a better way to do this?
pub struct Block {
    /// How many elements are (or can fit) in(to) the block.
    /// (A block can consist of many contiguous elements)
    element_count: i32,
    /// Index to the next free block.
    next_block_index: i32,
}

const NONE_INT: i32 = i32::MIN;

impl Block {
    /// Create a new block with provided parts.  This source wil be a region of
    /// memory in the freelist, for now only 64 bits.
    ///
    /// # Safety
    ///
    /// This function is highly unsage.
    ///
    /// * Transmutes the source.
    pub unsafe fn from_source_with_parts<T>(src: &mut T, element_count: i32, next_block_index: Option<i32>) -> &mut Block {
        let block: &mut Block = transmute(src);
        block.element_count = element_count;
        block.next_block_index = next_block_index.unwrap_or_else(|| NONE_INT);
        block
    }

    /// Create a new block from the source.  This source wil be a region of
    /// memory in the freelist, for now only 64 bits.
    ///
    /// # Safety
    ///
    /// This function is highly unsage.
    ///
    /// * Transmutes the source.
    pub unsafe fn from_source<T>(src: &mut T) -> &mut Block {
        transmute(src)
    }

    pub fn get_n_elements(&self) -> i32 {
        self.element_count
    }

    /// Returns the new number of elements.
    /// Errors if the block overlaps with the following block.
    pub fn grow(&mut self, increase: i32) -> Result<i32, i32> {
        let new_cap = self.element_count + increase;
        if self.has_next_block() && (new_cap >= self.next_block_index) {
            return Err(new_cap);
        }
        self.element_count = new_cap;
        Ok(new_cap)
    }

    /// Returns the new number of elements.
    /// Errors if the value is shrunk to or below 0.
    pub fn shrink(&mut self, decrease: i32) -> Result<i32, i32> {
        let new_cap = self.element_count - decrease;
        if new_cap <= 0 {
            return Err(new_cap);
        }
        self.element_count = new_cap;
        Ok(new_cap)
    }

    pub fn has_next_block(&self) -> bool {
        self.next_block_index != NONE_INT
    }

    /// The block should be removed from the freelist if this is true.
    pub fn is_empty(&self) -> bool {
        self.element_count == 0
    }

    pub fn get_next_block_index(&self) -> Option<i32> {
        match self.next_block_index {
            NONE_INT => None,
            _ => Some(self.next_block_index),
        }
    }

    /// This basically just changes `next_block_index`, since blocks
    /// do not have references to the previous block (for now?).
    pub fn connect_at(&mut self, block_index: Option<i32>) {
        self.next_block_index = block_index.unwrap_or_else(|| NONE_INT)
    }
}
