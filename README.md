My follow-along repo for the [zero2prod](https://www.zero2prod.com/index.html) rust book.

# dev

run 'cargo watch -x check -x test -x run'

curl -X GET 127.0.0.1:8000/health

## db migration

export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table

# test
start pg image: `./scripts/init_db.sh`

`cargo test`

`TEST_LOG=TRUE cargo test health_check_works | bunyan`
