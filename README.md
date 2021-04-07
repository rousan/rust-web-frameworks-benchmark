# Benchmarking Rust Web Frameworks: hyper vs gotham vs actix-web vs warp vs rocket

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

`Intel Core i7 CPU @ 3.20GHz × 12, 16GB of RAM`

Before run, I created a release build:

```sh
$ cargo build --release
```

### Framework 1: hyper (https://github.com/hyperium/hyper)

_Requests/sec:_ `520718.02` and _Latency:_ `368.31us`

```txt
Running 8s test @ http://127.0.0.1:8082
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   368.31us  327.11us   8.11ms   92.48%
    Req/Sec   131.79k     6.77k  147.18k    77.50%
  Latency Distribution
     50%  295.00us
     75%  390.00us
     90%  595.00us
     99%    1.64ms
  4196014 requests in 8.06s, 352.14MB read
Requests/sec: 520718.02
Transfer/sec:     43.70MB
```

### Framework 2: actix-web (https://github.com/actix/actix-web)

_Requests/sec:_ `562315.64` and _Latency:_ `365.86us`

```txt
Running 8s test @ http://127.0.0.1:8080
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   365.86us  659.57us  18.39ms   94.46%
    Req/Sec   142.44k     7.76k  150.25k    92.19%
  Latency Distribution
     50%  238.00us
     75%  328.00us
     90%  451.00us
     99%    3.35ms
  4534091 requests in 8.06s, 557.80MB read
Requests/sec: 562315.64
Transfer/sec:     69.18MB
```

### Framework 3: warp (https://github.com/seanmonstar/warp)

_Requests/sec:_ `506776.84` and _Latency:_ `393.21us`

```txt
Running 8s test @ http://127.0.0.1:3030
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   393.21us  487.15us  21.55ms   95.16%
    Req/Sec   128.68k     7.57k  146.42k    79.06%
  Latency Distribution
     50%  305.00us
     75%  410.00us
     90%  620.00us
     99%    1.80ms
  4098580 requests in 8.09s, 320.51MB read
  Non-2xx or 3xx responses: 4098580
Requests/sec: 506776.84
Transfer/sec:     39.63MB
```

### Framework 4: gotham (https://github.com/gotham-rs/gotham)

_Requests/sec:_ `300483.70` and _Latency:_ `677.14us`

```txt
Running 8s test @ http://127.0.0.1:8081
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   677.14us  513.16us  15.78ms   87.00%
    Req/Sec    76.36k     6.20k   88.57k    74.38%
  Latency Distribution
     50%  554.00us
     75%  816.00us
     90%    1.19ms
     99%    2.66ms
  2431445 requests in 8.09s, 384.92MB read
Requests/sec: 300483.70
Transfer/sec:     47.57MB
```

### Framework 5: Rocket (https://github.com/SergioBenitez/Rocket)

_Requests/sec:_ `115231.95` and _Latency:_ `412.75us`

```txt
Running 8s test @ http://127.0.0.1:8000
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   412.75us    1.35ms 204.10ms   99.83%
    Req/Sec    29.20k     1.60k   31.79k    85.94%
  Latency Distribution
     50%  312.00us
     75%  551.00us
     90%  737.00us
     99%    0.99ms
  929792 requests in 8.07s, 129.46MB read
  Socket errors: connect 0, read 929766, write 0, timeout 0
Requests/sec: 115231.95
Transfer/sec:     16.04MB
```

#### Caveat

In order to run a fair comparison a suitable environment variable should be set
while running, `ROCKET_ENV=prod ./rocket`.
Indeed, by default is set to `dev`, and it is spending a lot of time logging
everything on the console.

## Conclusion

`hyper` -> Perfect for an app which requires more controls on http and
performance. It provides `async` with `tokio.rs`.

`gotham` -> A high-level version of `hyper`.

`actix-web` -> Provides a very high-level API and injects some boilerplate code
as well as runtime. It provides `async` with `tokio.rs`.

`warp` -> A very high level version of `hyper` and easy to use and it requires
a few lines of codes to start a server. It provides `async` with `tokio.rs`.

`rocket` -> High level, actively developed and quite used. Nevertheless it has
not the same performances as the others.
