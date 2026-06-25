# Watch dev files
watch:
    cargo watch -q -c -w src/ -x run

test:
    cargo test -q quick_dev -- --no-capture

watch-test:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --no-capture"
