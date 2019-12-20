use juniper;

use super::product::Product;
use super::root::Context;
use super::util;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn products(&self, context: &Context) -> Vec<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let products = conn
            .query("SELECT * FROM products where user_id=:user_id", &[&self.id])
            .map(|result| util::create_products_from_rows(result))
            .unwrap();
        products
    }
}
