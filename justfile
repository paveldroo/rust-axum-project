# Watch dev files
watch:
    cargo watch -q -c -w src/ -x run

watch-test:
    cargo watch -q -c -w src/ -w tests/ -x "test -q quick_dev -- --no-capture"
