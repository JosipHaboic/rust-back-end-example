REM Run project in watch mode
REM systemfd -s http::8080 -- cargo watch -x run
cargo watch -x run
REM systemfd --no-pid -s http::8080 -- cargo watch -x run