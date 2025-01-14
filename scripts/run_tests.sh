cd ../tests/
RUST_TEST_THREADS=1 cargo test --test unit_tests
TEST_EXIT_CODE=$?

if [ $TEST_EXIT_CODE -eq 0 ]; then
  echo "Tests passed successfully!"
else
  echo "Tests failed with exit code $TEST_EXIT_CODE."
fi

echo "Cleaning up databases..."
cd ./..
rm test.db
rm rustentials.db
echo "All done"

exit $TEST_EXIT_CODE
