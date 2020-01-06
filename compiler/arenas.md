# Arenas

*[Arenas vs Indices](https://llogiq.github.io/2019/04/06/arena.html?utm_source=share&utm_medium=ios_app)*

**arena**: reserve some sufficiently large space to put your objects in, then allocate by incrementing a pointer; if your object is of a uniform type, you can simplify this to a `Vec` of that type `=>` indices become like references to the elements

Because our objects are evenly sized, we could (in theory) index up to `isize::MAX` elements...also that if we restrict ourselves to storing not more than `u32::MAX` elements, we can get away with using a `u32` as index even on 64-bit systems.

**Why do we care?** If we can keep the size of references as small as possible, we can increase the amount of data that stays in cache. *With less objects in the arena, we may be able to reduce pointer size even further*.
1. we use object size as a multiplier instead of alignment to extend the "address space"
2. we restrict the effect to objects within our arena

```rust
struct MVPArena<T>(Vec<T>);

impl<T> MVPArena<T> {
    fn add(&mut self, value: T) -> Idx { .. }
}

type Idx = u32;

impl<T> Index<<Idx> for MVPArena<T> {
    type Output = T;
    fn index(&self, idx: Idx) -> &T {
        &self[idx as usize]
    }
}
```

This allows objects to be referenced by `arena[idx]`. 

## Branding

Extend the `Arena` and the `Idx` type with a phantom data containing the *invariant* lifetim:

```rust
struct Idx<'a>(u332, PhantomData<*mut &'a ()>);
```

*Invariant* implies that the borrow checker may not contract or extend the lifetime. By using a mutable pointer, the compiler treats the data as if it's being both read (precludes shortening the lifetime) and written to (inhibits extensions).

Therefore, we extend the `Arena` to store the same `PhantomData` and change `add` and `index` to unify the lifetimes:
```rust
// ..
fn add(&mut self, value: T) -> Idx<'a> {
    // ..
    Idx(index, self.tag)
}

impl<'a, T> Index<Idx<'a>> for MVPArena<'a, T> {
    type Output = T;
    fn index(&self, idx: Idx<'a>) -> &'a T {
        &self[idx.0 as usize]
    }
}
``` 

Invariant lifetimes limit flexibility by making it so that we cannot get the lifetime out of scope and cannot store it within the arena; the following will compile with a lifetime error:

```rust
struct Tree<'i>(Option<(Idx<'i>, Idx<'i>)>);

fn build_tree(arena: &mut Arena<'i, Tree<'i>>, depth: usize) {
    if depth == 0 {
        arena.add(Tree(None))
    } else {
        arena.add(Tree(Some((build_tree(arena, depth -1), build_tree(arena, depth - 1)))))
    }
}

in_arena(|arena| {build_tree(arena, 3); });
```

We want to instead ensure that the indices are only ever given out by their respective arena and not mixed between arenas. 

* the arena needs a constructor that takes a distinctly-typed argument
* we need a proc macro that creates a new type on each call; creates code like...


```rust
{
    struct Tag; // Some distinct type

    let mut __arena == unsafe { Arena::new(Tag) };
    let arena = &mut __arena;
    // .. do something with arena here
}
```

Macro hygience means each part of the code gets its own `Tag` type.
* downside: indices and arenas can no longer be `Send` or `Sync`; if we could share an arena between threads, nothing would keep us from having two threads instantiate arenas from the same place in the code (so they could mix up the indices)
* upside: we can store our indices within our Arena

## Other Memory Layouts

* benefit of the arena is that the memory layout is continuous so indexing is a simple lookup
* downside is that the arena needs to reallocate whne growing and possibly copy around the memory

> *All problems in computer science can be solved by another level of indirection* ~ David Wheeler

## Crates

* [`compact_arena`](https://github.com/llogiq/compact_arena)
* [`toolshed`](https://github.com/ratel-rust/toolshed)
* [`generational-arena`](https://github.com/fitzgen/generational-arena)