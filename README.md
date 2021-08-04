# bs3

Blockchain simple state storage.

## Features (WIP)

- [ ] Stateless: Data in this storage don't have any effect for blockchain.
- [ ] Stateful: Data in this storage will affect blockchain.
- [ ] Transaction based on cache.
- [ ] Snapshot based on CoW.
  - [ ] Load snapshot.
  - [ ] Rollback snapshot.
  - [ ] Read Only snapshot.
- [ ] Support multi-type of backend.
  - [ ] Store trait.
  - [ ] Sled backend.

## Design

### Stateless

Stateless storage has interface same as `BTreeMap`.

### Stateful

Stateful store's API is same as Stateless store, only add one API:

``` rust
pub trait Stateful<D: Digest>: Stateless<D> {
    fn root(&self) -> Output<D>;
}
```

`bs3` compute merkle when you call `root` method.

### Transaction

> Transaction often use for compute blockchain transaction.

Transaction implement by `BTreeMap` cache.

### Snapshot

