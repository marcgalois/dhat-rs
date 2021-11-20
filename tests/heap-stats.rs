#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn main() {
    let v1 = vec![1u32, 2, 3, 4];
    let v2 = vec![1u32, 2, 3, 4];
    let mut v3 = vec![1u32, 2, 3, 4];
    let mut v4 = vec![1u32, 2, 3, 4];

    {
        let _profiler = dhat::Profiler::heap_start();

        // Things allocated beforehand aren't counted.
        let empty_stats = dhat::HeapStats {
            total_blocks: 0,
            total_bytes: 0,
            curr_blocks: 0,
            curr_bytes: 0,
            max_blocks: 0,
            max_bytes: 0,
        };
        assert_eq!(dhat::HeapStats::get(), empty_stats);

        // Allocated before, freed during.
        drop(v1);

        // Allocated before, reallocated during.
        v3.push(5);

        // Things allocated during are counted (and the realloc is treated like
        // an alloc, i.e. we count the entire thing, not just the difference
        // between the old and new sizes).
        let final_stats = dhat::HeapStats {
            total_blocks: 1,
            total_bytes: 32,
            curr_blocks: 1,
            curr_bytes: 32,
            max_blocks: 1,
            max_bytes: 32,
        };
        assert_eq!(dhat::HeapStats::get(), final_stats);
    }

    // Allocated before, freed after.
    drop(v2);

    // Allocated before, reallocated after.
    v4.push(5);
}
