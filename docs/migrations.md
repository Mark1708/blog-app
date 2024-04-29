# Миграции

## Установка SQLX
```shell
cargo install sqlx-cli
```

## Подготовка окружения
Создайте `.env` с переменной `DATABASE_URL`

## Работа с базой днных
```shell
sqlx database create
sqlx database drop
```

## Созданиее скрипта миграции
```shell
sqlx migrate add -r <name>
```

## Запуск миграции
```shell
sqlx migrate run
```

## Откат миграци
```shell
sqlx migrate revert
```