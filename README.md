### Graphql Schema to async-graphql definition

this repository is heavily inspired by https://github.com/atsuhiro/codegen-for-async-graphql

### run example
```
cargo run --bin async-graphql-reverse -- --input-schema examples/simple/input/schema.graphql --output-dir examples/simple/output
```

### install
```
cargo install --git https://github.com/tacogips/async-graphql-reverse --branch master async-graphql-reverse
```


## Supported Features
- [x] Object
- [x] InputObject
- [x] Enum
- [x] Interface
- [x] Union
- [ ] Subscriber
- [ ] Description
	- [x] resolver description
	- [ ] object description
	- [ ] input object description
