### Rust code generator from GraphQL schema for aysnc-graphql

this repository is heavily inspired by https://github.com/atsuhiro/codegen-for-async-graphql

### install
```
cargo install --git https://github.com/tacogips/async-graphql-reverse --branch main async-graphql-reverse
```

### usage

```
async-graphql-reverse 0.5.0
tacogips

USAGE:
    async-graphql-reverse [OPTIONS] --input-schema <INPUT_SCHEMA> --output-dir <OUTPUT_DIR> <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>
    -h, --help                           Print help information
    -i, --input-schema <INPUT_SCHEMA>
    -o, --output-dir <OUTPUT_DIR>
    -V, --version                        Print version information

SUBCOMMANDS:
    data-source
    help           Print this message or the help of the given subcommand(s)
    schema

```

#### example
see ./examples/simple/* for more details.

Say `schema.graphql` is like this.
```graphql
schema {
  query: Query
  mutation: Mutation
}

enum Status {
  REGISTERED
  EMAIL_VERIFIED
}

type Query {
  "me: Single-line comment"
  me: Me!
  active: Boolean!

  """
  this is comment for field
  multi line comment:
  """
  type: String
}

// ...
```

when you run `async-graphql-reverse` (the config file is optional)

```
# generate schema
async-graphql-reverse --input-schema examples/simple/input/schema.graphql --output-dir examples/simple/output --config examples/simple/input/reverse.toml schema

```

the following rust codes will be created at `--output-dir`

```rust
// DO NOT EDIT THIS FILE
// This file was generated by https://github.com/tacogips/async-graphql-reverse
use super::enums::SortDirection;
use super::enums::Status;
use super::input_objects::CreateFriendMutationInput;
use super::scalars::Url;
use super::unions::SearchResult;
use crate::datasource::DataSource;
use async_graphql::*;
#[derive(Debug, Clone)]
pub struct Query {}
#[Object]

#[derive(Debug, Clone)]
pub struct Query {}
#[Object]
impl Query {
    ///me: Single-line comment
    pub async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        ctx.data_unchecked::<DataSource>()
            .query_me(&ctx, self)
            .await
    }
    pub async fn active(&self, ctx: &Context<'_>) -> Result<bool> {
        ctx.data_unchecked::<DataSource>()
            .query_active(&ctx, self)
            .await
    }
    ///this is comment for field
    ///multi line comment:
    pub async fn r#type(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        ctx.data_unchecked::<DataSource>()
            .query_type(&ctx, self)
            .await
    }
    pub async fn custom_resolver(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        ctx.data_unchecked::<DataSource>()
            .query_custom_resolver(&ctx, self)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct Mutation {}
#[Object]
impl Mutation {
    // ...
}
// ...

```

Then you should implement Resolvers in a struct (default path: `crate::datasource::Datasource`) and put it into graphql schemas context.


```rust
pub struct DataSource;
impl DataSource {
    pub async fn query_me(&self, _ctx: &Context<'_>, _object: &Query) -> Result<Me> {
        Ok(Me {
            id: ID("this_is_me".to_string()),
            name: "mememe".to_string(),
            rank: 1.0,
            email: None,
            age: None,
            active: Some(true),
            web: None,
            search_second: None,
        })
    }
    pub async fn query_active(&self, _ctx: &Context<'_>, _object: &Query) -> Result<bool> {
        unimplemented!("resolver {} is unimpemented yet", "query_active")
    }
    pub async fn query_type(&self, _ctx: &Context<'_>, _object: &Query) -> Result<Option<String>> {
        unimplemented!("resolver {} is unimpemented yet", "query_type")
    }
    pub async fn mutation_create_friend_mutation(
        &self,
        _ctx: &Context<'_>,
        _object: &Mutation,
        _input: CreateFriendMutationInput,
    ) -> Result<Option<CreateFriendMutationPayload>> {
        unimplemented!(
            "resolver {} is unimpemented yet",
            "mutation_create_friend_mutation"
        )
    }

//...

```

```rust

use crate::datasource::DataSource;
use crate::output::{Mutation, Query};
//...

// in the case run with axum
#[tokio::main]
pub async fn main() {
    let schema = build_schema();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


pub fn build_schema() -> Schema<Query, Mutation, EmptySubscription> {
    let mut builder = Schema::build(Query {}, Mutation {}, EmptySubscription);
    builder = builder.data(DataSource);
    builder.finish()
}

async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}


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
