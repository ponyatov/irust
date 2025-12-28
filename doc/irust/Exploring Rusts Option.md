# 2.1.4 Exploring Rust’s Option

Rust’s [[Rust/Option|Option]] is one of the most delightful examples of generics in practice. Its definition is simple and elegant, yet it provides an incredibly powerful abstraction.

```Rust
pub enum Option<T> {
 None,
 Some(T),
}
```

- [[None]]
- [[Rust/Some|Some]]
