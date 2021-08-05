# bs3

Blockchain simple state storage.

## Features (WIP)

- [ ] Stateless: Data in this storage don't affect block.
- [ ] Stateful: Data in this storage will affect block.
- [X] Transaction based on cache.
  - [X] Force sync state to lastest success transaction. (For `check_tx`)
  - [X] Commit transaction for success transaction. (For `deliver_tx`)
  - [X] Revert transaction for failed transaction. (For `deliver_tx`)
- [ ] Snapshot based on CoW for each block.
  - [ ] Load snapshot from any height to recover node.
  - [ ] Rollback snapshot.
  - [ ] Read any height snapshot.
  - [ ] Use snapshot to compute world state and compute merkle root.
- [X] Support multi-type of backend.
  - [X] Store trait.
  - [ ] Sled backend.
  - [ ] Memory backend.

## Design

### Backend Requirement

- get value by key.
- batch execute.

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

- Block height `n`'s snapshot is state diff of height `n-1` between `n`. If `n == 0` mean no state.
- Each height have a index map, this map use to find which snapshot have this data.
  - This link refer to closer height less than itself.

![](docs/assets/BS3-snapshot.svg)

