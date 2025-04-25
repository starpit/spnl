# SPL: SPan Language UX Experimentation

This repository consists of Rust workspaces that implement
- **spl_ast**: Span Language AST and an `spl!` Rust macro that generates the AST
- **spl_run**: Runs a given SPL AST
- **spl**: A trivial command line front end

## SPL AST

The AST is currently specified using a Lisp-like DSL. The `spl!` macro
accepts this DSL and creates the underlying data structures.

### `g` `plus` and `cross`

There are three main SPL operators signified in the DSL as `g`,
`plus`, and `cross`:

- `(g model input)`: Used to ask a model to generate new output
- `(plus d1 d2 ...)`: Used to signify that the given items `d1`, `d2`,
  ... are to be considered independent of eachother.
- `(cross d1 d2 ...)`: Ibid, except now the items are to be considered
  as having a linear dependence on eachother.

### Helpers: ask, read, file, let
In addition to these three core operators, the DSL offers some
convenience operators. These include `(ask message)` which prompts the
user for a string; `askn` asks for an integer value, and `askf` asks
for a floating point value. As in Lisp, one may use `(let ((var1 val2)
(var2 val2) ...) subprogram)` to create scoped variable bindings for
use in the given sub-program. Use `(file filepath)` to read in a
string from the given local file path.

### Examples:

This will generate (`g`) some output, using the given "model server/model", provided with the given input "Hello world":
```lisp
(g "ollama/granite3.2:2b" "Hello world")
```

Same, except ask the user (or read from a file) which prompt should be send to the generation.
```lisp
(g "ollama/granite3.2:2b" (read "What should I ask the model?"))
(g "ollama/granite3.2:2b" (file "./prompt.txt"))
```

Send a sequence of prompts to the model:
```lisp
(g "ollama/granite3.2:2b" (cross (read "What should I ask the model?")  (file "./prompt.txt")))
```

The `g` operator also accepts optional max tokens and temperature
options. Here analyze three independent inputs, each generated with
max tokens of 1000 and a temperature of 0.3:
```lisp
(g "ollama/granite3.2:2b"
   (cross "Pick the best one"
          (plus (g "ollama/granite3.2:2b" "Generate a fun email" 1000 0.3)
                (g "ollama/granite3.2:2b" "Generate a fun email" 1000 0.3)
                (g "ollama/granite3.2:2b" "Generate a fun email" 1000 0.3))))
```
