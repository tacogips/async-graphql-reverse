### Graphql Schema definition to rust code for async-graphql

this repository is heavily inspired by https://github.com/atsuhiro/codegen-for-async-graphql

### install
```
cargo install --git https://github.com/tacogips/async-graphql-reverse --branch main async-graphql-reverse
```

### usage
```
# generate schema
async-graphql-reverse --input-schema examples/simple/input/schema.graphql --output-dir examples/simple/output schema
```
### Lint error

The generated codes would violate the clippy lint rules below.

- [clone_on_copy](https://rust-lang.github.io/rust-clippy/master/#clone_on_copy)
- [too_many_arguments](https://rust-lang.github.io/rust-clippy/master/#too_many_arguments)

You can suppress the lint errors adding `allow` attribute .
```rust
// On top of lib.rs or main.rs of your crate
#![allow(clippy::clone_on_copy,clippy::too_many_arguments)]
```

## Supported Features

- [x] Object
- [x] Object
- [x] InputObject
- [x] Enum
- [x] Interface
- [x] Union
- [ ] Subscriber
- [ ] Description
	- [ ] Object description
	- [x] Object Resolver description
	- [ ] Input object description
	- [ ] Enum description
	- [ ] Union description
	- [ ] Interface description
- [ ] Default value
