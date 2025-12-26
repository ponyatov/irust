# 2.1.2 Why generics?

In statically typed languages like Rust, the compiler needs to know the type of everything at compile time.

Generics allow you to write code that works with any type without the developer’s needing to know the type at compile time.

We employ generics to follow the [[DRY]] (Don’t Repeat Yourself) principle throughout our codebase. Writing the same code in many places where the only difference is the type signature is a recipe for headaches

The downside to generics is that they can make code harder to read and write

Also, the compiler can’t always figure out the types you want, so you may need to provide hints to tell it what you’re trying to do
