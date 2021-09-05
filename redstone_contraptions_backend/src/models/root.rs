use super::contraption::{Contraption, ContraptionInput};
use crate::db::Pool;
use diesel::prelude::*;
use juniper::{EmptySubscription, FieldError, FieldResult, Value};

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all contraptions")]
    fn contraptions(context: &Context) -> FieldResult<Vec<Contraption>> {
        use crate::schema::contraption::dsl::*;
        let conn = context.dbpool.get().map_err(|_| {
            FieldError::new("Could not open connection to the database", Value::null())
        })?;

        contraption
            .limit(100)
            .load::<Contraption>(&conn)
            .map_err(|_| FieldError::new("Error loading contraptions", Value::null()))
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_contraption(
        context: &Context,
        new_contraption: ContraptionInput,
    ) -> FieldResult<Contraption> {
        use crate::schema::contraption;

        let conn = context.dbpool.get().map_err(|_| {
            FieldError::new("Could not open connection to the database", Value::null())
        })?;

        diesel::insert_into(contraption::table)
            .values(&new_contraption)
            .get_result::<Contraption>(&conn)
            .map_err(|_| FieldError::new("Error creating contraption", Value::null()))
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
