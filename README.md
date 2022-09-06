## DB

```sh
# Start the Database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# Optional psql 
docker exec -it -u postgres pg psql
```

## Tests
```sh
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'

```