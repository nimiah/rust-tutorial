Time:

- **5–7 full days**, or
- **10–14 half-day sessions**, or
- **8–12 weeks part-time**

---

## Course Goal

By the end of the course, you will:

- Understand **Rust’s mental model** (ownership, borrowing, lifetimes)
- Write **safe, idiomatic Rust**
- Build **real applications** (CLI, service, or library)
- Reason about **performance and concurrency**
- Be comfortable reading Rust code in production projects
- Understand the **clean structure** in Rust
- Build microservice Rust backend

---

# Rust Course Agenda

## 1. Introduction & Rust Mindset (½ day)

### Topics

- Why Rust? (Safety, performance, concurrency)
- How was Rust born? [https://youtu.be/LAy7EfGF_1w?si=eqmjQL2DpnVrYT6t]
- Rust vs C/C++ / Java / Go / Python
- When _not_ to use Rust
- Rust tooling overview

  - `rustc`, `cargo`, `rustup`

### Hands-on

- Install Rust
- Create first Cargo project
- Build & run
- Explore `cargo check`, `cargo fmt`, `cargo clippy`

---

## 2. Basic Syntax & Control Flow (½ day)

### Topics

- Variables & immutability
- Shadowing
- Scalar & compound types
- Functions & expressions
- `if`, `match`, loops (`loop`, `while`, `for`)
- Comments & documentation comments

### Hands-on

- Small exercises (calculator, number guessing)
- Refactor code using `match`

---

## 3. Ownership, Borrowing & Lifetimes (2 days – **core module**)

> This is where Rust “clicks” or doesn’t.

### Topics

- Stack vs heap
- Ownership rules
- Move vs copy
- Borrowing (`&T`, `&mut T`)
- Mutable aliasing rules
- Scope and drop
- Lifetimes (conceptual → explicit)
- Common compiler errors & how to read them

### Hands-on

- Fix ownership errors
- Refactor cloning-heavy code
- Write functions with references
- Simple lifetime annotations

---

## 4. Structs, Enums & Pattern Matching (1 day)

### Topics

- Structs & impl blocks
- Tuple structs & unit structs
- Enums as _sum types_
- `Option<T>` and `Result<T, E>`
- Pattern matching & destructuring
- `if let` / `while let`

### Hands-on

- Model a domain using enums
- Replace nulls/exceptions with `Option` & `Result`
- Error handling exercises

---

## 5. Modules, Crates & Visibility (½ day)

### Topics

- Modules and `mod`
- `pub` and privacy rules
- `use` and re-exports
- Crate layout best practices
- Workspace basics

### Hands-on

- Refactor a project into modules
- Create a library crate

---

## 6. Traits & Generics (1 day)

### Topics

- Defining and implementing traits
- Trait bounds
- Generic types & functions
- `impl Trait`
- Static vs dynamic dispatch
- Trait objects (`dyn Trait`)
- Derive macros (`Debug`, `Clone`, etc.)

### Hands-on

- Generic data structures
- Implement traits for custom types
- Compare trait objects vs generics

---

## 7. Error Handling & Testing (½ day)

### Topics

- `Result` best practices
- Custom error types
- `thiserror` / `anyhow` (conceptually)
- Panic vs recoverable errors
- Unit tests
- Integration tests
- Doc tests

### Hands-on

- Build robust error handling
- Write tests for existing code

---

## 8. Collections & Standard Library (½ day)

### Topics

- `Vec`, `HashMap`, `HashSet`
- String vs `&str`
- Iterators vs loops
- Iterator adapters (`map`, `filter`, `fold`)
- Ownership with collections

### Hands-on

- Data processing with iterators
- Refactor imperative code to functional style

---

## 9. Concurrency & Parallelism (1 day)

### Topics

- Fearless concurrency concept
- Threads & `std::thread`
- Message passing (`mpsc`)
- Shared state (`Arc`, `Mutex`, `RwLock`)
- `Send` and `Sync`
- Intro to async/await (conceptual)

### Hands-on

- Multi-threaded worker pool
- Fix data races using Rust guarantees

---

## 10. Async Rust (Optional / Advanced – 1 day)

### Topics

- Async model & futures
- `async` / `await`
- Executors (Tokio overview)
- Async vs threads
- Common pitfalls

### Hands-on

- Async HTTP client
- Simple async service

---

## 11. Unsafe Rust & FFI (Advanced – ½ day)

### Topics

- What `unsafe` means (and doesn’t)
- Raw pointers
- When unsafe is justified
- Calling C code (FFI)
- Safety contracts

### Hands-on

- Read unsafe code
- Wrap unsafe logic safely

---

## 12. Performance & Memory (½ day)

### Topics

- Zero-cost abstractions
- Heap allocation control
- Copy vs clone
- Profiling basics
- Benchmarks (`criterion`)

### Hands-on

- Optimize a slow function
- Measure performance impact

---

## 13. Real-World Rust Project (1–2 days)

### Options

- CLI tool
- REST API
- Log parser
- Concurrent file processor
- Library with public API

### Focus

- Code review
- Idiomatic Rust
- Error handling
- Tests & docs

---

## Teaching Tips

- **Compiler errors are a teaching tool**—embrace them
- Explain _why_ rules exist before syntax
- Use **visual memory models** (ownership diagrams)
- Encourage reading real Rust code (std, popular crates)

---

## Target Outcomes by Level

### Junior Developers

- Understand ownership & borrowing
- Write safe Rust without fighting the compiler
- Build small tools confidently

### Mid-Senior Developers

- Design APIs using traits & lifetimes
- Reason about performance & concurrency
- Read and review production Rust code

---

If you’d like, I can also:

- Tailor this for **backend**, **systems**, or **embedded**
- Create **slides + exercises**
- Provide a **week-by-week plan**
- Adapt it for **C++/Java/Go developers**

Just tell me your audience and time constraints 🙂
