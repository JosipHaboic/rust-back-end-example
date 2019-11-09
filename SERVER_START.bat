REM Run project in watch mode
systemfd -s http::8080 -- cargo watch -x run
REM systemfd --no-pid -s http::8080 -- cargo watch -x run