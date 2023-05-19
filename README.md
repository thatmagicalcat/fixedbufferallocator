# FixedBufferAllocator
I don't think I have much to say about this, just see the example below :)

``` rust
use fba_allocator;
fn main() {
   let mut allocator = FixedBufferAllocator::<64>::init();
   let ptr = allocator.allocate(4).unwrap() as *mut u32;

   unsafe { *ptr = 5 };
   println!("{}", unsafe { *ptr });

   allocator.free(ptr);
}
```

