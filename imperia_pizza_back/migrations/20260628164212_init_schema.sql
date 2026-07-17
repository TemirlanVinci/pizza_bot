CREATE TABLE categories (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE branches (
    id          INTEGER PRIMARY KEY,                
    name        VARCHAR(100) NOT NULL,             
    address     VARCHAR(255) NOT NULL,             
    work_hours  VARCHAR(100) NOT NULL,             
    map_link    TEXT,                             
    phone       VARCHAR(20),                       
    is_active   BOOLEAN NOT NULL DEFAULT true,   
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP 
);

CREATE TABLE products (
    id          INTEGER PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id),
    name        TEXT NOT NULL,
    description TEXT,
    price       INTEGER NOT NULL,  
    weight      INTEGER NOT NULL,     
    image_url   TEXT NOT NULL
);


DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id          BIGSERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE favorites (
    id SERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(telegram_id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    UNIQUE(user_id, product_id)
);


CREATE TABLE cart_items (
    id SERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(telegram_id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    UNIQUE(user_id, product_id)
);