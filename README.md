# Rust playground

String similarity

Rust program was compiled with
```
export RUSTFLAGS="-C target-cpu=native"
cargo build --release
```

# Benchmarks 


```
$ hyperfine -s basic -L n 10,100,1000,10000,100000,1000000 'cargo run --release data/names-1.txt data/names-{n}.txt'
$ hyperfine -s basic -L n 10,100,1000,10000,100000,1000000 'python3 find_closest_names.py data/names-1.txt data/names-{n}.txt'
```

| n        | python    | rust     |
| -------- | --------- | -------- |
| 1        |   31.9 ms |  30.2 ms |
| 10       |   31.6 ms |  31.6 ms |
| 100      |   32.5 ms |  32.4 ms |
| 1000     |   39.3 ms |  38.4 ms |
| 10000    |  118.0 ms |  46.6 ms |
| 100000   |  910.8 ms | 136.8 ms |
             
```
$ hyperfine -s basic -L n 10,100,1000,10000 'cargo run --release data/names-1000.txt data/names-{n}.txt'
$ hyperfine -s basic -L n 10,100,1000,10000 'python3 par_find_closest_names.py data/names-1000.txt data/names-{n}.txt'
```

| n      | python    | rust      |
| ------ | --------- | --------- |
| 10     |  200.4 ms |   34.3 ms |
| 100    |  196.7 ms |   36.5 ms |
| 1000   |  198.5 ms |   82.0 ms |
| 10000  |  980.8 ms |  572.8 ms |
| 100000 |  9.517 s  |   6.177 s |
