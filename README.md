# Waaagh Language

**NOTE(not implemented): This language is currently in design and not implemented. It might take years before you can actually use it.**

I got much frustrated with existing programming languages, so I decide to make a language for myself.

## Features Decision

Many features stolen from other languages which I think is the best practice.

### Lisp-Like Syntax

- Minimal core language
- The S-expression is "mathematical" or unopinioned enough
- The simplest one is the most extensible one => read macro (Racket Scribble)

### URI Based Module

```lisp
(import "https://example.com/foo.wgh"
 foo)
(import "git+https://github.com/example/foo?dir=bar.wgh"
 (bar baz))

(foo:x)
```

- Stolen from JS/TS/Nix
- Encourages hacking => how to encourage hospitable code
- JSON-LD like prefix syntax => keyword renaming
- Type System have be more transparent due to version management

### Dependent Type

- "Pi-Sigma" system without builtin inductive type
- Inductive type through macro
- With universes
- Take inspiration from neut 0.2

### Gradually Compiled

- Gradually compiled: JIT -> AOT
- Fast for one-run script
- Also fast for cpu bounded task

## Mascot

- Frog 蛙(pronounced waa)
- Frog is also a green-skined creature that says "waaagh" 
