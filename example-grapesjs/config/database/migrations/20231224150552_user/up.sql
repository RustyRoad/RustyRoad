
CREATE TABLE Roles (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    password TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    role_id INTEGER,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);
CREATE TABLE Permissions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    role_id INTEGER NOT NULL,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);
CREATE TABLE Sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    session_data TEXT,
    expiration_date TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);INSERT INTO Roles (name) VALUES ('admin');INSERT INTO Permissions (name, role_id) VALUES ('create_user', 1);INSERT INTO Permissions (name, role_id) VALUES ('read_user', 1);INSERT INTO Users (password, username, role_id) VALUES ('$2b$12$Kq87Efx2THn8P8Lla.ZYG.oem3AgpWnygWwljb5Q9eEvYuguRBhFa', 'admin', 1);