#! /bin/bash
echo "Starting benchmarks..."
for i in {1..5}
do
    wrk -t6 -c512 -d30s http://localhost:3000
done
echo "Benchmarks finished"