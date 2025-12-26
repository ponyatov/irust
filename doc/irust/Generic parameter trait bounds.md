# 2.1.6 Generic parameter trait bounds

[[trait bound]]s are a feature of generics that allows you to control which types can be used with a particular structure or function by specifying which traits must be implemented.

> Specifically, trait bounds enable us to specify which features must be available for a given generic type parameter

We can specify multiple trait bounds that apply on a per-parameter basis.

```Rust
#[derive(Clone)]
struct ListItem<T>
where
 T: Clone + Debug,
{
 data: Box<T>,
 next: Option<Box<ListItem<T>>>,
}
```
- We’ve derived the [[Rust/Clone#trait]] , which allows us to call `clone()` on the struct to copy it
- generic type T must also implement the `Clone` to be shure we can clone it making new list element
- we want to require that Clone and Debug be implemented (to be able to print item)

