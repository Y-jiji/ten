# ten
tensor computation framework leveraging sparsity

# procedure

support eager/lazy mode operation. 

1. build graph ir from tensor operators
2. load physical backends
3. perform algebraic optimizations, specify data formats (one-pass build)
4. lowering access patterns to for loops and parallelize them
5. generate backend code
