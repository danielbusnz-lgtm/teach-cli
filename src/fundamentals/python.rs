use std::collections::BTreeMap;

pub fn get() -> BTreeMap<u32, String> {
    let mut topics: BTreeMap<u32, String> = BTreeMap::new();

    topics.insert(1, String::from("Variables and dynamic typing"));
    topics.insert(2, String::from("Primitive types (int, float, str, bool)"));
    topics.insert(3, String::from("Collection types (list, tuple, set, dict)"));
    topics.insert(4, String::from("Type hints and annotations"));
    topics.insert(5, String::from("Generic types (list[int], dict[str, int])"));
    topics.insert(6, String::from("Union types (int | str)"));
    topics.insert(7, String::from("if/elif/else statements"));
    topics.insert(8, String::from("for loops"));
    topics.insert(9, String::from("while loops"));
    topics.insert(10, String::from("match statements (pattern matching)"));
    topics.insert(11, String::from("break and continue"));
    topics.insert(12, String::from("Function definitions (def)"));
    topics.insert(
        13,
        String::from("Parameters (positional, keyword, default)"),
    );
    topics.insert(14, String::from("Return values"));
    topics.insert(15, String::from("*args (variable positional arguments)"));
    topics.insert(16, String::from("**kwargs (variable keyword arguments)"));
    topics.insert(17, String::from("Decorators"));
    topics.insert(18, String::from("Lambda functions"));
    topics.insert(19, String::from("Local scope"));
    topics.insert(20, String::from("Enclosing scope"));
    topics.insert(21, String::from("Global scope"));
    topics.insert(22, String::from("Built-in scope"));
    topics.insert(23, String::from("LEGB rule"));
    topics.insert(24, String::from("global keyword"));
    topics.insert(25, String::from("nonlocal keyword"));
    topics.insert(26, String::from("Class definitions"));
    topics.insert(27, String::from("Objects and instances"));
    topics.insert(28, String::from("self reference"));
    topics.insert(29, String::from("__init__ constructor"));
    topics.insert(30, String::from("Instance variables"));
    topics.insert(31, String::from("Class variables"));
    topics.insert(32, String::from("Methods"));
    topics.insert(33, String::from("Single inheritance"));
    topics.insert(34, String::from("Multiple inheritance"));
    topics.insert(35, String::from("Method Resolution Order (MRO)"));
    topics.insert(36, String::from("super()"));
    topics.insert(37, String::from("Method overriding"));
    topics.insert(38, String::from("isinstance()"));
    topics.insert(39, String::from("issubclass()"));
    topics.insert(40, String::from("Public attributes"));
    topics.insert(41, String::from("Protected attributes (_name)"));
    topics.insert(42, String::from("Private attributes (__name)"));
    topics.insert(43, String::from("@property decorator"));
    topics.insert(44, String::from("@attribute.setter decorator"));
    topics.insert(45, String::from("Polymorphism"));
    topics.insert(46, String::from("Duck typing"));
    topics.insert(47, String::from("Operator overloading"));
    topics.insert(48, String::from("Abstract Base Classes (ABC)"));
    topics.insert(49, String::from("@abstractmethod decorator"));
    topics.insert(50, String::from("Protocol (structural subtyping)"));
    topics.insert(51, String::from("@dataclass decorator"));
    topics.insert(52, String::from("Dataclass fields"));
    topics.insert(53, String::from("frozen=True (immutable dataclasses)"));
    topics.insert(
        54,
        String::from("Descriptors (__get__, __set__, __delete__)"),
    );
    topics.insert(55, String::from("Enum class"));
    topics.insert(56, String::from("Enum member access"));
    topics.insert(57, String::from("Enum .value attribute"));
    topics.insert(58, String::from("auto() for automatic values"));
    topics.insert(59, String::from("IntEnum"));
    topics.insert(60, String::from("StrEnum"));
    topics.insert(61, String::from("Flag"));
    topics.insert(62, String::from("IntFlag"));
    topics.insert(63, String::from("@unique decorator"));
    topics.insert(64, String::from("Type annotations"));
    topics.insert(65, String::from("Type checkers (mypy, pyright)"));
    topics.insert(66, String::from("Generics (TypeVar, Generic)"));
    topics.insert(67, String::from("Literal types"));
    topics.insert(68, String::from("TypeGuard"));
    topics.insert(69, String::from("ParamSpec"));
    topics.insert(70, String::from("async def (coroutine functions)"));
    topics.insert(71, String::from("await keyword"));
    topics.insert(72, String::from("Coroutine objects"));
    topics.insert(73, String::from("Awaitable objects"));
    topics.insert(74, String::from("asyncio.create_task()"));
    topics.insert(75, String::from("Task cancellation"));
    topics.insert(76, String::from("TaskGroup (async with)"));
    topics.insert(77, String::from("Event loop"));
    topics.insert(78, String::from("asyncio.run()"));
    topics.insert(79, String::from("asyncio.gather()"));
    topics.insert(80, String::from("asyncio.wait()"));
    topics.insert(81, String::from("asyncio.as_completed()"));
    topics.insert(
        82,
        String::from("Async context managers (__aenter__, __aexit__)"),
    );
    topics.insert(83, String::from("async with statement"));
    topics.insert(84, String::from("Async iterators (__aiter__, __anext__)"));
    topics.insert(85, String::from("async for statement"));
    topics.insert(86, String::from("Async generators"));
    topics.insert(87, String::from("try block"));
    topics.insert(88, String::from("except block"));
    topics.insert(89, String::from("else block"));
    topics.insert(90, String::from("finally block"));
    topics.insert(91, String::from("Exception hierarchy"));
    topics.insert(92, String::from("Multiple except clauses"));
    topics.insert(93, String::from("Exception chaining (raise from)"));
    topics.insert(94, String::from("Custom exceptions"));
    topics.insert(95, String::from("Exception attributes"));
    topics.insert(96, String::from("with statement"));
    topics.insert(97, String::from("__enter__ method"));
    topics.insert(98, String::from("__exit__ method"));
    topics.insert(99, String::from("contextlib module"));
    topics.insert(100, String::from("@contextmanager decorator"));
    topics.insert(101, String::from("ExitStack"));
    topics.insert(102, String::from("List comprehensions"));
    topics.insert(103, String::from("Dict comprehensions"));
    topics.insert(104, String::from("Set comprehensions"));
    topics.insert(105, String::from("Nested comprehensions"));
    topics.insert(106, String::from("Conditional comprehensions"));
    topics.insert(107, String::from("Generator expressions"));
    topics.insert(108, String::from("Lazy evaluation"));
    topics.insert(109, String::from("yield keyword"));
    topics.insert(110, String::from("yield from"));
    topics.insert(111, String::from("Generator state"));
    topics.insert(112, String::from("__str__ (string representation)"));
    topics.insert(113, String::from("__repr__ (debug representation)"));
    topics.insert(114, String::from("__len__ (length)"));
    topics.insert(115, String::from("__getitem__ (indexing)"));
    topics.insert(116, String::from("__setitem__ (item assignment)"));
    topics.insert(117, String::from("__call__ (callable objects)"));
    topics.insert(118, String::from("__enter__/__exit__ (context manager)"));
    topics.insert(119, String::from("__add__, __sub__, __mul__ (operators)"));
    topics.insert(120, String::from("__eq__, __lt__, __gt__ (comparisons)"));
    topics.insert(121, String::from("Iterable (__iter__)"));
    topics.insert(122, String::from("Iterator (__iter__ + __next__)"));
    topics.insert(123, String::from("StopIteration exception"));
    topics.insert(124, String::from("iter() function"));
    topics.insert(125, String::from("next() function"));

    topics
}

pub fn len() -> usize {
    get().len()
}

pub fn rand_topic() -> Option<String> {
    let id = rand::random_range(1..=len() as u32);
    get().get(&id).cloned()
}
