# bs3

Blockchain simple state storage.

## Features (WIP)

- [ ] Stateless: Data in this storage don't have any effect for blockchain.
- [ ] Stateful: Data in this storage will affect blockchain.
- [ ] Snapshot based on CoW.
  - [ ] Load snapshot.
  - [ ] Rollback snapshot.
  - [ ] Read snapshot.
- [ ] Support multi-type of backend.

## Design

### Stateless

Stateless storage has interface same as `BTreeMap`, beacuse `Stateless` trait
impl `DerefMut` directly.

``` rust
pub trait Stateless<D: Digest>:
    DerefMut<Output = BTreeMap<Output<D>, Vec<u8>>>
{

    fn flush(&self) -> Result<usize>;

    async flush_async(&self) -> Result<usize>;
}
```

### Stateful

Stateful store's API is same as Stateless store, only add one API:

``` rust
pub trait Stateful<D: Digest>: Stateless<D> {
    fn root(&self) -> Output<D>;
}
```

