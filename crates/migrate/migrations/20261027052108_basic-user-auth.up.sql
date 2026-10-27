CREATE TABLE IF NOT EXISTS auth.users (
    id INT PRIMARY KEY,
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255),
    created_date DATE DEFAULT CURRENT_DATE

);
