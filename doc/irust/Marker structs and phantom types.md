# 2.1.5 [[marker]] structs and phantom types
## Маркерные структуры и фантомные типы

Иногда возникает необходимость создавать структуры с обобщёнными параметрами, но при этом не обязательно использовать эти параметры в самой структуре. Такая ситуация требует использования **фантомных типов ([[phantom types]])**, которые позволяют использовать обобщённые параметры, не являющиеся членами вашей структуры. Фантомные типы позволяют применять паттерны, такие как **тегирование структур ([[struct tagging]])**, которые мы обсудим в главе 7.

[[Marker traits]]

Рассмотрим следующий фрагмент кода, который содержит структуру с параметром типа, но этот тип не используется в самой структуре (у нас есть только информация о типе во время компиляции):

```Rust
struct Dog<Breed> {
    name: String,
}
```

Структура `Dog` содержит имя собаки. Мы хотим отслеживать породу собаки, но нас интересуют эти детали **только во время компиляции** (а не во время выполнения). Поэтому мы можем эффективно хранить эту информацию как параметр типа и не включать поле `breed: Breed` в структуру.

Нам нужно создать некоторые типы для идентификации наших пород, что мы сделаем следующим образом:

```rust
struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}
```

Эти структуры являются **маркерными типами ([[Rust/marker|marker]] types)** или **типами-единицами ([[type/unit type]]s)** — они не содержат данных и используются исключительно для передачи информации о типах во время компиляции.

Однако при попытке скомпилировать код с такой структурой мы столкнёмся с ошибкой:

```
error[E0392]: parameter `Breed` is never used
 --> src/main.rs:27:12
|
27 | struct Dog<Breed> {
 | ^^^^^ unused parameter
 |
 = help: consider removing `Breed`, referring to it in a field,
 or using a marker such as `PhantomData`
 = help: if you intended `Breed` to be a const parameter,
 use `const Breed: usize` instea
```

Компилятор Rust недоволен, потому что мы добавили неиспользуемый обобщённый параметр в структуру, что компилятор (справедливо) отмечает как ошибку. Эта проверка является важной особенностью Rust — компилятор хочет убедиться, что:

1. Все объявленные параметры действительно используются
2. Нет неявных затрат на поддержку неиспользуемых параметров
3. Код ясен и не содержит "мусорных" параметров

Мы можем добавить фантомное поле, чтобы дать компилятору понять, что нам нужен этот параметр, но мы заботимся о значении **только во время компиляции** и, следовательно, не нужно хранить его в структуре:

```rust
use std::marker::PhantomData;

struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,  // Фантомное поле
}
```

![[Rust/PhantomData]]

Когда мы пишем `PhantomData<Breed>`, мы сообщаем компилятору:

1. **"Я знаю о типе `Breed`"**: Этот тип является частью сигнатуры типа `Dog`
2. **"Я не храню данные этого типа"**: В структуре нет фактического поля типа `Breed`
3. **"Но я хочу, чтобы он влиял на тип"**: `Dog<Labrador>` и `Dog<Poodle>` должны быть разными типами


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
