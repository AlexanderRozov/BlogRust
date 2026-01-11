
### Публичная часть
- Список постов на главной странице
- Страница поста с комментариями
- Добавление комментариев через форму

### Админка
- Вход в админку (username: `admin`, password: `admin123`)
- CRUD постов
- Удаление комментариев

## Установка и запуск

### Локальный запуск

1. Установите зависимости:
   - Rust (последняя стабильная версия)
   - PostgreSQL

2. Создайте базу данных:
```bash
createdb blog
```

3. Настройте переменные окружения (создайте `.env` файл):
```
DATABASE_URL=postgresql://blog:blog@localhost:5432/blog
PORT=3000
SESSION_SECRET=dev-secret-key-change-in-production
```

4. Запустите миграции:
```bash
sqlx migrate run
```

5. Сгенерируйте хэш пароля для админа (если нужно):
```bash
cargo run --bin hash-password -- admin123
```

6. Запустите сервер:
```bash
cargo run --release
```

Сервер будет доступен на `http://localhost:3000`

### Docker

1. Запустите с помощью docker-compose:
```bash
docker-compose up --build
```

2. Откройте браузер: `http://localhost:3000`

## Генерация хэша пароля

Для создания нового пользователя или изменения пароля:

```bash
cargo run --bin hash-password -- ваш_пароль
```

Затем обновите хэш в базе данных или миграции.

## Структура проекта

```
blog/
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── migrations/
│   └── 001_init.sql
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── db.rs
│   ├── bin/
│   │   └── hash-password.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── post.rs
│   │   └── comment.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── public.rs
│   │   └── admin.rs
│   ├── templates/
│   │   ├── mod.rs
│   │   ├── base.html
│   │   ├── index.html
│   │   ├── post.html
│   │   └── admin/
│   │       ├── login.html
│   │       └── dashboard.html
│   └── static/
│       └── style.css
```

## Тестирование

Проект включает unit-тесты и интеграционные тесты.

### Unit-тесты

Unit-тесты находятся в каждом модуле в блоке `#[cfg(test)]`. Они тестируют отдельные функции и методы:

```bash
cargo test
```

Запуск только unit-тестов (без интеграционных):
```bash
cargo test --lib
```

### Интеграционные тесты

Интеграционные тесты находятся в директории `tests/` и требуют запущенной базы данных:

1. Создайте тестовую БД:
```bash
createdb blog_test
```

2. Установите переменную окружения:
```bash
export DATABASE_URL=postgresql://blog:blog@localhost:5432/blog_test
```

3. Запустите миграции:
```bash
sqlx migrate run --database-url $DATABASE_URL
```

4. Запустите интеграционные тесты:
```bash
cargo test --test integration_test
```

Или все тесты с базой данных:
```bash
cargo test -- --ignored
```

### Покрытие кода

Для анализа покрытия кода тестами можно использовать `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Разработка

### Добавление новых функций

Проект спроектирован для легкого расширения:

- **Markdown поддержка**: Добавьте парсер markdown в модель Post
- **RSS**: Добавьте новый route `/rss.xml`
- **Теги**: Добавьте таблицу `tags` и связь many-to-many с постами
- **Full-text search**: Используйте PostgreSQL full-text search

### Обновление зависимостей

Для обновления всех зависимостей до актуальных версий:

```bash
cargo update
```

## Лицензия

Apache 2.0

