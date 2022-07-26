#[derive(Serialize, Deserialize, Debug)]
pub enum GraphQLOperation {
    Query,
    Mutation,
    Subscription
}