# Folder Structure

After Creating any project (Other than `tonic`), the folder structure will be as follows:

```
src
├── main.rs
├── db.rs

CARGO.toml
```


For `tonic`
```
src
├── lib.rs
├── db.rs
├── server.rs
├── client.rs

CARGO.toml
```

`db.rs`: Contains the code for the database connection pool. There is `async` call even on a synchronous function, but theoritically it should be fine. I am not sure if it is a good practice or not. Please let me know if it creates a problem.
