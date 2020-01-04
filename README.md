# Rust playground

String similarity

# Benchmarks 


```
$ hyperfine -s basic -L n 10,100,1000,10000,100000,1000000 'cargo run data/names-1.txt data/names-{n}.txt'
$ hyperfine -s basic -L n 10,100,1000,10000,100000,1000000 'python3 find_closest_names.py data/names-1.txt data/names-{n}.txt'
```

| n        | python    | rust     |
| -------- | --------- | -------- |
| 1        |   31.9 ms |   29.1 ms|
| 10       |   31.6 ms |   30.1 ms|
| 100      |   32.5 ms |   31.9 ms|
| 1000     |   39.3 ms |   50.9 ms|
| 10000    |  118.0 ms |  236.8 ms|
| 100000   |  910.8 ms |   2.326 s|
             
```
$ hyperfine -s basic -L n 10,100,1000,10000 'cargo run data/names-1000.txt data/names-{n}.txt'
$ hyperfine -s basic -L n 10,100,1000,10000 'python3 find_closest_names.py data/names-1000.txt data/names-{n}.txt'
```

| n    | python    | rust     |
| ---- | --------- | -------- |
|10    |   38.9 ms |   51.0 ms|
|100   |   83.1 ms |  209.9 ms|
|1000  |  504.5 ms |   2.060 s|
|10000 |   4.659 s |  22.065 s|
