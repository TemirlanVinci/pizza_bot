CREATE TABLE categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
CREATE TABLE branches (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(255) NOT NULL,
    work_hours VARCHAR(100) NOT NULL,
    map_link TEXT,
    phone VARCHAR(20),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id),
    name TEXT NOT NULL,
    description TEXT,
    price INTEGER NOT NULL,
    weight INTEGER NOT NULL,
    image_url TEXT NOT NULL
);
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
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
-- =========================================================
-- НОВЫЕ ТАБЛИЦЫ
-- =========================================================
-- Админы бота (для проверки прав в PATCH /admin/orders/.../status и /admin/users/ban)
CREATE TABLE admins (
    telegram_id BIGINT PRIMARY KEY,
    name VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW()
);
-- Заказы. user_name/phone_number/address дублируются намеренно —
-- это снапшот на момент заказа, чтобы история не менялась задним числом,
-- если пользователь потом сменит номер или адрес.
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(telegram_id),
    user_name VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    delivery_type VARCHAR(10) NOT NULL CHECK (delivery_type IN ('delivery', 'pickup')),
    address TEXT NOT NULL,
    payment_method VARCHAR(20) NOT NULL CHECK (payment_method IN ('cash', 'visa_courier')),
    status VARCHAR(20) NOT NULL DEFAULT 'confirmed' CHECK (
        status IN (
            'confirmed',
            'cooking',
            'delivering',
            'completed',
            'cancelled'
        )
    ),
    total_price INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
-- Позиции внутри заказа. name и price_at_purchase — тоже снапшот
-- на случай, если товар потом переименуют, удалят или изменят цену.
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id),
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    price_at_purchase INTEGER NOT NULL
);
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
-- Бан-лист. phone_number намеренно НЕ уникален и может быть NULL-независим
-- от telegram_id — банить можно и по номеру отдельно (для поимки шутников,
-- которые заводят новые Telegram-аккаунты, но используют старый номер).
CREATE TABLE banned_users (
    id SERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE,
    phone_number VARCHAR(20),
    ban_reason TEXT,
    banned_by BIGINT REFERENCES admins(telegram_id),
    banned_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX idx_banned_users_phone ON banned_users(phone_number);
-- История телефонов пользователя (не перезаписываем, а копим)
CREATE TABLE user_phones (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(telegram_id) ON DELETE CASCADE,
    phone_number VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX idx_user_phones_user_id ON user_phones(user_id);
CREATE INDEX idx_user_phones_phone ON user_phones(phone_number);
-- История адресов пользователя (не перезаписываем, а копим)
CREATE TABLE user_addresses (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(telegram_id) ON DELETE CASCADE,
    address TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX idx_user_addresses_user_id ON user_addresses(user_id);
CREATE INDEX idx_user_addresses_address ON user_addresses(address);