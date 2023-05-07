# Visited
Data structure to keep track of visited objects that does not require clearing for several iterations.

## What is this for?
Sometimes, in algorithms such as breadth-first search, you need to keep a vector that represents whether a node 
was visited or not. For these instances, of course, a normal vector of bools is quite okay.

The issues begin when you need to repeat the same BFS over and over, such as when you are computing a centrality 
metric such as a Closeness centrality or a Betweenness centrality. In these cases, you would need to reset the state
of the counter.

If we say that the graph has `|V|` nodes, you would be resetting an object of length `|V|` for `|V|` times, so you would
be executing an operation, granted quite a small one, for `|V|^2` times. That is not good, and we can do better.
Let's replace the boolean values with integers, as booleans  are commonly not bits but `u8` values unless you are using a 
bitvector.

We start by setting the vector of counters to `0`, and we set a `visited` flag to `1`. Whenever we want to reset the state
of the vector, we simply run `visited+=1`, implicitly clearing all values of the vector.

Now, up until we reach `T::MAX` if we are using `T` unsigned or whathever other unsigned numerical value we intend to use, we can
simply clear the entire vector implicitly by bumping the visited flag. The entire vector still requires an extensive clearing
when the flag reaches saturation if we cannot assume that the vector is all visited by the end of the execution of the
algorithm, such as in the execution of BFS runs in disconnected graphs.

So with this approach, we can reduce the complexity from `|V|^2` to `|V|^2 / min(T::MAX, |V|)`. For instance, if you use `T=u16` and 
you have less than `32k` nodes, the complexity would be reduced to `|V|`.

## How do I use with this crate?
For now, I am still exploring ways to improve this softwate, so you can use it in your Rust project by
including in your `Cargo.toml` by adding the following line under `[dependencies]`:

```toml
visited-rs = { git = "https://github.com/LucaCappelletti94/visited-rs", branch = "main" }
```

Then, you can import the structs contained in it as such:

```rust
use visited_rs::prelude::*;
```

You can init a new object with, for instance, the number of nodes as:

```rust
let mut visited = Visited::zero(number_of_nodes);
```

Then, in your BFS-like loop, you will want something like:

```rust
let previous_state = visited.set_and_get_visited(node);
```

Or you can do them separately as such:

```rust
let previous_state = visited.is_visited(node);
visited.set_visited(node);
```

Finally, sometimes it make sense to write code in a [data-race aware](https://en.wikipedia.org/wiki/Race_condition) manner.
For those special occasion where you really want to break the mutability system, I have prepared the following [`unsafe`](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) methods:

```rust
unsafe{visited.set_visited_racing(node);}
unsafe{let previous_state = visited.set_and_get_visited_racing(node);}
```

Use them wisely.

And finally, when you want to clear the object for the next round of BFS, you can simply use:

```rust
visited.clear();
```

## Helping
You can help out by [opening an issue](https://github.com/LucaCappelletti94/visited-rs/issues) or a [pull request](https://github.com/LucaCappelletti94/visited-rs/pulls). If you like, you can also [directly support my work on GitHub](https://github.com/sponsors/LucaCappelletti94).