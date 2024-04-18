# Rash 🦀 - reasonable again shell

A bash 🐚 alternative with reasonable syntax, written in Rust.

# Basic syntax 📜
```go
  for i = 0; i < 10; ++i; {
      if i == 7; {
        print("Lucky number!");
      }
  }
```
# Types 📦
- string ("hello world" || 'hello world')
- int
- float
- bool

# TODO ✅
- [x] Add types
- [x] Add variables
- [x] Add loops, conditions and basic functions
- [ ] More programming fundamentals
- [ ] LSP???

# Benchmark 📈

Please don't look here

| Language     | Time      |
|--------------|-----------|
| Rash 🚀      | 13.28s    |
| Python 🐍    | 1.45s     |
| Rust 🦀      | 0.35s     |
| JavaScript 🕸️| 4.52s     |

### Code tested

```go
for i = 0; i < 100; ++i; {
	for j = 0; j < 100; ++j; {
		for k = 0; k < 100; ++k; {
			print(i * j * k;);
		}
	}
}
```

Honestly I'm quite pleased with this outcome, this language is pretty naively implemented so it's cool to see it working
