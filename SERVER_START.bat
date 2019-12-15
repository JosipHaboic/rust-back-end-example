REM Run project in watch mode
REM systemfd -s http::8080 -- cargo watch -x run
SET RUST_BACKTRACE=0
cargo watch -x run
REM systemfd --no-pid -s http::8080 -- cargo watch -x run