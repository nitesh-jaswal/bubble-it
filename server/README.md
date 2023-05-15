To install SQLx CLI, run the following command:

```
cargo install sqlx-cli
```

This command creates a new database with the specified name. You can run the command with the following syntax:

```
sqlx database create
```


This command applies all the outstanding migrations to the database. 
To run migrations, use the following command:

```
sqlx migrate run
```

This command builds and runs the Rust project. If the project is configured to use a database, it will connect to the database and execute queries in the code. To run the project, use the following command:

```
cargo run
``` 
