# web_redis

```bash
cargo run
# or
REDIS_URL="redis://localhost/0" cargo run
```

### perf
```text
➜  ~ wrk -t12 -c200 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.40ms    1.37ms  42.93ms   81.85%
    Req/Sec     1.41k    83.00     1.69k    79.17%
  505619 requests in 30.02s, 58.35MB read
  Socket errors: connect 0, read 43, write 0, timeout 0
Requests/sec:  16845.26
Transfer/sec:      1.94MB
```