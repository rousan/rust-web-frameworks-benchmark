# Benchmarking Rust Web Frameworks: hyper vs gotham vs actix-web vs rocket

I was building a very performance sensitive API service using Rust and I was trying to find out which http framework I should use
as there a lot of frameworks out there for Rust. So, I just shortlisted some of them and I ran a `hello world route` for every frameworks
and its result is very interesting.

## Benchmarking Tool

I used `wrk` to test those http servers:

```sh
$ wrk --latency -t4 -c200 -d8s http://127.0.0.1:8081
```

## Results

I ran all those tests on:

`MacBook Pro, 2.2 GHz Intel Core i7 processor with 4 cores, 16GB of RAM`

Before run, I created a release build:

```sh
$ cargo build --release
```

### Framework 1: hyper (https://github.com/hyperium/hyper)

*Requests/sec:* `112557.51` and *Latency:* `1.77ms`

```
Running 8s test @ http://127.0.0.1:8082
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.77ms  186.06us   4.40ms   83.89%
    Req/Sec    28.29k     1.51k   34.67k    83.44%
  Latency Distribution
     50%    1.74ms
     75%    1.81ms
     90%    1.94ms
     99%    2.41ms
  900610 requests in 8.00s, 75.58MB read
  Socket errors: connect 0, read 31, write 0, timeout 0
Requests/sec: 112557.51
Transfer/sec:      9.45MB
```

Requests/sec: `112557.51`
Latency: `1.77ms`


### Framework 2: gotham (https://github.com/gotham-rs/gotham)

```txt
Running 8s test @ http://127.0.0.1:8081
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.98ms  160.96us   5.11ms   84.14%
    Req/Sec    25.16k   566.48    26.28k    73.77%
  Latency Distribution
     50%    1.96ms
     75%    2.03ms
     90%    2.13ms
     99%    2.56ms
  810962 requests in 8.10s, 128.38MB read
  Socket errors: connect 0, read 23, write 0, timeout 0
Requests/sec: 100097.73
Transfer/sec:     15.85MB
```

Requests/sec: `100097.73`

Latency: `1.98ms`


### Framework 3: actix-web (https://github.com/actix/actix-web)

```txt
Running 8s test @ http://127.0.0.1:8080
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.06ms  198.35us   4.64ms   83.54%
    Req/Sec    24.23k     1.38k   29.09k    79.06%
  Latency Distribution
     50%    2.06ms
     75%    2.12ms
     90%    2.21ms
     99%    2.81ms
  771307 requests in 8.00s, 94.89MB read
  Socket errors: connect 0, read 38, write 0, timeout 0
Requests/sec:  96397.31
Transfer/sec:     11.86MB
```

Requests/sec: `96397.31`

Latency: `2.06ms`


### Framework 4: Rocket (https://github.com/SergioBenitez/Rocket)
```txt
Running 8s test @ http://127.0.0.1:8000
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.37ms  428.85us   5.77ms   80.13%
    Req/Sec     4.61k     1.22k    5.51k    88.89%
  Latency Distribution
     50%    3.32ms
     75%    3.57ms
     90%    3.89ms
     99%    4.67ms
  16519 requests in 8.09s, 2.30MB read
  Socket errors: connect 0, read 16613, write 0, timeout 0
Requests/sec:   2041.93
Transfer/sec:    291.14KB
```

Requests/sec: `2041.93` 😱

Latency: `3.37ms`


## Conclusion

`hyper`   -> Perfect for an app which requires more controls on http. It provides `async` with `tokio.rs`.

`gotham`  -> A high-level version of `hyper`.

`actix`   -> Provides very high-level API and injects some boilerplate code as well as runtime. It provides `async` with `tokio.rs`.

`rocket`  -> The performance it gave I am not sure about this at all. No async.
