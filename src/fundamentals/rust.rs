use std::collections::BTreeMap;

pub fn get() -> BTreeMap<u32, String> {
    let mut topics: BTreeMap<u32, String> = BTreeMap::new();

    // Variables and Types
    topics.insert(1, String::from("let bindings (immutable by default)"));
    topics.insert(2, String::from("let mut (mutable bindings)"));
    topics.insert(3, String::from("const (compile-time constants)"));
    topics.insert(4, String::from("static (global variables)"));
    topics.insert(5, String::from("Type inference"));
    topics.insert(6, String::from("Type annotations"));
    topics.insert(7, String::from("Scalar types (i32, u64, f64, bool, char)"));
    topics.insert(8, String::from("Compound types (tuple, array)"));
    topics.insert(9, String::from("String vs &str"));
    topics.insert(10, String::from("Vec<T> (dynamic arrays)"));
    topics.insert(11, String::from("HashMap<K, V>"));
    topics.insert(12, String::from("HashSet<T>"));
    topics.insert(13, String::from("BTreeMap and BTreeSet"));

    // Ownership and Borrowing
    topics.insert(14, String::from("Ownership rules"));
    topics.insert(15, String::from("Move semantics"));
    topics.insert(16, String::from("Copy trait"));
    topics.insert(17, String::from("Clone trait"));
    topics.insert(18, String::from("References (&T)"));
    topics.insert(19, String::from("Mutable references (&mut T)"));
    topics.insert(20, String::from("Borrowing rules"));
    topics.insert(21, String::from("Lifetimes ('a)"));
    topics.insert(22, String::from("Lifetime elision"));
    topics.insert(23, String::from("'static lifetime"));
    topics.insert(24, String::from("Dangling references (prevented)"));

    // Control Flow
    topics.insert(25, String::from("if/else expressions"));
    topics.insert(26, String::from("loop (infinite loop)"));
    topics.insert(27, String::from("while loops"));
    topics.insert(28, String::from("for loops and iterators"));
    topics.insert(29, String::from("match expressions"));
    topics.insert(30, String::from("if let"));
    topics.insert(31, String::from("while let"));
    topics.insert(32, String::from("break and continue"));
    topics.insert(33, String::from("break with value"));
    topics.insert(34, String::from("Loop labels"));

    // Functions
    topics.insert(35, String::from("fn keyword"));
    topics.insert(36, String::from("Parameters and return types"));
    topics.insert(37, String::from("Expression-based returns"));
    topics.insert(38, String::from("Early returns"));
    topics.insert(39, String::from("Closures (|x| x + 1)"));
    topics.insert(
        40,
        String::from("Closure capture modes (Fn, FnMut, FnOnce)"),
    );
    topics.insert(41, String::from("move closures"));
    topics.insert(42, String::from("Higher-order functions"));

    // Structs
    topics.insert(43, String::from("Struct definitions"));
    topics.insert(44, String::from("Tuple structs"));
    topics.insert(45, String::from("Unit structs"));
    topics.insert(46, String::from("Struct update syntax (..)"));
    topics.insert(47, String::from("impl blocks"));
    topics.insert(48, String::from("Associated functions"));
    topics.insert(49, String::from("Methods (&self, &mut self, self)"));
    topics.insert(50, String::from("Self type"));

    // Enums
    topics.insert(51, String::from("Enum definitions"));
    topics.insert(52, String::from("Enum variants with data"));
    topics.insert(53, String::from("Option<T> (Some/None)"));
    topics.insert(54, String::from("Result<T, E> (Ok/Err)"));
    topics.insert(55, String::from("Pattern matching on enums"));
    topics.insert(56, String::from("if let with enums"));

    // Traits
    topics.insert(57, String::from("Trait definitions"));
    topics.insert(58, String::from("Implementing traits"));
    topics.insert(59, String::from("Default trait implementations"));
    topics.insert(60, String::from("Derive macro (#[derive(...)])"));
    topics.insert(61, String::from("Common derives (Debug, Clone, PartialEq)"));
    topics.insert(62, String::from("Trait bounds (T: Trait)"));
    topics.insert(63, String::from("Multiple trait bounds (T: A + B)"));
    topics.insert(64, String::from("where clauses"));
    topics.insert(65, String::from("Associated types"));
    topics.insert(66, String::from("Supertraits"));
    topics.insert(67, String::from("Trait objects (dyn Trait)"));
    topics.insert(68, String::from("Object safety"));

    // Generics
    topics.insert(69, String::from("Generic functions"));
    topics.insert(70, String::from("Generic structs"));
    topics.insert(71, String::from("Generic enums"));
    topics.insert(72, String::from("Generic impl blocks"));
    topics.insert(73, String::from("Monomorphization"));
    topics.insert(74, String::from("Phantom data"));

    // Error Handling
    topics.insert(75, String::from("panic! macro"));
    topics.insert(76, String::from("Result<T, E> for recoverable errors"));
    topics.insert(77, String::from("? operator"));
    topics.insert(78, String::from("unwrap() and expect()"));
    topics.insert(79, String::from("unwrap_or() and unwrap_or_else()"));
    topics.insert(80, String::from("Custom error types"));
    topics.insert(81, String::from("From trait for error conversion"));
    topics.insert(82, String::from("thiserror and anyhow crates"));

    // Iterators
    topics.insert(83, String::from("Iterator trait"));
    topics.insert(84, String::from("iter(), iter_mut(), into_iter()"));
    topics.insert(85, String::from("map()"));
    topics.insert(86, String::from("filter()"));
    topics.insert(87, String::from("collect()"));
    topics.insert(88, String::from("fold() and reduce()"));
    topics.insert(89, String::from("enumerate()"));
    topics.insert(90, String::from("zip()"));
    topics.insert(91, String::from("chain()"));
    topics.insert(92, String::from("take() and skip()"));
    topics.insert(93, String::from("Iterator adapters (lazy)"));
    topics.insert(94, String::from("Iterator consumers"));

    // Smart Pointers
    topics.insert(95, String::from("Box<T> (heap allocation)"));
    topics.insert(96, String::from("Rc<T> (reference counting)"));
    topics.insert(97, String::from("Arc<T> (atomic reference counting)"));
    topics.insert(98, String::from("RefCell<T> (interior mutability)"));
    topics.insert(99, String::from("Mutex<T>"));
    topics.insert(100, String::from("RwLock<T>"));
    topics.insert(101, String::from("Weak<T>"));
    topics.insert(102, String::from("Deref and DerefMut traits"));

    // Concurrency
    topics.insert(103, String::from("std::thread::spawn"));
    topics.insert(104, String::from("JoinHandle"));
    topics.insert(105, String::from("move closures with threads"));
    topics.insert(106, String::from("Send and Sync traits"));
    topics.insert(107, String::from("Message passing (channels)"));
    topics.insert(108, String::from("mpsc (multi-producer single-consumer)"));
    topics.insert(109, String::from("Shared state concurrency"));

    // Async
    topics.insert(110, String::from("async fn"));
    topics.insert(111, String::from("await keyword"));
    topics.insert(112, String::from("Future trait"));
    topics.insert(113, String::from("async runtimes (tokio, async-std)"));
    topics.insert(114, String::from("tokio::spawn"));
    topics.insert(115, String::from("select! macro"));
    topics.insert(116, String::from("async streams"));

    // Modules and Crates
    topics.insert(117, String::from("mod keyword"));
    topics.insert(118, String::from("pub visibility"));
    topics.insert(119, String::from("use statements"));
    topics.insert(120, String::from("Crate root (lib.rs, main.rs)"));
    topics.insert(121, String::from("External crates (Cargo.toml)"));
    topics.insert(122, String::from("pub(crate) and pub(super)"));
    topics.insert(123, String::from("Re-exports (pub use)"));

    // Macros
    topics.insert(124, String::from("Declarative macros (macro_rules!)"));
    topics.insert(125, String::from("Procedural macros"));
    topics.insert(126, String::from("Derive macros"));
    topics.insert(127, String::from("Attribute macros"));
    topics.insert(128, String::from("Function-like macros"));
    topics.insert(129, String::from("Common macros (vec!, println!, format!)"));

    // Unsafe
    topics.insert(130, String::from("unsafe blocks"));
    topics.insert(131, String::from("Raw pointers (*const T, *mut T)"));
    topics.insert(132, String::from("unsafe functions"));
    topics.insert(133, String::from("unsafe traits"));
    topics.insert(134, String::from("FFI (Foreign Function Interface)"));

    // Testing
    topics.insert(135, String::from("#[test] attribute"));
    topics.insert(136, String::from("assert!, assert_eq!, assert_ne!"));
    topics.insert(137, String::from("#[should_panic]"));
    topics.insert(138, String::from("Result in tests"));
    topics.insert(139, String::from("Integration tests (tests/ directory)"));
    topics.insert(140, String::from("#[cfg(test)]"));

    topics
}

pub fn len() -> usize {
    get().len()
}
pub fn rand_topic() -> Option<String> {
    let id = rand::random_range(1..=len() as u32);
    get().get(&id).cloned()
}
