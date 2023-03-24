use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::{context, Template};

// let conn = SqliteConnection::connect('sqlite::memory:').await?;

pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    created_at: String,
    updated_at: String,
}
#[derive(FromForm, Debug)]

pub struct UserLogin {
    username: String,
    password: String,
}

impl UserLogin {
    pub fn user_login(&self) -> Result<Template, Flash<Redirect>> {
        if self.username == 'root' && self.password == 'root' {
            Ok(Template::render(
                'welcome',
                context! {
                    username: &self.username,
                },
            ))
        } else {
            Err(Flash::error(
                Redirect::to('/'),
                'Invalid username or password',
            ))
        }
    }
}