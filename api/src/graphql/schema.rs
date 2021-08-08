use juniper::{RootNode, EmptyMutation, EmptySubscription};
use crate::graphql::query::Query;
use crate::database::Context;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}