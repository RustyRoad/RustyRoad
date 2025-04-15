# To Do

- [] Add Migration CLI for each database
  - [x] Postgres
  - [ ] MySQL
  - [ ] SQLite
  - [ ] MongoDB
- [ ] Configure for docker with docker-compose
- [ ] React SSR or Leptos UI
  - React SSR has proven difficult with TypeScript[1](https://github.com/rileyseaburg/rust-react-ssr
- [ ] Make CLI more user friendly
  - [ ] Display Command options for all commands


1. First migration (seo_schedules):
```sh
rustyroad migration run
```

2. Verify the migrations were applied:
```sh
rustyroad migration list
```

3. Check the database schema to confirm tables were created:
```sh
rustyroad db schema
```
rustyroad migration list


Both commands are now running in terminal 2 but we're not getting output.
We need to find another way to verify the migrations were ap