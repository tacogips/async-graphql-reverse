### Graphql Schema definition to rust code for async-graphql

this repository is heavily inspired by https://github.com/atsuhiro/codegen-for-async-graphql

### install
```
cargo install --git https://github.com/tacogips/async-graphql-reverse --branch master async-graphql-reverse
```

### usage
```
# generate schema
async-graphql-reverse --input-schema examples/simple/input/schema.graphql --output-dir examples/simple/output schema
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
