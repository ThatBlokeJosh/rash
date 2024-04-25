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
```go 
// Shell command syntax
url = "https://pokey.remington.boo" // Great website 
c"curl ${url}"
```

# Install 🏹
```bash
git clone https://github.com/ThatBlokeJosh/rash.git
cd rash
chmod +x setup
./setup
```
Or you can use the provided rash script like so
```bash 
rash setup.rash
```

# TODO ✅
- [x] Add types
- [x] Add variables
- [x] Add loops, conditions and basic functions
- [x] Function definitons, importing
- [ ] LSP???

# Benchmark 📈


| Language     | Time      |
|--------------|-----------|
| Rash 🚀      | 4.76s    |
| Bash 🗑️      | 2.60s     |
| Python 🐍    | 1.45s     |
| Rust 🦀      | 0.35s     |
| JavaScript 🟨| 4.87s     |
| TypeScript 🟦| 4.57s     |

### Code tested

```go
for i = 0; i < 100; ++i; {
  for j = 0; j < 100; ++j; {
    for k = 0; k < 100; ++k; {
	    print(i * j * k);
    }
  }
}
```
