use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::{NewUser, User};
use crate::schema::{porters, users};

pub struct UserUpdateData {
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service: Option<String>,
    pub is_active: bool,
}

pub struct UserRepository;

impl UserRepository {
    pub fn list_with_porter_id(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Option<String>)>> {
        users::table
            .left_join(porters::table.on(porters::user_id.eq(users::id)))
            .select((users::all_columns, porters::id.nullable()))
            .order(users::created_at.desc())
            .load::<(User, Option<String>)>(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, user_id: &str) -> QueryResult<User> {
        users::table.find(user_id).first::<User>(conn)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    }

    pub fn find_by_username_optional(
        conn: &mut PgConnection,
        username: &str,
    ) -> QueryResult<Option<User>> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()
    }

    pub fn insert(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: &str,
        data: UserUpdateData,
    ) -> QueryResult<User> {
        diesel::update(users::table.find(user_id))
            .set((
                users::username.eq(data.username),
                users::password_hash.eq(data.password_hash),
                users::role.eq(data.role),
                users::first_name.eq(data.first_name),
                users::last_name.eq(data.last_name),
                users::email.eq(data.email),
                users::service.eq(data.service),
                users::is_active.eq(data.is_active),
            ))
            .get_result::<User>(conn)
    }

    pub fn set_active(
        conn: &mut PgConnection,
        user_id: &str,
        is_active: bool,
    ) -> QueryResult<User> {
        diesel::update(users::table.find(user_id))
            .set(users::is_active.eq(is_active))
            .get_result::<User>(conn)
    }

    pub fn delete_by_id(conn: &mut PgConnection, user_id: &str) -> QueryResult<usize> {
        diesel::delete(users::table.find(user_id)).execute(conn)
    }
}
