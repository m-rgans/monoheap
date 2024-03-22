# MonoHeap
This container is meant for things that you dont want moving around, and that other things need to keep a reference to, but actually directly using references would be a pain. Its basically a primitive arena allocator.

## Tokens
a token is used to retrieve something from the monoheap after insertion. They are produced when the thing is inserted. They implement clone and copy.
Tokens have a unique allocation number that prevents them from being dereferenced after they've been removed. An invalid token will yield None.