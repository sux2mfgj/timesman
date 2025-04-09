

Sqlite
--

```
DATABASE_URL="sqlite:./sqlite.db" cargo sqlx database create
DATABASE_URL="sqlite:./sqlite.db" cargo sqlx migrate run 
```

```
sea-orm-cli generate entity -u sqlite:./sqlite.db -o src/sqlite/
```

