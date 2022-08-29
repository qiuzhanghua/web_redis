# web_redis

```bash
cargo run
# or
REDIS_URL="redis://localhost/0" cargo run
```

### perf(r2d2 + redis)
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
### perf(bb8 + redis + async)

```text
➜  src git:(master) ✗ wrk -t8 -c200 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.11ms   79.74us  10.74ms   96.37%
    Req/Sec     3.21k   661.66     5.94k    84.51%
  672473 requests in 30.10s, 77.60MB read
  Socket errors: connect 0, read 57, write 0, timeout 0
Requests/sec:  22340.66
Transfer/sec:      2.58MB
```

(Old Mac)


## perf(bb8 + redis + async)

```text
❯ wrk -t8 -c200 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.18ms    2.17ms  33.88ms   71.33%
    Req/Sec     4.86k   680.83     6.30k    64.17%
  1160658 requests in 30.02s, 133.93MB read
  Socket errors: connect 0, read 12, write 0, timeout 0
Requests/sec:  38665.86
Transfer/sec:      4.46MB
```

(m1x)

## perf(bb8 + redis + async)
with cargo build --release
```text
❯ wrk -t8 -c200 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.28ms    2.08ms  37.20ms   83.74%
    Req/Sec     8.21k     2.11k   12.54k    77.58%
  1960987 requests in 30.02s, 226.29MB read
Requests/sec:  65324.65
Transfer/sec:      7.54MB
```

(m1x)