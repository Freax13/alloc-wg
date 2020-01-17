initSidebarItems({"fn":[["alloc","Allocate memory with the global allocator."],["alloc_zeroed","Allocate zero-initialized memory with the global allocator."],["dealloc","Deallocate memory with the global allocator."],["handle_alloc_error","Abort on memory allocation error or failure."],["handle_collection_alloc_error","Abort on memory allocation error or failure and panics on capacity overflow."],["realloc","Reallocate memory with the global allocator."]],"struct":[["AllocErr","The `AllocErr` error indicates an allocation failure that may be due to resource exhaustion or to something wrong when combining the given input arguments with this allocator."],["CapacityOverflow",""],["Global","The global memory allocator."],["Layout","Layout of a block of memory."],["LayoutErr","The parameters given to `Layout::from_size_align` or some other `Layout` constructor do not satisfy its documented constraints."],["NonZeroLayout","Non-zero Layout of a block of memory."]],"trait":[["Abort","Marker trait to indicate that the allocator is allowed to abort on OOM."],["AllocRef",""],["BuildAllocRef",""],["DeallocRef",""],["GlobalAlloc","A memory allocator that can be registered as the standard library’s default through the `#[global_allocator]` attribute."],["ReallocRef",""]]});