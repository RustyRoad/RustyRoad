use actix_identity::Identity;
use actix_web::HttpMessage;
use actix_web::HttpResponseBuilder;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use bcrypt::verify;
use rustyroad::database::Database;
use serde::Deserialize;
use sqlx::PgPool;

use tera::Context;
use tera::Tera;

pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize, Debug)]
pub struct UserLogin {
    username: String,
    password: String,
}

impl UserLogin {
    async fn get_hashed_password_from_db(
        username: &str,
        pool: &sqlx::PgPool,
    ) -> Result<String, sqlx::Error> {
        let row: (String,) = sqlx::query_as("SELECT password FROM Users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }
    pub async fn user_login(
        &self,
        tmpl: web::Data<Tera>,
        database: Database,
        request: HttpRequest
    ) -> Result<HttpResponse, Error> {
        let mut ctx = Context::new();

        // Create the database URL
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database.username, database.password, database.host, database.port, database.name
        );

        // Create the database connection pool
        let db_pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres.");

        // Retrieve the hashed password from the database
        match Self::get_hashed_password_from_db(&self.username, &db_pool).await {
            Ok(hashed_password) => {
                match verify(&self.password, &hashed_password) {
                    Ok(password_match) => {
                        if password_match {
                            // Here you can set the identity directly
                            Identity::login(&request.extensions(), self.username.clone()).unwrap();
                            return Ok(
                                // Forwards the request to the dashboard
                                HttpResponse::Found() // <- Redirect to the dashboard
                                    .append_header(("Location", "/dashboard"))
                                    .finish(),
                            );

                        } else {
                            ctx.insert("error", "Invalid username or password");
                            let rendered = tmpl.render("pages/login.html.tera", &ctx).unwrap();
                            return Ok(HttpResponse::Ok().body(rendered));
                        }
                    }
                    Err(e) => {
                        panic!("Failed to verify password: {}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Failed to retrieve hashed password from database: {}", e);
            }
        }
    }

    pub async fn user_logout(
       tmpl: web::Data<Tera>,
       user: Identity,
    ) -> Result<HttpResponse, Error> {
       user.logout();

       let mut context = Context::new();
       context.insert("route_name", "login");
       context.insert("message", "You have been logged out.");
       let rendered = tmpl.render("pages/login.html.tera", &context).unwrap();
       Ok(HttpResponse::Ok().body(rendered))
    }
}
