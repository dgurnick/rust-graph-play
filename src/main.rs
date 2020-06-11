use warp::Filter;
use std::sync::Arc;
use juniper::http::graphiql::graphiql_source;
use juniper::RootNode;
use tokio_postgres::Client;
use std::convert::Infallible;
use juniper::http::GraphQLRequest;
use tokio_postgres::NoTls;

struct QueryRoot;
struct MutationRoot;

// Make the struct work with Juniper
#[derive(juniper::GraphQLObject)]
struct Customer {
    id: String,
    name: String,
    age: i32,
    email: String,
    address: String,
}

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {}

// Define the mutations
#[juniper::graphql_object(Context = Context)]
impl MutationRoot {

    async fn register_customer(
        ctx: &Context,
        name: String,
        age: i32,
        email: String,
        address: String,
    ) -> juniper::FieldResult<Customer> {
        Ok(Customer {
            id: "1".into(),
            name,
            age,
            email,
            address
        })
    }
}

type Schema = RootNode<'static, QueryRoot, MutationRoot>;

struct Context {
    client: Client,
}



#[tokio::main]
async fn main() {

    // Connect to Postgres
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
        .await
        .unwrap();

    // Let the connection run on its own
    tokio::spawn(async move {
       if let Err(e) = connection.await {
           eprintln!("Database connection error: {}", e);
       }
    });

    client.execute(
        "CREATE TABLE IF NOT EXISTS customers ( \
        id UUID PRIMARY KEY, \
        name TEXT NOT NULL, \
        age INT NOT NULL, \
        email TEXT UNIQUE NOT NULL, \
        address TEXT NOT NULL \
        )",
        &[],
        )
        .await
        .expect("Could not create customers table");


    // Define the schema
    let schema = Arc::new(Schema::new(QueryRoot, MutationRoot));

    // Make schema into a warp filter so that it is reachable via route handlers
    let schema = warp::any().map(move || Arc::clone(&schema));

    // Set up a context so we can do things like DB, etc
    let ctx = Arc::new(Context { client });

    // Make the context available to warp routes
    let ctx = warp::any().map(move || Arc::clone(&ctx));

    // Set up the graphql query handler
    let graphql_route = warp::post()
        .and(warp::path!("graphql"))
        .and(schema.clone())
        .and(ctx.clone())
        .and(warp::body::json())
        .and_then(graphql);

    // Set up the graph response (contract)
    let graphiql_route = warp::get()
        .and(warp::path("graphiql"))
        .map(|| warp::reply::html(graphiql_source("graphql")));

    // Combine our routes
    let routes = graphql_route.or(graphiql_route);

    // Start listening for requests
    warp::serve(routes).run(([127,0,0,1], 8000)).await;

}

async fn graphql (
    schema: Arc<Schema>,
    ctx: Arc<Context>,
    req: GraphQLRequest
) -> Result<impl warp::Reply, Infallible> {
    let res = req.execute_async(&schema, &ctx).await;
    let json = serde_json::to_string(&res)
        .expect("Invalid JSON response");
    Ok(json)
}
