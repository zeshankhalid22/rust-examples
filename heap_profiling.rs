#[cfg(feature = "dhat-heap")]
// Setting the global allocator to dhat::Alloc
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    // If the dhat-heap feature is enabled, create a new heap profiler
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // allocates 5 * (4B) on heap
    let mut nums = vec![1,2,3,4,5];
    // This allocates 10 more items(4B each)
    nums.resize(10,0);
    
    // run program with 
    // cargo run --bin cell  --features dhat-heap
}
