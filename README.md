# rust-graph-play
Playground for graphql in Rust. 

# References

1. https://github.com/joshua-cooper/rust-graphql-intro
1. https://www.lpalmieri.com/posts/2020-05-10-announcement-zero-to-production-in-rust/

# Getting started
docker run --rm -it -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:alpine

# Mutations

## Create customer

```rust
mutation {
  registerCustomer(
    name: "Tim Apple", 
    age: 14, 
    email: "tim1@website.com", 
    address:"Some place green"
  ) {
    id,
    name,
    age,
  } 
}
```

# Change email
```rust
mutation {
  registerCustomer(
    name: "Tim Apple", 
    age: 14, 
    email: "tim1@website.com", 
    address:"Some place green"
  ) {
    id,
    name,
    age,
  } 
}
```