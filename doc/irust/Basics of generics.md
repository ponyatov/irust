# 2.1.3 Basics of generics



```Rust
struct Container<T> {
	value: T,
}
```

Here, we have a basic container that holds a value of type T, which is defined as a generic parameter in angle brackets. 

When you see the angle brackets (< … >), you know you’re working with generics

Creating an instance of a generic struct is relatively easy. Often, the compiler can infer the type parameter automatically:

![[Pasted image 20250520060302.png]]

Sometimes, the compiler needs hints to determine the generic type. Suppose we want to store an `Option<String>` in our container but initialize it with None. If we try the code
```Rust
let ambiguous_container = Container { value: None };
```
the compiler will fail with the following error:
```
error[E0282]: type annotations needed for `Container<Option<T>>`
```

```Rust
let ambiguous_container: Container<Option<String>> =
	Container { value: None };
```

![[new constructor]]

We can do some neat things with generics, such as constructing recursive structures with generics. As an example, we can create a structure that holds an instance of itself, such as a [[lang/linked list|linked list]] that includes a generic parameter:

![[Pasted image 20250520060926.png]]

We can implement the [[Rust/Clone#trait]] automatically by using the `#[derive]` attribute.

![[Rust/where]]

We can also use this pattern with enums. Consider this enum, which could be used to construct linked lists (albeit a useless form of them):
```Rust
enum Recursive<T> {
	Next(Box<Recursive<T>>),
	Boxed(Box<T>),
	Optional(Option<T>),
}
```
Here, an enum called Recursive can hold 
- a pointer to another Recursive, 
- a boxed T, or an 
- optional T.

This example is fairly useless, but it shows what you can do with generics.

```Rust
enum NextNode<T> {
	Next(Box<ListNode<T>>), // have some data
	End,                    // or end of list (nil)
}

struct ListNode<T> {
 data: Box<T>,              // any data in a heap
 next: NextNode<T>,         // linked list fat pointer
}
```

NOTE Implementing [[algo/linked list|linked list]]s in Rust properly is more complicated than I show in this chapter. I’ll revisit linked lists later in this book and demonstrate using [[Rust/Rc|Rc]] and [[RefCell]], which is a better way to construct linked lists. The preceding example wouldn’t be useful for most practical applications.
