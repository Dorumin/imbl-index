A persistent (through immutability and structural sharing) IndexMap type for the [`imbl`](https://docs.rs/imbl) crate.

# Benchmarks
<details>
<summary>
Expand raw data
</summary>

| bench                                | time       |
| ------------------------------------ | ---------- |
| indexmap_i64/lookup_100              | 2.2859 µs |
| indexmap_i64/lookup_1000             | 36.547 µs |
| indexmap_i64/lookup_5000             | 357.18 µs |
| indexmap_i64/lookup_10000            | 836.44 µs |
| indexmap_i64/insert_100              | 41.216 µs |
| indexmap_i64/insert_1000             | 682.60 µs |
| indexmap_i64/insert_5000             | 5.0769 ms  |
| indexmap_i64/insert_10000            | 10.630 ms  |
| indexmap_i64/insert_mut_100          | 7.2860 µs |
| indexmap_i64/insert_mut_1000         | 104.05 µs |
| indexmap_i64/insert_mut_5000         | 579.78 µs |
| indexmap_i64/insert_mut_10000        | 1.1287 ms  |
| indexmap_i64/remove_100              | 42.718 µs |
| indexmap_i64/remove_mut_100          | 6.9188 µs |
| indexmap_i64/remove_1000             | 766.29 µs |
| indexmap_i64/remove_mut_1000         | 95.901 µs |
| indexmap_i64/remove_5000             | 5.3402 ms  |
| indexmap_i64/remove_mut_5000         | 685.83 µs |
| indexmap_i64/iter_100                | 334.56 ns  |
| indexmap_i64/iter_1000               | 3.0491 µs |
| indexmap_i64/iter_5000               | 15.207 µs |
| indexmap_i64/iter_10000              | 30.748 µs |
| hashmap_std_i64/lookup_100           | 1.1492 µs |
| hashmap_std_i64/lookup_1000          | 11.005 µs |
| hashmap_std_i64/lookup_5000          | 55.508 µs |
| hashmap_std_i64/lookup_10000         | 113.28 µs |
| hashmap_std_i64/insert_100           | 8.6501 µs |
| hashmap_std_i64/insert_1000          | 322.95 µs |
| hashmap_std_i64/insert_5000          | 8.0221 ms  |
| hashmap_std_i64/insert_10000         | 30.768 ms  |
| hashmap_std_i64/insert_mut_100       | 3.2221 µs |
| hashmap_std_i64/insert_mut_1000      | 38.602 µs |
| hashmap_std_i64/insert_mut_5000      | 168.87 µs |
| hashmap_std_i64/insert_mut_10000     | 340.78 µs |
| hashmap_std_i64/remove_100           | 7.2563 µs |
| hashmap_std_i64/remove_mut_100       | 1.3613 µs |
| hashmap_std_i64/remove_1000          | 822.38 µs |
| hashmap_std_i64/remove_mut_1000      | 14.264 µs |
| hashmap_std_i64/remove_5000          | 16.242 ms  |
| hashmap_std_i64/remove_mut_5000      | 75.090 µs |
| hashmap_std_i64/iter_100             | 68.763 ns  |
| hashmap_std_i64/iter_1000            | 701.16 ns  |
| hashmap_std_i64/iter_5000            | 3.6024 µs |
| hashmap_std_i64/iter_10000           | 8.5148 µs |
| hashmap_im_i64/lookup_100            | 1.2592 µs |
| hashmap_im_i64/lookup_1000           | 12.438 µs |
| hashmap_im_i64/lookup_5000           | 71.257 µs |
| hashmap_im_i64/lookup_10000          | 153.35 µs |
| hashmap_im_i64/insert_100            | 25.467 µs |
| hashmap_im_i64/insert_1000           | 393.87 µs |
| hashmap_im_i64/insert_5000           | 3.0017 ms  |
| hashmap_im_i64/insert_10000          | 6.0760 ms  |
| hashmap_im_i64/insert_mut_100        | 3.2600 µs |
| hashmap_im_i64/insert_mut_1000       | 45.440 µs |
| hashmap_im_i64/insert_mut_5000       | 190.78 µs |
| hashmap_im_i64/insert_mut_10000      | 334.88 µs |
| hashmap_im_i64/remove_100            | 22.786 µs |
| hashmap_im_i64/remove_mut_100        | 3.6274 µs |
| hashmap_im_i64/remove_1000           | 457.85 µs |
| hashmap_im_i64/remove_mut_1000       | 43.573 µs |
| hashmap_im_i64/remove_5000           | 2.9811 ms  |
| hashmap_im_i64/remove_mut_5000       | 238.81 µs |
| hashmap_im_i64/iter_100              | 396.75 ns  |
| hashmap_im_i64/iter_1000             | 4.2238 µs |
| hashmap_im_i64/iter_5000             | 24.190 µs |
| hashmap_im_i64/iter_10000            | 45.073 µs |
| ordmap_i64/lookup_100                | 1.0206 µs |
| ordmap_i64/lookup_1000               | 18.998 µs |
| ordmap_i64/lookup_5000               | 141.69 µs |
| ordmap_i64/lookup_10000              | 325.51 µs |
| ordmap_i64/insert_100                | 12.235 µs |
| ordmap_i64/insert_1000               | 240.06 µs |
| ordmap_i64/insert_5000               | 1.8052 ms  |
| ordmap_i64/insert_10000              | 4.0773 ms  |
| ordmap_i64/insert_mut_100            | 3.0041 µs |
| ordmap_i64/insert_mut_1000           | 45.082 µs |
| ordmap_i64/insert_mut_5000           | 324.23 µs |
| ordmap_i64/insert_mut_10000          | 711.57 µs |
| ordmap_i64/remove_100                | 13.394 µs |
| ordmap_i64/remove_mut_100            | 2.8501 µs |
| ordmap_i64/remove_1000               | 254.09 µs |
| ordmap_i64/remove_mut_1000           | 43.966 µs |
| ordmap_i64/remove_5000               | 1.8537 ms  |
| ordmap_i64/remove_mut_5000           | 342.77 µs |
| ordmap_i64/iter_100                  | 373.47 ns  |
| ordmap_i64/iter_1000                 | 3.5838 µs |
| ordmap_i64/iter_5000                 | 18.032 µs |
| ordmap_i64/iter_10000                | 35.987 µs |
| indexmap_str/lookup_100              | 3.0359 µs |
| indexmap_str/lookup_1000             | 55.400 µs |
| indexmap_str/lookup_5000             | 487.95 µs |
| indexmap_str/lookup_10000            | 1.1080 ms  |
| indexmap_str/insert_100              | 180.60 µs |
| indexmap_str/insert_1000             | 2.3769 ms  |
| indexmap_str/insert_5000             | 12.829 ms  |
| indexmap_str/insert_10000            | 26.443 ms  |
| indexmap_str/insert_mut_100          | 23.294 µs |
| indexmap_str/insert_mut_1000         | 281.55 µs |
| indexmap_str/insert_mut_5000         | 1.4826 ms  |
| indexmap_str/insert_mut_10000        | 3.0675 ms  |
| indexmap_str/remove_100              | 140.58 µs |
| indexmap_str/remove_mut_100          | 24.638 µs |
| indexmap_str/remove_1000             | 1.8897 ms  |
| indexmap_str/remove_mut_1000         | 305.70 µs |
| indexmap_str/remove_5000             | 11.333 ms  |
| indexmap_str/remove_mut_5000         | 1.9892 ms  |
| indexmap_str/iter_100                | 342.09 ns  |
| indexmap_str/iter_1000               | 3.1297 µs |
| indexmap_str/iter_5000               | 15.648 µs |
| indexmap_str/iter_10000              | 32.740 µs |
| hashmap_std_str/lookup_100           | 1.8542 µs |
| hashmap_std_str/lookup_1000          | 18.229 µs |
| hashmap_std_str/lookup_5000          | 143.61 µs |
| hashmap_std_str/lookup_10000         | 336.30 µs |
| hashmap_std_str/insert_100           | 328.68 µs |
| hashmap_std_str/insert_1000          | 32.880 ms  |
| hashmap_std_str/insert_5000          | 887.10 ms  |
| hashmap_std_str/insert_10000         | 3.3712 s   |
| hashmap_std_str/insert_mut_100       | 12.519 µs |
| hashmap_std_str/insert_mut_1000      | 139.29 µs |
| hashmap_std_str/insert_mut_5000      | 711.86 µs |
| hashmap_std_str/insert_mut_10000     | 1.5003 ms  |
| hashmap_std_str/remove_100           | 352.67 µs |
| hashmap_std_str/remove_mut_100       | 9.4234 µs |
| hashmap_std_str/remove_1000          | 35.129 ms  |
| hashmap_std_str/remove_mut_1000      | 103.60 µs |
| hashmap_std_str/remove_5000          | 867.79 ms  |
| hashmap_std_str/remove_mut_5000      | 583.83 µs |
| hashmap_std_str/iter_100             | 90.046 ns  |
| hashmap_std_str/iter_1000            | 908.39 ns  |
| hashmap_std_str/iter_5000            | 4.9313 µs |
| hashmap_std_str/iter_10000           | 10.367 µs |
| hashmap_im_str/lookup_100            | 2.1823 µs |
| hashmap_im_str/lookup_1000           | 29.504 µs |
| hashmap_im_str/lookup_5000           | 221.81 µs |
| hashmap_im_str/lookup_10000          | 485.48 µs |
| hashmap_im_str/insert_100            | 104.35 µs |
| hashmap_im_str/insert_1000           | 1.6700 ms  |
| hashmap_im_str/insert_5000           | 8.5247 ms  |
| hashmap_im_str/insert_10000          | 16.963 ms  |
| hashmap_im_str/insert_mut_100        | 11.557 µs |
| hashmap_im_str/insert_mut_1000       | 133.03 µs |
| hashmap_im_str/insert_mut_5000       | 681.90 µs |
| hashmap_im_str/insert_mut_10000      | 1.3481 ms  |
| hashmap_im_str/remove_100            | 100.92 µs |
| hashmap_im_str/remove_mut_100        | 12.754 µs |
| hashmap_im_str/remove_1000           | 1.5004 ms  |
| hashmap_im_str/remove_mut_1000       | 143.00 µs |
| hashmap_im_str/remove_5000           | 8.1225 ms  |
| hashmap_im_str/remove_mut_5000       | 941.81 µs |
| hashmap_im_str/iter_100              | 464.29 ns  |
| hashmap_im_str/iter_1000             | 5.4610 µs |
| hashmap_im_str/iter_5000             | 37.190 µs |
| hashmap_im_str/iter_10000            | 62.360 µs |
| ordmap_str/lookup_100                | 3.2563 µs |
| ordmap_str/lookup_1000               | 83.455 µs |
| ordmap_str/lookup_5000               | 671.36 µs |
| ordmap_str/lookup_10000              | 1.5743 ms  |
| ordmap_str/insert_100                | 116.32 µs |
| ordmap_str/insert_1000               | 1.8106 ms  |
| ordmap_str/insert_5000               | 11.086 ms  |
| ordmap_str/insert_10000              | 25.731 ms  |
| ordmap_str/insert_mut_100            | 13.420 µs |
| ordmap_str/insert_mut_1000           | 157.46 µs |
| ordmap_str/insert_mut_5000           | 902.25 µs |
| ordmap_str/insert_mut_10000          | 1.8449 ms  |
| ordmap_str/remove_100                | 98.170 µs |
| ordmap_str/remove_mut_100            | 17.620 µs |
| ordmap_str/remove_1000               | 1.4875 ms  |
| ordmap_str/remove_mut_1000           | 244.66 µs |
| ordmap_str/remove_5000               | 9.8947 ms  |
| ordmap_str/remove_mut_5000           | 1.5807 ms  |
| ordmap_str/iter_100                  | 340.90 ns  |
| ordmap_str/iter_1000                 | 3.0939 µs |
| ordmap_str/iter_5000                 | 15.413 µs |
| ordmap_str/iter_10000                | 30.974 µs |
| indexmap_big/lookup_100              | 79.381 µs |
| indexmap_big/lookup_1000             | 794.64 µs |
| indexmap_big/lookup_5000             | 4.4318 ms  |
| indexmap_big/lookup_10000            | 11.270 ms  |
| indexmap_big/insert_100              | 493.44 µs |
| indexmap_big/insert_1000             | 6.6227 ms  |
| indexmap_big/insert_5000             | 61.793 ms  |
| indexmap_big/insert_10000            | 287.37 ms  |
| indexmap_big/insert_mut_100          | 211.53 µs |
| indexmap_big/insert_mut_1000         | 3.2550 ms  |
| indexmap_big/insert_mut_5000         | 21.206 ms  |
| indexmap_big/insert_mut_10000        | 72.474 ms  |
| indexmap_big/remove_100              | 632.91 µs |
| indexmap_big/remove_mut_100          | 81.881 µs |
| indexmap_big/remove_1000             | 6.4059 ms  |
| indexmap_big/remove_mut_1000         | 715.10 µs |
| indexmap_big/remove_5000             | 98.376 ms  |
| indexmap_big/remove_mut_5000         | 8.1534 ms  |
| indexmap_big/iter_100                | 10.957 ns  |
| indexmap_big/iter_1000               | 10.956 ns  |
| indexmap_big/iter_5000               | 2.9919 µs |
| indexmap_big/iter_10000              | 20.736 µs |
| hashmap_std_big/lookup_100           | 78.787 µs |
| hashmap_std_big/lookup_1000          | 790.36 µs |
| hashmap_std_big/lookup_5000          | 4.1947 ms  |
| hashmap_std_big/lookup_10000         | 9.6619 ms  |
| hashmap_std_big/insert_100           | 190.73 µs |
| hashmap_std_big/insert_1000          | 3.0667 ms  |
| hashmap_std_big/insert_5000          | 7.6690 s   |
| hashmap_std_big/insert_10000         | 121.93 s   |
| hashmap_std_big/insert_mut_100       | 144.46 µs |
| hashmap_std_big/insert_mut_1000      | 2.7543 ms  |
| hashmap_std_big/insert_mut_5000      | 24.342 ms  |
| hashmap_std_big/insert_mut_10000     | 79.661 ms  |
| hashmap_std_big/remove_100           | 901.86 µs |
| hashmap_std_big/remove_mut_100       | 94.940 µs |
| hashmap_std_big/remove_1000          | 21.695 ms  |
| hashmap_std_big/remove_mut_1000      | 838.63 µs |
| hashmap_std_big/remove_5000          | 11.103 s   |
| hashmap_std_big/remove_mut_5000      | 8.0986 ms  |
| hashmap_std_big/iter_100             | 2.5032 ns  |
| hashmap_std_big/iter_1000            | 35.444 ns  |
| hashmap_std_big/iter_5000            | 787.17 ns  |
| hashmap_std_big/iter_10000           | 4.1548 µs |
| hashmap_im_big/lookup_100            | 77.897 µs |
| hashmap_im_big/lookup_1000           | 783.58 µs |
| hashmap_im_big/lookup_5000           | 4.2245 ms  |
| hashmap_im_big/lookup_10000          | 10.085 ms  |
| hashmap_im_big/insert_100            | 7.2831 ms  |
| hashmap_im_big/insert_1000           | 75.494 ms  |
| hashmap_im_big/insert_5000           | 440.50 ms  |
| hashmap_im_big/insert_10000          | 1.0080 s   |
| hashmap_im_big/insert_mut_100        | 200.34 µs |
| hashmap_im_big/insert_mut_1000       | 3.2916 ms  |
| hashmap_im_big/insert_mut_5000       | 18.185 ms  |
| hashmap_im_big/insert_mut_10000      | 44.000 ms  |
| hashmap_im_big/remove_100            | 1.2253 ms  |
| hashmap_im_big/remove_mut_100        | 82.923 µs |
| hashmap_im_big/remove_1000           | 12.548 ms  |
| hashmap_im_big/remove_mut_1000       | 706.14 µs |
| hashmap_im_big/remove_5000           | 175.05 ms  |
| hashmap_im_big/remove_mut_5000       | 6.8499 ms  |
| hashmap_im_big/iter_100              | 6.1566 ns  |
| hashmap_im_big/iter_1000             | 6.1257 ns  |
| hashmap_im_big/iter_5000             | 5.8804 µs |
| hashmap_im_big/iter_10000            | 66.031 µs |
| ordmap_big/lookup_100                | 10.445 µs |
| ordmap_big/lookup_1000               | 119.44 µs |
| ordmap_big/lookup_5000               | 2.5948 ms  |
| ordmap_big/lookup_10000              | 14.549 ms  |
| ordmap_big/insert_100                | 641.66 µs |
| ordmap_big/insert_1000               | 8.8965 ms  |
| ordmap_big/insert_5000               | 83.144 ms  |
| ordmap_big/insert_10000              | 278.67 ms  |
| ordmap_big/insert_mut_100            | 133.14 µs |
| ordmap_big/insert_mut_1000           | 2.3931 ms  |
| ordmap_big/insert_mut_5000           | 17.414 ms  |
| ordmap_big/insert_mut_10000          | 62.351 ms  |
| ordmap_big/remove_100                | 278.45 µs |
| ordmap_big/remove_mut_100            | 13.251 µs |
| ordmap_big/remove_1000               | 3.3438 ms  |
| ordmap_big/remove_mut_1000           | 100.05 µs |
| ordmap_big/remove_5000               | 72.237 ms  |
| ordmap_big/remove_mut_5000           | 4.8885 ms  |
| ordmap_big/iter_100                  | 11.745 ns  |
| ordmap_big/iter_1000                 | 11.780 ns  |
| ordmap_big/iter_5000                 | 3.2942 µs |
| ordmap_big/iter_10000                | 21.503 µs |
| indexmap_specific_i64/get_index_100  | 18.658 µs |
| indexmap_specific_i64/front_100      | 1.0291 ns  |
| indexmap_specific_i64/back_100       | 1.5037 ns  |
| indexmap_specific_i64/get_index_1000 | 1.5393 ms  |
| indexmap_specific_i64/front_1000     | 1.6051 ns  |
| indexmap_specific_i64/back_1000      | 2.5228 ns  |
| indexmap_specific_i64/get_index_5000 | 38.381 ms  |
| indexmap_specific_i64/front_5000     | 2.4550 ns  |
| indexmap_specific_i64/back_5000      | 3.7862 ns  |

Scripts used to generate summaries: (nushell)
```
let bench_times = open bench.stderr | collect | parse -r '(?m)^(.+?)\s+time:\s+\[\S+ \S+ ([\d\.]+ \S+)' | rename bench time;

$bench_times | to md --pretty
```

```
$bench_times
| insert timeval { |row| $row.time | str replace " s" " sec" | str replace -r '\s' '' | into duration }
| each { |row| $row | merge ($row.bench | parse -r '(?<struct>.+)_(?<cat>(?<dtype>...)/(?<test>.+?)_(?<batch>\d+))' | first) }
| group-by test
| values
| where { let len = uniq-by struct | length; $len > 1 }
| sort-by { |rows| $rows.0.cat }
| each { |rows|
    let test = $rows.0.test

    let displayrows = $rows | insert grouper { |r| $"($r.batch)-($r.dtype)" }
    | group-by grouper
    | update cells { |rows|
        let fastest = $rows.timeval | math min;
        let batch = $rows.0.batch;
        let dtype = $rows.0.dtype;
        $rows | update time { |row| $"($row.time) \(($row.timeval / $fastest | math round -p 1)x\)" }
        | select struct time
        | transpose --header-row
        | insert batch $batch
        | insert dtype $dtype
    }
    | values
    | flatten
    | select dtype batch hashmap_std hashmap_im ordmap indexmap

    $"## ($test)\n($displayrows | to md --pretty)"
}
| str join "\n\n"

# For specific
$bench_times
| insert timeval { |row| $row.time | str replace " s" " sec" | str replace -r '\s' '' | into duration }
| each { |row| $row | merge ($row.bench | parse -r '(?<struct>.+)_(?<cat>(?<dtype>...)/(?<test>.+?)_(?<batch>\d+))' | first) }
| group-by test
| values
| where { let len = uniq-by struct | length; $len == 1 }
| sort-by { |rows| $rows.0.cat }
| each { |rows|
    let test = $rows.0.test

    let displayrows = $rows | insert grouper { |r| $"($r.batch)-($r.dtype)" } | group-by grouper | update cells { |rows| let fastest = $rows.timeval | math min; let batch = $rows.0.batch; let dtype = $rows.0.dtype; $rows | update time { |row| $"($row.time) \(($row.timeval / $fastest | math round -p 1)x\)" } | select struct time | transpose --header-row | insert batch $batch | insert dtype $dtype } | values | flatten | select batch dtype indexmap_specific | rename batch dtype indexmap

    $"### ($test)\n($displayrows | to md --pretty)"
}
| str join "\n\n
```
</details>

## insert
| dtype | batch | hashmap_std        | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ------------------ | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 8.6501 µs (1.0x)  | 25.467 µs (2.9x) | 12.235 µs (1.4x) | 41.216 µs (4.8x) |
| i64   | 1000  | 322.95 µs (1.3x)  | 393.87 µs (1.6x) | 240.06 µs (1.0x) | 682.60 µs (2.8x) |
| i64   | 5000  | 8.0221 ms (4.4x)   | 3.0017 ms (1.7x)  | 1.8052 ms (1.0x)  | 5.0769 ms (2.8x)  |
| i64   | 10000 | 30.768 ms (7.5x)   | 6.0760 ms (1.5x)  | 4.0773 ms (1.0x)  | 10.630 ms (2.6x)  |
| str   | 100   | 328.68 µs (3.1x)  | 104.35 µs (1.0x) | 116.32 µs (1.1x) | 180.60 µs (1.7x) |
| str   | 1000  | 32.880 ms (19.7x)  | 1.6700 ms (1.0x)  | 1.8106 ms (1.1x)  | 2.3769 ms (1.4x)  |
| str   | 5000  | 887.10 ms (104.1x) | 8.5247 ms (1.0x)  | 11.086 ms (1.3x)  | 12.829 ms (1.5x)  |
| str   | 10000 | 3.3712 s (198.7x)  | 16.963 ms (1.0x)  | 25.731 ms (1.5x)  | 26.443 ms (1.6x)  |
| big   | 100   | 190.73 µs (1.0x)  | 7.2831 ms (38.2x) | 641.66 µs (3.4x) | 493.44 µs (2.6x) |
| big   | 1000  | 3.0667 ms (1.0x)   | 75.494 ms (24.6x) | 8.8965 ms (2.9x)  | 6.6227 ms (2.2x)  |
| big   | 5000  | 7.6690 s (124.1x)  | 440.50 ms (7.1x)  | 83.144 ms (1.3x)  | 61.793 ms (1.0x)  |
| big   | 10000 | 121.93 s (437.5x)  | 1.0080 s (3.6x)   | 278.67 ms (1.0x)  | 287.37 ms (1.0x)  |

## insert_mut
| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 3.2221 µs (1.1x) | 3.2600 µs (1.1x) | 3.0041 µs (1.0x) | 7.2860 µs (2.4x) |
| i64   | 1000  | 38.602 µs (1.0x) | 45.440 µs (1.2x) | 45.082 µs (1.2x) | 104.05 µs (2.7x) |
| i64   | 5000  | 168.87 µs (1.0x) | 190.78 µs (1.1x) | 324.23 µs (1.9x) | 579.78 µs (3.4x) |
| i64   | 10000 | 340.78 µs (1.0x) | 334.88 µs (1.0x) | 711.57 µs (2.1x) | 1.1287 ms (3.4x)  |
| str   | 100   | 12.519 µs (1.1x) | 11.557 µs (1.0x) | 13.420 µs (1.2x) | 23.294 µs (2.0x) |
| str   | 1000  | 139.29 µs (1.0x) | 133.03 µs (1.0x) | 157.46 µs (1.2x) | 281.55 µs (2.1x) |
| str   | 5000  | 711.86 µs (1.0x) | 681.90 µs (1.0x) | 902.25 µs (1.3x) | 1.4826 ms (2.2x)  |
| str   | 10000 | 1.5003 ms (1.1x)  | 1.3481 ms (1.0x)  | 1.8449 ms (1.4x)  | 3.0675 ms (2.3x)  |
| big   | 100   | 144.46 µs (1.1x) | 200.34 µs (1.5x) | 133.14 µs (1.0x) | 211.53 µs (1.6x) |
| big   | 1000  | 2.7543 ms (1.2x)  | 3.2916 ms (1.4x)  | 2.3931 ms (1.0x)  | 3.2550 ms (1.4x)  |
| big   | 5000  | 24.342 ms (1.4x)  | 18.185 ms (1.0x)  | 17.414 ms (1.0x)  | 21.206 ms (1.2x)  |
| big   | 10000 | 79.661 ms (1.8x)  | 44.000 ms (1.0x)  | 62.351 ms (1.4x)  | 72.474 ms (1.6x)  |

## iter
| dtype | batch | hashmap_std       | hashmap_im         | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ------------------ | ----------------- | ----------------- |
| i64   | 100   | 68.763 ns (1.0x)  | 396.75 ns (5.8x)   | 373.47 ns (5.5x)  | 334.56 ns (4.9x)  |
| i64   | 1000  | 701.16 ns (1.0x)  | 4.2238 µs (6.0x)  | 3.5838 µs (5.1x) | 3.0491 µs (4.3x) |
| i64   | 5000  | 3.6024 µs (1.0x) | 24.190 µs (6.7x)  | 18.032 µs (5.0x) | 15.207 µs (4.2x) |
| i64   | 10000 | 8.5148 µs (1.0x) | 45.073 µs (5.3x)  | 35.987 µs (4.2x) | 30.748 µs (3.6x) |
| str   | 100   | 90.046 ns (1.0x)  | 464.29 ns (5.2x)   | 340.90 ns (3.8x)  | 342.09 ns (3.8x)  |
| str   | 1000  | 908.39 ns (1.0x)  | 5.4610 µs (6.0x)  | 3.0939 µs (3.4x) | 3.1297 µs (3.4x) |
| str   | 5000  | 4.9313 µs (1.0x) | 37.190 µs (7.5x)  | 15.413 µs (3.1x) | 15.648 µs (3.2x) |
| str   | 10000 | 10.367 µs (1.0x) | 62.360 µs (6.0x)  | 30.974 µs (3.0x) | 32.740 µs (3.2x) |
| big   | 100   | 2.5032 ns (1.0x)  | 6.1566 ns (3.0x)   | 11.745 ns (5.5x)  | 10.957 ns (5.0x)  |
| big   | 1000  | 35.444 ns (5.8x)  | 6.1257 ns (1.0x)   | 11.780 ns (1.8x)  | 10.956 ns (1.7x)  |
| big   | 5000  | 787.17 ns (1.0x)  | 5.8804 µs (7.5x)  | 3.2942 µs (4.2x) | 2.9919 µs (3.8x) |
| big   | 10000 | 4.1548 µs (1.0x) | 66.031 µs (15.9x) | 21.503 µs (5.2x) | 20.736 µs (5.0x) |

## lookup
| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 1.1492 µs (1.1x) | 1.2592 µs (1.2x) | 1.0206 µs (1.0x) | 2.2859 µs (2.2x) |
| i64   | 1000  | 11.005 µs (1.0x) | 12.438 µs (1.1x) | 18.998 µs (1.7x) | 36.547 µs (3.3x) |
| i64   | 5000  | 55.508 µs (1.0x) | 71.257 µs (1.3x) | 141.69 µs (2.6x) | 357.18 µs (6.4x) |
| i64   | 10000 | 113.28 µs (1.0x) | 153.35 µs (1.4x) | 325.51 µs (2.9x) | 836.44 µs (7.4x) |
| str   | 100   | 1.8542 µs (1.0x) | 2.1823 µs (1.2x) | 3.2563 µs (1.8x) | 3.0359 µs (1.6x) |
| str   | 1000  | 18.229 µs (1.0x) | 29.504 µs (1.6x) | 83.455 µs (4.6x) | 55.400 µs (3.0x) |
| str   | 5000  | 143.61 µs (1.0x) | 221.81 µs (1.5x) | 671.36 µs (4.7x) | 487.95 µs (3.4x) |
| str   | 10000 | 336.30 µs (1.0x) | 485.48 µs (1.4x) | 1.5743 ms (4.7x)  | 1.1080 ms (3.3x)  |
| big   | 100   | 78.787 µs (7.5x) | 77.897 µs (7.5x) | 10.445 µs (1.0x) | 79.381 µs (7.6x) |
| big   | 1000  | 790.36 µs (6.6x) | 783.58 µs (6.6x) | 119.44 µs (1.0x) | 794.64 µs (6.7x) |
| big   | 5000  | 4.1947 ms (1.6x)  | 4.2245 ms (1.6x)  | 2.5948 ms (1.0x)  | 4.4318 ms (1.7x)  |
| big   | 10000 | 9.6619 ms (1.0x)  | 10.085 ms (1.0x)  | 14.549 ms (1.5x)  | 11.270 ms (1.2x)  |

## remove
| dtype | batch | hashmap_std        | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ------------------ | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 7.2563 µs (1.0x)  | 22.786 µs (3.1x) | 13.394 µs (1.8x) | 42.718 µs (5.9x) |
| i64   | 1000  | 822.38 µs (3.2x)  | 457.85 µs (1.8x) | 254.09 µs (1.0x) | 766.29 µs (3.0x) |
| i64   | 5000  | 16.242 ms (8.8x)   | 2.9811 ms (1.6x)  | 1.8537 ms (1.0x)  | 5.3402 ms (2.9x)  |
| str   | 100   | 352.67 µs (3.6x)  | 100.92 µs (1.0x) | 98.170 µs (1.0x) | 140.58 µs (1.4x) |
| str   | 1000  | 35.129 ms (23.6x)  | 1.5004 ms (1.0x)  | 1.4875 ms (1.0x)  | 1.8897 ms (1.3x)  |
| str   | 5000  | 867.79 ms (106.8x) | 8.1225 ms (1.0x)  | 9.8947 ms (1.2x)  | 11.333 ms (1.4x)  |
| big   | 100   | 901.86 µs (3.2x)  | 1.2253 ms (4.4x)  | 278.45 µs (1.0x) | 632.91 µs (2.3x) |
| big   | 1000  | 21.695 ms (6.5x)   | 12.548 ms (3.8x)  | 3.3438 ms (1.0x)  | 6.4059 ms (1.9x)  |
| big   | 5000  | 11.103 s (153.7x)  | 175.05 ms (2.4x)  | 72.237 ms (1.0x)  | 98.376 ms (1.4x)  |

## remove_mut
| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 1.3613 µs (1.0x) | 3.6274 µs (2.7x) | 2.8501 µs (2.1x) | 6.9188 µs (5.1x) |
| i64   | 1000  | 14.264 µs (1.0x) | 43.573 µs (3.1x) | 43.966 µs (3.1x) | 95.901 µs (6.7x) |
| i64   | 5000  | 75.090 µs (1.0x) | 238.81 µs (3.2x) | 342.77 µs (4.6x) | 685.83 µs (9.1x) |
| str   | 100   | 9.4234 µs (1.0x) | 12.754 µs (1.4x) | 17.620 µs (1.9x) | 24.638 µs (2.6x) |
| str   | 1000  | 103.60 µs (1.0x) | 143.00 µs (1.4x) | 244.66 µs (2.4x) | 305.70 µs (3.0x) |
| str   | 5000  | 583.83 µs (1.0x) | 941.81 µs (1.6x) | 1.5807 ms (2.7x)  | 1.9892 ms (3.4x)  |
| big   | 100   | 94.940 µs (7.2x) | 82.923 µs (6.3x) | 13.251 µs (1.0x) | 81.881 µs (6.2x) |
| big   | 1000  | 838.63 µs (8.4x) | 706.14 µs (7.1x) | 100.05 µs (1.0x) | 715.10 µs (7.1x) |
| big   | 5000  | 8.0986 ms (1.7x)  | 6.8499 ms (1.4x)  | 4.8885 ms (1.0x)  | 8.1534 ms (1.7x)  |

## IndexMap-specific
### back
| batch   | dtype | indexmap         |
| ------- | ----- | ---------------- |
| 100     | i64   | 1.6001 ns (1.0x) |
| 1000    | i64   | 2.5607 ns (1.0x) |
| 5000    | i64   | 3.8135 ns (1.0x) |
| 10000   | i64   | 3.9608 ns (1.0x) |
| 50000   | i64   | 5.0282 ns (1.0x) |
| 100000  | i64   | 5.0901 ns (1.0x) |
| 500000  | i64   | 6.5425 ns (1.0x) |
| 1000000 | i64   | 8.0057 ns (1.0x) |

### front
| batch   | dtype | indexmap         |
| ------- | ----- | ---------------- |
| 100     | i64   | 1.0701 ns (1.0x) |
| 1000    | i64   | 1.7641 ns (1.0x) |
| 5000    | i64   | 2.3726 ns (1.0x) |
| 10000   | i64   | 2.5087 ns (1.0x) |
| 50000   | i64   | 3.2361 ns (1.0x) |
| 100000  | i64   | 3.3696 ns (1.0x) |
| 500000  | i64   | 4.2048 ns (1.0x) |
| 1000000 | i64   | 5.1439 ns (1.0x) |

### get_index
| batch   | dtype | indexmap          |
| ------- | ----- | ----------------- |
| 100     | i64   | 198.14 ns (1.0x)  |
| 1000    | i64   | 1.6010 µs (1.0x) |
| 5000    | i64   | 7.6336 µs (1.0x) |
| 10000   | i64   | 16.420 µs (1.0x) |
| 50000   | i64   | 87.050 µs (1.0x) |
| 100000  | i64   | 188.99 µs (1.0x) |
| 500000  | i64   | 1.1504 ms (1.0x)  |
| 1000000 | i64   | 2.9002 ms (1.0x)  |

## Takeaways
- Immutable IndexMap offers worst-in-class performance across the immutable structures, but not by an outrageous amount. It only really beats out the standard library's HashMap on clone ops (remove, insert)
- It also performs poorly (O(n)) on get_index due to the OrdMap backing; it's implemented as .iter().nth()
- The exact numbers above should be taken with a grain of salt due to the low sample count
- Should I have included the `indexmap` crate in the benchmarks? Probably
- Was it worth it to include the `big` datatype (a 4kb stack-allocated array) in the benchmarks? Not really, all it did was make the 10k insert stdlib benchmark tale 20min to complete
