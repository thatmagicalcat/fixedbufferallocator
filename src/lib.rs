mod allocator;

pub use allocator::FixedBufferAllocator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocation_and_deallocation() {
        let mut allocator = allocator::FixedBufferAllocator::<64>::init();

        assert_eq!(allocator.size(), 64);

        let ptr = allocator.allocate(8).expect("Failed to allocate memory") as *mut u64;

        assert_eq!(allocator.bytes_free(), 56);

        unsafe { *ptr = 5; }

        assert_eq!(unsafe { *ptr }, 5);

        allocator.free(ptr.cast());

        assert_eq!(allocator.bytes_free(), allocator.size());
    }
}
