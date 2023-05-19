#![allow(unused)]

#[derive(Debug)]
pub struct FixedBufferAllocator<const SIZE: usize> {
    buffer: [u8; SIZE],
    /// for keeping track of free blocks
    blocks: Vec<MemoryBlock>,
}

impl<const SIZE: usize> FixedBufferAllocator<SIZE> {
    /// You should not use the buffer are initializing FBA
    pub fn init() -> Self {
        let mut buffer = [0; SIZE];
        Self {
            blocks: vec![MemoryBlock::new(
                buffer.first_mut().unwrap(),
                buffer.last_mut().unwrap(),
                true // the memory is free
            )],
            buffer
        }
    }

    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    pub fn bytes_free(&self) -> usize {
        let mut bytes_free = 0;
        self.blocks.iter().filter(|b| b.free).for_each(|b| {
            bytes_free += b.size();
        });

        bytes_free
    }

    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, String> {
        // check if there exists any memory block greater than or equal to the size
        let mut memory_block = None;
        for (i, c) in self.blocks.iter().enumerate() {
            if c.free && c.size() >= size {
                memory_block = Some(i)
            }
        }

        let Some(idx) = memory_block else {
            return Err("Out of memory!".to_string());
        };

        let memory_block = self.blocks[idx];

        self.blocks.remove(idx);

        // split the memory block into two
        let allocated_memory_block =
            MemoryBlock::new(memory_block.start, unsafe { memory_block.start.add(size) }, false);
        let allocated_memory_ptr = allocated_memory_block.start;

        self.blocks.insert(idx, allocated_memory_block);

        if memory_block.size() != size {
            self.blocks.insert(
                idx + 1,
                MemoryBlock::new(unsafe { memory_block.start.add(size) }, memory_block.end, true),
            );
        }

        Ok(allocated_memory_ptr)
    }

    pub fn print_chunks(&self) {
        println!("{:?}", self.blocks);
    }

    pub fn free(&mut self, ptr: *mut u8) {
        let (idx, _) = self
            .blocks
            .iter_mut()
            .enumerate()
            .find(|chunk| !chunk.1.free && chunk.1.start == ptr)
            .expect("Invalid pointer");

        self.merge_blocks(idx);
    }

    fn merge_blocks(&mut self, mut block_idx: usize) {
        if block_idx != 0 && self.blocks[block_idx - 1].free {
            self.blocks[block_idx - 1].end = self.blocks[block_idx].end;
            self.blocks.remove(block_idx);

            block_idx -= 1;
        }

        if block_idx + 1 < self.blocks.len() && self.blocks[block_idx + 1].free {
            self.blocks[block_idx + 1].start = self.blocks[block_idx].start;
            self.blocks.remove(block_idx);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MemoryBlock {
    /// start position of the memory block
    start: *mut u8,

    /// end position of the memory block
    end: *mut u8,

    /// is the memory freed?
    free: bool,
}

impl MemoryBlock {
    pub fn new(start: *mut u8, end: *mut u8, free: bool) -> Self {
        Self {
            start,
            end,
            free,
        }
    }

    pub fn size(&self) -> usize {
        let size = unsafe { self.end.offset_from(self.start) };
        if size < 0 {
            panic!("Invalid pointer");
        }

        (size + 1) as _
    }
}
