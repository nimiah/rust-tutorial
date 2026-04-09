# ISSUES

### Conflict values between docker-compose & .env

- docker-compose `POSTGRES_DB` = `mydatabase`, host is `localhost`

```
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: admin123
      POSTGRES_DB: mydatabase
```

- ❌ but POSTGRES_DB in `.env` = `postgres`

```env
DATABASE_URL=postgresql://admin:admin123@192.168.111.10:5432/postgres
```

- ❌ but host in `.env` = `192.168.111.10`
