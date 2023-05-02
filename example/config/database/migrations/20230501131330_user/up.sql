
CREATE TABLE Roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);
CREATE TABLE Users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    password VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    role_id INT,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);
CREATE TABLE Permissions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    role_id INT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES Roles(id)
);
CREATE TABLE Sessions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    session_token VARCHAR(255) NOT NULL UNIQUE,
    session_data TEXT,
    expiration_date DATETIME,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);INSERT INTO Roles (name) VALUES ('admin');INSERT INTO Permissions (name, role_id) VALUES ('create_user', 1);INSERT INTO Permissions (name, role_id) VALUES ('read_user', 1);INSERT INTO Users (password, username, role_id) VALUES ('$2b$12$dU/ZoDYRPtOAKW4STgWz0ew22aH.EDXA5Q80E.XYvyE7fbgldPpXG', 'admin', 1);