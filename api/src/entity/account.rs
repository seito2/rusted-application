use crate::diesel::RunQueryDsl;
use crate::models::Account;
use crate::schema::{self, accounts};
use diesel::query_dsl::methods::FindDsl;
use diesel::{insert_into, PgConnection, QueryResult};

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct AccountForm {
    name: String,
    email: String,
    password: String,
}

pub struct AccountEntity {
    pub(crate) connection: PgConnection,
}

impl AccountEntity {
    pub fn create(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<Account, diesel::result::Error> {
        let account_form = AccountForm {
            name,
            email,
            password,
        };

        let inserted = insert_into(schema::accounts::table)
            .values(&account_form)
            .returning(accounts::id)
            .get_result(&self.connection)?;

        return self.get(inserted);
    }

    pub fn all(&self) -> Vec<Account> {
        schema::accounts::table
            .load::<Account>(&self.connection)
            .expect("Error loading accounts")
    }

    pub fn get(&self, account_id: i32) -> Result<Account, diesel::result::Error> {
        schema::accounts::dsl::accounts
            .find(account_id)
            .first(&self.connection)
    }
}
