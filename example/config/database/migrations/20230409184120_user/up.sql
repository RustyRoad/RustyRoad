CREATE DATABASE example DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
        CREATE TABLE Users (
	      id INTEGER PRIMARY KEY AUTOINCREMENT,
	      password TEXT NOT NULL,
	      username TEXT NOT NULL UNIQUE,
	      role_id INTEGER,
	      FOREIGN KEY (role_id) REFERENCES Roles(id)
        );
        CREATE TABLE Roles (
	      id INTEGER PRIMARY KEY AUTOINCREMENT,
	      name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE Permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
	          name TEXT NOT NULL UNIQUE,
	          role_id INTEGER NOT NULL,
	          FOREIGN KEY (role_id) REFERENCES Roles(id)
        )
        CREATE TABLE Sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
	          user_id INTEGER NOT NULL,
	          session_token TEXT NOT NULL UNIQUE,
	          session_data TEXT,
	          expiration_date DATETIME,
	          FOREIGN KEY (user_id) REFERENCES Users(id)
        )
        INSERT INTO Roles (name) VALUES ('admin');
        INSERT INTO Permissions (name, role_id) VALUES ('create_user', 1);
        INSERT INTO Permissions (name, role_id) VALUES ('read_user', 1);
        INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);CREATE DATABASE example DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
        CREATE TABLE Users (
	      id INTEGER PRIMARY KEY AUTOINCREMENT,
	      password TEXT NOT NULL,
	      username TEXT NOT NULL UNIQUE,
	      role_id INTEGER,
	      FOREIGN KEY (role_id) REFERENCES Roles(id)
        );
        CREATE TABLE Roles (
	      id INTEGER PRIMARY KEY AUTOINCREMENT,
	      name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE Permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
	          name TEXT NOT NULL UNIQUE,
	          role_id INTEGER NOT NULL,
	          FOREIGN KEY (role_id) REFERENCES Roles(id)
        )
        CREATE TABLE Sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
	          user_id INTEGER NOT NULL,
	          session_token TEXT NOT NULL UNIQUE,
	          session_data TEXT,
	          expiration_date DATETIME,
	          FOREIGN KEY (user_id) REFERENCES Users(id)
        )
        INSERT INTO Roles (name) VALUES ('admin');
        INSERT INTO Permissions (name, role_id) VALUES ('create_user', 1);
        INSERT INTO Permissions (name, role_id) VALUES ('read_user', 1);
        INSERT INTO Users (password, username, role_id) VALUES ('admin', 'admin', 1);DROP DATABASE example DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;