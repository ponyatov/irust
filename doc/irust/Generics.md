## 2.1 Generics

generics are likely the first big topic you’ll need to learn

> Rust’s generics are compile-time, type-safe abstractions that also enhance metaprogramming; they allow you to use placeholders instead of concrete types in function and structure definitions.

Most commonly, we use generics to define structures, functions, and traits that **work with any type**

Generics let you build types that are composed of other types without necessarily needing to know about all possible type combinations

sample: You might have a function that works with integers, floats, or strings, and you don’t want to write the same function multiple times for each type

Some languages bolted on generics as late features, but Rust was (mostly) designed from the start with generics in mind. As a result, generics fit well within the language, are used nearly everywhere, and don’t feel kludgy or out of place.

Generics (combined with [[trait]]s, which we’ll discuss in section 2.2) permit type-safe programming in a way that doesn’t require explicit definitions of every possible type.

https://fasterthanli.me/articles/a-half-hour-to-learn-rust#generics

- [[Generic functions]]
- [[Type parameter constraints trait bounds]]
- [[Monomorphization]]

## 2.1.1 [[A Turing-complete type system]]
## 2.1.2 [[Why generics]]?
## 2.1.3 [[Basics of generics]]
## 2.1.4 [[Exploring Rusts Option]]
## 2.1.5 [[Marker structs and phantom types]]
## 2.1.6 [[Generic parameter trait bounds]]
