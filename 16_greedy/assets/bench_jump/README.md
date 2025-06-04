1. Open ``bench_jump`` folder in a terminal
1. Just type ``cargo bench``
1. In Cargo.toml don't forget the ``[[bench]]`` section

```toml
[[bench]]
name = "jump"
harness = false
```

Here is what I can see today

```
     Running benches\jump.rs (target\release\deps\jump-847fd10e21a6f00d.exe)
Gnuplot not found, using plotters backend
for loop                time:   [2.0850 µs 2.0910 µs 2.0980 µs]
Found 13 outliers among 100 measurements (13.00%)
  9 (9.00%) high mild
  4 (4.00%) high severe

fold with range         time:   [2.0878 µs 2.0952 µs 2.1047 µs]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

fold with iter enumerate
                        time:   [4.5923 µs 4.6171 µs 4.6406 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

```
