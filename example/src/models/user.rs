use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::Cookie;
use actix_web::cookie::*;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use bcrypt::verify;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rustyroad::database::Database;
use serde::Deserialize;
use sqlx::PgPool;

use tera::Context;
use tera::Tera;

// Added chrono for handling DateTime
use chrono::offset::Utc;
use chrono::Duration;

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
                            // Generate a new session token
                            let session_token: String = rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(30)
                                .map(char::from)
                                .collect();

                            let expires = OffsetDateTime::now_utc() + time::Duration::days(1);

                            // Set the session token in a cookie
                            let mut session_cookie =
                                Cookie::new("session_token", session_token.clone());
                            session_cookie.set_secure(false); // Set to true if using HTTPS
                            session_cookie.set_http_only(true);
                            session_cookie.set_same_site(actix_web::cookie::SameSite::Strict);
                            session_cookie.set_expires(expires);

                            // Set the session token cookie in the response
                            let mut response = HttpResponse::Ok().body("Login successful!");
                            response.add_cookie(&session_cookie).unwrap();

                            // Set the session expiration date to 1 day from now
                            let expiration_date = Utc::now() + Duration::days(1);

                            // Insert the new session into the database
                            let result = sqlx::query(
                                "INSERT INTO Sessions (user_id, session_token, expiration_date) VALUES ((SELECT id FROM Users WHERE username = $1), $2, $3)",
                            )
                            .bind(&self.username)
                            .bind(&session_token)
                            .bind(expiration_date)
                            .execute(&db_pool)
                            .await;

                            match result {
                                Ok(rows_affected) => {
                                    if rows_affected.rows_affected() == 0 {
                                        // Handle the case where no rows were affected (e.g., log an error or return an error message)
                                        ctx.insert("error", "Failed to create a new session");
                                        let rendered =
                                            tmpl.render("pages/login.html.tera", &ctx).unwrap();
                                        return Ok(HttpResponse::Ok().body(rendered));
                                    } else {
                                        ctx.insert("username", &self.username);
                                        ctx.insert("session_token", &session_token);
                                        let rendered =
                                            tmpl.render("pages/dashboard.html.tera", &ctx).unwrap();
                                        return Ok(HttpResponse::Ok().body(rendered));
                                    }
                                }
                                Err(e) => {
                                    panic!("Failed to execute query: {}", e);
                                }
                            }
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
        database: Database,
        req: HttpRequest,
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

        // Retrieve the session token from the cookies
        let session_token = req
            .cookie("session_token")
            .map(|cookie| cookie.value().to_string());

        if let Some(token) = session_token {
            // Delete the session from the database
            let result = sqlx::query("DELETE FROM Sessions WHERE session_token = ?")
                .bind(&token)
                .execute(&db_pool)
                .await;

            match result {
                Ok(_) => {
                    // Remove the session token cookie from the response
                    let mut response = HttpResponse::Ok().body("Logout successful!");
                    response.del_cookie("session_token");

                    let rendered = tmpl.render("pages/login.html.tera", &ctx).unwrap();
                    return Ok(HttpResponse::Ok().body(rendered));
                }
                Err(e) => {
                    panic!("Failed to delete session from database: {}", e);
                }
            }
        } else {
            ctx.insert("error", "No session token found");
            let rendered = tmpl.render("pages/login.html.tera", &ctx).unwrap();
            return Ok(HttpResponse::Ok().body(rendered));
        }
    }
}
