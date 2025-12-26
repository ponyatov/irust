# 2.1.5 [[marker]] structs and phantom types
## [[Marker traits]]

Sometimes, you want to make structures with generic parameters, but you don’t necessarily want to use the generic parameters in the structure itself.

This situation calls for [[phantom type]]s, which enable you to use generic parameters that aren’t members of your structure.

Phantom types allow the use of patterns such as [[struct tagging]]

```Rust
struct Dog<Breed> {
	name: String,
}
```

The `Dog` structure holds a dog’s name. We want to keep track of the breed of the dog, but **we care about those details only at compile time** (not run time), so we can effectively store that information as a type parameter and not bother including a `breed: Breed` field within the struct.

![[Rust/marker#struct]]

We’re using an empty struct to label each breed. We could use any type, but we’ll use an empty struct for this example.

But: The compiler is unhappy because we added an unused generic parameter to the struct, which the compiler (rightfully) notes is an error.

![[Rust/marker#PhantomData]]

We can add specialized implementations of Dog for each distinct breed if we choose to do so. We can return the name of the breed without storing that value as state or as a separate field within the structure:

![[Pasted image 20250520064904.png]]

`impl Dog<Labrador>` is a concrete specialization for `Dog` with the `Labrador` breed type. [[impl]] doesn’t require the `Breed` generic parameter because it’s a concrete [[specialization]].

We can return the breed name without storing it as a field in the struct. The name will be constant as part of the compiled program’s `.rodata` segment.

For each impl block, we’re creating a concrete specialization for Dog with the given type. We can add as many concrete specializations as we want, and if we’re missing one, the compiler will let us know.

Note that in the `breed_name()` methods, we don’t need to use the `'static` lifetime with our `&str` reference because the methods take `&self`. Thus, the compiler can reasonably conclude that the lifetime of the returned string will match `&self`.

![[Lifetimes and static]]

```Rust
let my_poodle: Dog<Poodle> = Dog {
 name: "Jeffrey".into(),
 breed: PhantomData,
};

println!(
 "My dog is a {}, named {}",
 my_poodle.breed_name(),
 my_poodle.name,
);
```
- struct must be constructed with `PhantomData` used as init
