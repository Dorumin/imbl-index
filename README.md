A persistent (through immutability and structural sharing) IndexMap type for the [`imbl`](https://docs.rs/imbl) crate.

# Benchmarks
<details>
<summary>
Expand raw data
</summary>

| bench                                   | time       |
| --------------------------------------- | ---------- |
| indexmap_i64/lookup_100                 | 2.2112 µs |
| indexmap_i64/lookup_1000                | 35.510 µs |
| indexmap_i64/lookup_5000                | 352.27 µs |
| indexmap_i64/lookup_10000               | 843.33 µs |
| indexmap_i64/lookup_one_100             | 23.071 ns  |
| indexmap_i64/lookup_one_1000            | 33.585 ns  |
| indexmap_i64/lookup_one_5000            | 41.951 ns  |
| indexmap_i64/lookup_one_10000           | 44.634 ns  |
| indexmap_i64/lookup_one_50000           | 52.835 ns  |
| indexmap_i64/lookup_one_100000          | 54.579 ns  |
| indexmap_i64/insert_100                 | 44.440 µs |
| indexmap_i64/insert_1000                | 739.50 µs |
| indexmap_i64/insert_5000                | 5.2003 ms  |
| indexmap_i64/insert_10000               | 10.826 ms  |
| indexmap_i64/insert_one_100             | 927.15 ns  |
| indexmap_i64/insert_one_1000            | 1.5553 µs |
| indexmap_i64/insert_one_5000            | 1.8646 µs |
| indexmap_i64/insert_one_10000           | 1.9254 µs |
| indexmap_i64/insert_mut_100             | 7.6923 µs |
| indexmap_i64/insert_mut_1000            | 109.36 µs |
| indexmap_i64/insert_mut_5000            | 559.90 µs |
| indexmap_i64/insert_mut_10000           | 1.1089 ms  |
| indexmap_i64/reinsert_mut_one_100       | 143.85 ns  |
| indexmap_i64/reinsert_mut_one_1000      | 213.81 ns  |
| indexmap_i64/reinsert_mut_one_5000      | 235.55 ns  |
| indexmap_i64/reinsert_mut_one_10000     | 252.18 ns  |
| indexmap_i64/reinsert_mut_one_50000     | 392.75 ns  |
| indexmap_i64/reinsert_mut_one_100000    | 459.31 ns  |
| indexmap_i64/remove_100                 | 39.084 µs |
| indexmap_i64/remove_mut_100             | 6.5872 µs |
| indexmap_i64/remove_1000                | 786.99 µs |
| indexmap_i64/remove_mut_1000            | 95.437 µs |
| indexmap_i64/remove_5000                | 5.3150 ms  |
| indexmap_i64/remove_mut_5000            | 692.17 µs |
| indexmap_i64/remove_10000               | 11.486 ms  |
| indexmap_i64/remove_mut_10000           | 1.4346 ms  |
| indexmap_i64/iter_100                   | 365.04 ns  |
| indexmap_i64/iter_1000                  | 3.5792 µs |
| indexmap_i64/iter_5000                  | 16.670 µs |
| indexmap_i64/iter_10000                 | 33.166 µs |
| hashmap_std_i64/lookup_100              | 1.1313 µs |
| hashmap_std_i64/lookup_1000             | 11.145 µs |
| hashmap_std_i64/lookup_one_100          | 11.213 ns  |
| hashmap_std_i64/lookup_one_1000         | 11.154 ns  |
| hashmap_std_i64/lookup_one_5000         | 11.152 ns  |
| hashmap_std_i64/lookup_one_10000        | 11.140 ns  |
| hashmap_std_i64/lookup_one_50000        | 11.161 ns  |
| hashmap_std_i64/lookup_one_100000       | 11.214 ns  |
| hashmap_std_i64/insert_100              | 8.0567 µs |
| hashmap_std_i64/insert_1000             | 311.13 µs |
| hashmap_std_i64/insert_one_100          | 233.15 ns  |
| hashmap_std_i64/insert_one_1000         | 2.5858 µs |
| hashmap_std_i64/insert_mut_100          | 3.2507 µs |
| hashmap_std_i64/insert_mut_1000         | 39.664 µs |
| hashmap_std_i64/reinsert_mut_one_100    | 38.032 ns  |
| hashmap_std_i64/reinsert_mut_one_1000   | 32.707 ns  |
| hashmap_std_i64/reinsert_mut_one_5000   | 36.044 ns  |
| hashmap_std_i64/reinsert_mut_one_10000  | 38.546 ns  |
| hashmap_std_i64/reinsert_mut_one_50000  | 53.511 ns  |
| hashmap_std_i64/reinsert_mut_one_100000 | 61.325 ns  |
| hashmap_std_i64/remove_100              | 7.5556 µs |
| hashmap_std_i64/remove_mut_100          | 1.2943 µs |
| hashmap_std_i64/remove_1000             | 952.30 µs |
| hashmap_std_i64/remove_mut_1000         | 13.460 µs |
| hashmap_std_i64/iter_100                | 72.652 ns  |
| hashmap_std_i64/iter_1000               | 713.17 ns  |
| hashmap_im_i64/lookup_100               | 1.2196 µs |
| hashmap_im_i64/lookup_1000              | 12.789 µs |
| hashmap_im_i64/lookup_5000              | 72.025 µs |
| hashmap_im_i64/lookup_10000             | 158.11 µs |
| hashmap_im_i64/lookup_one_100           | 12.904 ns  |
| hashmap_im_i64/lookup_one_1000          | 13.386 ns  |
| hashmap_im_i64/lookup_one_5000          | 13.632 ns  |
| hashmap_im_i64/lookup_one_10000         | 13.768 ns  |
| hashmap_im_i64/lookup_one_50000         | 13.874 ns  |
| hashmap_im_i64/lookup_one_100000        | 14.326 ns  |
| hashmap_im_i64/insert_100               | 25.141 µs |
| hashmap_im_i64/insert_1000              | 380.23 µs |
| hashmap_im_i64/insert_5000              | 2.9101 ms  |
| hashmap_im_i64/insert_10000             | 6.0241 ms  |
| hashmap_im_i64/insert_one_100           | 525.48 ns  |
| hashmap_im_i64/insert_one_1000          | 915.39 ns  |
| hashmap_im_i64/insert_one_5000          | 998.19 ns  |
| hashmap_im_i64/insert_one_10000         | 992.82 ns  |
| hashmap_im_i64/insert_mut_100           | 3.4218 µs |
| hashmap_im_i64/insert_mut_1000          | 46.224 µs |
| hashmap_im_i64/insert_mut_5000          | 193.98 µs |
| hashmap_im_i64/insert_mut_10000         | 342.06 µs |
| hashmap_im_i64/reinsert_mut_one_100     | 49.202 ns  |
| hashmap_im_i64/reinsert_mut_one_1000    | 73.991 ns  |
| hashmap_im_i64/reinsert_mut_one_5000    | 72.559 ns  |
| hashmap_im_i64/reinsert_mut_one_10000   | 65.405 ns  |
| hashmap_im_i64/reinsert_mut_one_50000   | 125.47 ns  |
| hashmap_im_i64/reinsert_mut_one_100000  | 127.28 ns  |
| hashmap_im_i64/remove_100               | 23.082 µs |
| hashmap_im_i64/remove_mut_100           | 3.7433 µs |
| hashmap_im_i64/remove_1000              | 457.65 µs |
| hashmap_im_i64/remove_mut_1000          | 44.098 µs |
| hashmap_im_i64/remove_5000              | 3.1237 ms  |
| hashmap_im_i64/remove_mut_5000          | 244.54 µs |
| hashmap_im_i64/remove_10000             | 6.5044 ms  |
| hashmap_im_i64/remove_mut_10000         | 434.27 µs |
| hashmap_im_i64/iter_100                 | 412.42 ns  |
| hashmap_im_i64/iter_1000                | 4.9301 µs |
| hashmap_im_i64/iter_5000                | 25.622 µs |
| hashmap_im_i64/iter_10000               | 45.138 µs |
| ordmap_i64/lookup_100                   | 1.0368 µs |
| ordmap_i64/lookup_1000                  | 19.349 µs |
| ordmap_i64/lookup_5000                  | 144.60 µs |
| ordmap_i64/lookup_10000                 | 330.26 µs |
| ordmap_i64/lookup_one_100               | 9.6837 ns  |
| ordmap_i64/lookup_one_1000              | 17.228 ns  |
| ordmap_i64/lookup_one_5000              | 23.092 ns  |
| ordmap_i64/lookup_one_10000             | 26.401 ns  |
| ordmap_i64/lookup_one_50000             | 33.863 ns  |
| ordmap_i64/lookup_one_100000            | 37.796 ns  |
| ordmap_i64/insert_100                   | 12.574 µs |
| ordmap_i64/insert_1000                  | 252.28 µs |
| ordmap_i64/insert_5000                  | 1.8961 ms  |
| ordmap_i64/insert_10000                 | 4.2308 ms  |
| ordmap_i64/insert_one_100               | 279.45 ns  |
| ordmap_i64/insert_one_1000              | 520.44 ns  |
| ordmap_i64/insert_one_5000              | 739.98 ns  |
| ordmap_i64/insert_one_10000             | 875.45 ns  |
| ordmap_i64/insert_mut_100               | 3.1733 µs |
| ordmap_i64/insert_mut_1000              | 46.225 µs |
| ordmap_i64/insert_mut_5000              | 331.52 µs |
| ordmap_i64/insert_mut_10000             | 721.44 µs |
| ordmap_i64/reinsert_mut_one_100         | 73.400 ns  |
| ordmap_i64/reinsert_mut_one_1000        | 112.90 ns  |
| ordmap_i64/reinsert_mut_one_5000        | 124.17 ns  |
| ordmap_i64/reinsert_mut_one_10000       | 133.14 ns  |
| ordmap_i64/reinsert_mut_one_50000       | 177.66 ns  |
| ordmap_i64/reinsert_mut_one_100000      | 205.23 ns  |
| ordmap_i64/remove_100                   | 13.661 µs |
| ordmap_i64/remove_mut_100               | 2.9135 µs |
| ordmap_i64/remove_1000                  | 262.40 µs |
| ordmap_i64/remove_mut_1000              | 44.231 µs |
| ordmap_i64/remove_5000                  | 1.8936 ms  |
| ordmap_i64/remove_mut_5000              | 347.88 µs |
| ordmap_i64/remove_10000                 | 4.3432 ms  |
| ordmap_i64/remove_mut_10000             | 758.97 µs |
| ordmap_i64/iter_100                     | 373.56 ns  |
| ordmap_i64/iter_1000                    | 3.5718 µs |
| ordmap_i64/iter_5000                    | 18.079 µs |
| ordmap_i64/iter_10000                   | 35.971 µs |
| indexmap_str/lookup_100                 | 3.0650 µs |
| indexmap_str/lookup_1000                | 54.386 µs |
| indexmap_str/lookup_5000                | 480.04 µs |
| indexmap_str/lookup_10000               | 1.0883 ms  |
| indexmap_str/lookup_one_100             | 30.792 ns  |
| indexmap_str/lookup_one_1000            | 41.655 ns  |
| indexmap_str/lookup_one_5000            | 48.936 ns  |
| indexmap_str/lookup_one_10000           | 54.372 ns  |
| indexmap_str/lookup_one_50000           | 59.495 ns  |
| indexmap_str/lookup_one_100000          | 61.018 ns  |
| indexmap_str/insert_100                 | 188.23 µs |
| indexmap_str/insert_1000                | 2.3437 ms  |
| indexmap_str/insert_5000                | 12.604 ms  |
| indexmap_str/insert_10000               | 27.103 ms  |
| indexmap_str/insert_one_100             | 2.5682 µs |
| indexmap_str/insert_one_1000            | 3.8403 µs |
| indexmap_str/insert_one_5000            | 3.7564 µs |
| indexmap_str/insert_one_10000           | 4.0194 µs |
| indexmap_str/insert_mut_100             | 24.468 µs |
| indexmap_str/insert_mut_1000            | 279.54 µs |
| indexmap_str/insert_mut_5000            | 1.4756 ms  |
| indexmap_str/insert_mut_10000           | 3.0149 ms  |
| indexmap_str/reinsert_mut_one_100       | 349.74 ns  |
| indexmap_str/reinsert_mut_one_1000      | 452.70 ns  |
| indexmap_str/reinsert_mut_one_5000      | 514.14 ns  |
| indexmap_str/reinsert_mut_one_10000     | 687.49 ns  |
| indexmap_str/reinsert_mut_one_50000     | 1.6078 µs |
| indexmap_str/reinsert_mut_one_100000    | 1.7296 µs |
| indexmap_str/remove_100                 | 169.35 µs |
| indexmap_str/remove_mut_100             | 28.777 µs |
| indexmap_str/remove_1000                | 1.9964 ms  |
| indexmap_str/remove_mut_1000            | 322.06 µs |
| indexmap_str/remove_5000                | 11.446 ms  |
| indexmap_str/remove_mut_5000            | 1.9411 ms  |
| indexmap_str/remove_10000               | 24.560 ms  |
| indexmap_str/remove_mut_10000           | 4.1186 ms  |
| indexmap_str/iter_100                   | 357.38 ns  |
| indexmap_str/iter_1000                  | 3.2336 µs |
| indexmap_str/iter_5000                  | 16.157 µs |
| indexmap_str/iter_10000                 | 32.394 µs |
| hashmap_std_str/lookup_100              | 2.0452 µs |
| hashmap_std_str/lookup_1000             | 22.240 µs |
| hashmap_std_str/lookup_one_100          | 20.653 ns  |
| hashmap_std_str/lookup_one_1000         | 20.553 ns  |
| hashmap_std_str/lookup_one_5000         | 20.935 ns  |
| hashmap_std_str/lookup_one_10000        | 20.572 ns  |
| hashmap_std_str/lookup_one_50000        | 21.068 ns  |
| hashmap_std_str/lookup_one_100000       | 20.583 ns  |
| hashmap_std_str/insert_100              | 379.87 µs |
| hashmap_std_str/insert_1000             | 34.895 ms  |
| hashmap_std_str/insert_one_100          | 9.4765 µs |
| hashmap_std_str/insert_one_1000         | 102.70 µs |
| hashmap_std_str/insert_mut_100          | 12.189 µs |
| hashmap_std_str/insert_mut_1000         | 135.24 µs |
| hashmap_std_str/reinsert_mut_one_100    | 97.538 ns  |
| hashmap_std_str/reinsert_mut_one_1000   | 95.362 ns  |
| hashmap_std_str/reinsert_mut_one_5000   | 116.50 ns  |
| hashmap_std_str/reinsert_mut_one_10000  | 123.62 ns  |
| hashmap_std_str/reinsert_mut_one_50000  | 166.37 ns  |
| hashmap_std_str/reinsert_mut_one_100000 | 270.89 ns  |
| hashmap_std_str/remove_100              | 371.27 µs |
| hashmap_std_str/remove_mut_100          | 9.9188 µs |
| hashmap_std_str/remove_1000             | 36.155 ms  |
| hashmap_std_str/remove_mut_1000         | 99.737 µs |
| hashmap_std_str/iter_100                | 95.154 ns  |
| hashmap_std_str/iter_1000               | 958.07 ns  |
| hashmap_im_str/lookup_100               | 2.1798 µs |
| hashmap_im_str/lookup_1000              | 29.428 µs |
| hashmap_im_str/lookup_5000              | 213.91 µs |
| hashmap_im_str/lookup_10000             | 458.73 µs |
| hashmap_im_str/lookup_one_100           | 21.164 ns  |
| hashmap_im_str/lookup_one_1000          | 21.569 ns  |
| hashmap_im_str/lookup_one_5000          | 22.403 ns  |
| hashmap_im_str/lookup_one_10000         | 22.387 ns  |
| hashmap_im_str/lookup_one_50000         | 23.220 ns  |
| hashmap_im_str/lookup_one_100000        | 23.307 ns  |
| hashmap_im_str/insert_100               | 121.57 µs |
| hashmap_im_str/insert_1000              | 1.8211 ms  |
| hashmap_im_str/insert_5000              | 9.0666 ms  |
| hashmap_im_str/insert_10000             | 18.116 ms  |
| hashmap_im_str/insert_one_100           | 1.4683 µs |
| hashmap_im_str/insert_one_1000          | 2.7807 µs |
| hashmap_im_str/insert_one_5000          | 2.1140 µs |
| hashmap_im_str/insert_one_10000         | 2.4953 µs |
| hashmap_im_str/insert_mut_100           | 12.138 µs |
| hashmap_im_str/insert_mut_1000          | 145.59 µs |
| hashmap_im_str/insert_mut_5000          | 720.02 µs |
| hashmap_im_str/insert_mut_10000         | 1.4145 ms  |
| hashmap_im_str/reinsert_mut_one_100     | 140.52 ns  |
| hashmap_im_str/reinsert_mut_one_1000    | 171.91 ns  |
| hashmap_im_str/reinsert_mut_one_5000    | 185.31 ns  |
| hashmap_im_str/reinsert_mut_one_10000   | 200.74 ns  |
| hashmap_im_str/reinsert_mut_one_50000   | 364.20 ns  |
| hashmap_im_str/reinsert_mut_one_100000  | 503.41 ns  |
| hashmap_im_str/remove_100               | 118.89 µs |
| hashmap_im_str/remove_mut_100           | 14.369 µs |
| hashmap_im_str/remove_1000              | 1.5951 ms  |
| hashmap_im_str/remove_mut_1000          | 163.88 µs |
| hashmap_im_str/remove_5000              | 8.7068 ms  |
| hashmap_im_str/remove_mut_5000          | 985.48 µs |
| hashmap_im_str/remove_10000             | 17.664 ms  |
| hashmap_im_str/remove_mut_10000         | 1.8837 ms  |
| hashmap_im_str/iter_100                 | 472.35 ns  |
| hashmap_im_str/iter_1000                | 5.3049 µs |
| hashmap_im_str/iter_5000                | 34.319 µs |
| hashmap_im_str/iter_10000               | 55.219 µs |
| ordmap_str/lookup_100                   | 3.1029 µs |
| ordmap_str/lookup_1000                  | 79.484 µs |
| ordmap_str/lookup_5000                  | 653.35 µs |
| ordmap_str/lookup_10000                 | 1.5351 ms  |
| ordmap_str/lookup_one_100               | 29.764 ns  |
| ordmap_str/lookup_one_1000              | 44.121 ns  |
| ordmap_str/lookup_one_5000              | 54.501 ns  |
| ordmap_str/lookup_one_10000             | 58.974 ns  |
| ordmap_str/lookup_one_50000             | 66.836 ns  |
| ordmap_str/lookup_one_100000            | 72.613 ns  |
| ordmap_str/insert_100                   | 131.72 µs |
| ordmap_str/insert_1000                  | 1.9043 ms  |
| ordmap_str/insert_5000                  | 11.479 ms  |
| ordmap_str/insert_10000                 | 25.308 ms  |
| ordmap_str/insert_one_100               | 1.7844 µs |
| ordmap_str/insert_one_1000              | 2.5660 µs |
| ordmap_str/insert_one_5000              | 3.3604 µs |
| ordmap_str/insert_one_10000             | 3.4313 µs |
| ordmap_str/insert_mut_100               | 13.780 µs |
| ordmap_str/insert_mut_1000              | 164.49 µs |
| ordmap_str/insert_mut_5000              | 944.96 µs |
| ordmap_str/insert_mut_10000             | 1.9325 ms  |
| ordmap_str/reinsert_mut_one_100         | 175.79 ns  |
| ordmap_str/reinsert_mut_one_1000        | 279.77 ns  |
| ordmap_str/reinsert_mut_one_5000        | 376.85 ns  |
| ordmap_str/reinsert_mut_one_10000       | 421.07 ns  |
| ordmap_str/reinsert_mut_one_50000       | 622.21 ns  |
| ordmap_str/reinsert_mut_one_100000      | 830.81 ns  |
| ordmap_str/remove_100                   | 104.00 µs |
| ordmap_str/remove_mut_100               | 19.585 µs |
| ordmap_str/remove_1000                  | 1.5868 ms  |
| ordmap_str/remove_mut_1000              | 260.93 µs |
| ordmap_str/remove_5000                  | 10.019 ms  |
| ordmap_str/remove_mut_5000              | 1.6258 ms  |
| ordmap_str/remove_10000                 | 22.802 ms  |
| ordmap_str/remove_mut_10000             | 3.5936 ms  |
| ordmap_str/iter_100                     | 350.50 ns  |
| ordmap_str/iter_1000                    | 3.2479 µs |
| ordmap_str/iter_5000                    | 15.930 µs |
| ordmap_str/iter_10000                   | 32.629 µs |
| indexmap_big/lookup_100                 | 80.410 µs |
| indexmap_big/lookup_1000                | 809.99 µs |
| indexmap_big/lookup_5000                | 4.5495 ms  |
| indexmap_big/lookup_10000               | 11.623 ms  |
| indexmap_big/lookup_one_100             | 806.92 ns  |
| indexmap_big/lookup_one_1000            | 819.77 ns  |
| indexmap_big/lookup_one_5000            | 828.08 ns  |
| indexmap_big/lookup_one_10000           | 834.79 ns  |
| indexmap_big/lookup_one_50000           | 847.91 ns  |
| indexmap_big/lookup_one_100000          | 847.40 ns  |
| indexmap_big/insert_100                 | 543.86 µs |
| indexmap_big/insert_1000                | 6.9731 ms  |
| indexmap_big/insert_5000                | 63.175 ms  |
| indexmap_big/insert_10000               | 352.30 ms  |
| indexmap_big/insert_one_100             | 19.549 µs |
| indexmap_big/insert_one_1000            | 19.130 µs |
| indexmap_big/insert_one_5000            | 43.709 µs |
| indexmap_big/insert_one_10000           | 63.913 µs |
| indexmap_big/insert_mut_100             | 216.70 µs |
| indexmap_big/insert_mut_1000            | 3.3218 ms  |
| indexmap_big/insert_mut_5000            | 22.296 ms  |
| indexmap_big/insert_mut_10000           | 69.266 ms  |
| indexmap_big/reinsert_mut_one_100       | 3.1892 µs |
| indexmap_big/reinsert_mut_one_1000      | 3.1655 µs |
| indexmap_big/reinsert_mut_one_5000      | 7.9692 µs |
| indexmap_big/reinsert_mut_one_10000     | 9.9066 µs |
| indexmap_big/reinsert_mut_one_50000     | 18.632 µs |
| indexmap_big/reinsert_mut_one_100000    | 28.986 µs |
| indexmap_big/remove_100                 | 697.82 µs |
| indexmap_big/remove_mut_100             | 84.346 µs |
| indexmap_big/remove_1000                | 7.1701 ms  |
| indexmap_big/remove_mut_1000            | 727.28 µs |
| indexmap_big/remove_5000                | 95.516 ms  |
| indexmap_big/remove_mut_5000            | 8.7223 ms  |
| indexmap_big/remove_10000               | 489.87 ms  |
| indexmap_big/remove_mut_10000           | 61.714 ms  |
| indexmap_big/iter_100                   | 12.834 ns  |
| indexmap_big/iter_1000                  | 12.865 ns  |
| indexmap_big/iter_5000                  | 3.1720 µs |
| indexmap_big/iter_10000                 | 21.011 µs |
| hashmap_std_big/lookup_100              | 81.331 µs |
| hashmap_std_big/lookup_1000             | 825.56 µs |
| hashmap_std_big/lookup_one_100          | 827.09 ns  |
| hashmap_std_big/lookup_one_1000         | 810.70 ns  |
| hashmap_std_big/lookup_one_5000         | 810.42 ns  |
| hashmap_std_big/lookup_one_10000        | 826.26 ns  |
| hashmap_std_big/lookup_one_50000        | 822.01 ns  |
| hashmap_std_big/lookup_one_100000       | 843.61 ns  |
| hashmap_std_big/insert_100              | 263.09 µs |
| hashmap_std_big/insert_1000             | 3.5598 ms  |
| hashmap_std_big/insert_one_100          | 20.974 µs |
| hashmap_std_big/insert_one_1000         | 68.891 µs |
| hashmap_std_big/insert_mut_100          | 197.65 µs |
| hashmap_std_big/insert_mut_1000         | 2.6993 ms  |
| hashmap_std_big/reinsert_mut_one_100    | 1.7852 µs |
| hashmap_std_big/reinsert_mut_one_1000   | 1.7822 µs |
| hashmap_std_big/reinsert_mut_one_5000   | 2.0267 µs |
| hashmap_std_big/reinsert_mut_one_10000  | 2.5829 µs |
| hashmap_std_big/reinsert_mut_one_50000  | 2.7928 µs |
| hashmap_std_big/reinsert_mut_one_100000 | 2.8155 µs |
| hashmap_std_big/remove_100              | 904.78 µs |
| hashmap_std_big/remove_mut_100          | 85.359 µs |
| hashmap_std_big/remove_1000             | 18.878 ms  |
| hashmap_std_big/remove_mut_1000         | 735.51 µs |
| hashmap_std_big/iter_100                | 2.3706 ns  |
| hashmap_std_big/iter_1000               | 28.152 ns  |
| hashmap_im_big/lookup_100               | 79.810 µs |
| hashmap_im_big/lookup_1000              | 801.59 µs |
| hashmap_im_big/lookup_5000              | 4.3529 ms  |
| hashmap_im_big/lookup_10000             | 10.374 ms  |
| hashmap_im_big/lookup_one_100           | 796.61 ns  |
| hashmap_im_big/lookup_one_1000          | 796.57 ns  |
| hashmap_im_big/lookup_one_5000          | 802.59 ns  |
| hashmap_im_big/lookup_one_10000         | 806.33 ns  |
| hashmap_im_big/lookup_one_50000         | 802.34 ns  |
| hashmap_im_big/lookup_one_100000        | 808.42 ns  |
| hashmap_im_big/insert_100               | 3.2327 ms  |
| hashmap_im_big/insert_1000              | 31.886 ms  |
| hashmap_im_big/insert_5000              | 271.17 ms  |
| hashmap_im_big/insert_10000             | 669.62 ms  |
| hashmap_im_big/insert_one_100           | 37.076 µs |
| hashmap_im_big/insert_one_1000          | 35.369 µs |
| hashmap_im_big/insert_one_5000          | 84.775 µs |
| hashmap_im_big/insert_one_10000         | 80.641 µs |
| hashmap_im_big/insert_mut_100           | 264.22 µs |
| hashmap_im_big/insert_mut_1000          | 3.0612 ms  |
| hashmap_im_big/insert_mut_5000          | 19.055 ms  |
| hashmap_im_big/insert_mut_10000         | 46.001 ms  |
| hashmap_im_big/reinsert_mut_one_100     | 1.9416 µs |
| hashmap_im_big/reinsert_mut_one_1000    | 1.9076 µs |
| hashmap_im_big/reinsert_mut_one_5000    | 2.7859 µs |
| hashmap_im_big/reinsert_mut_one_10000   | 3.3898 µs |
| hashmap_im_big/reinsert_mut_one_50000   | 44.432 µs |
| hashmap_im_big/reinsert_mut_one_100000  | 25.509 µs |
| hashmap_im_big/remove_100               | 2.9397 ms  |
| hashmap_im_big/remove_mut_100           | 101.83 µs |
| hashmap_im_big/remove_1000              | 28.950 ms  |
| hashmap_im_big/remove_mut_1000          | 744.02 µs |
| hashmap_im_big/remove_5000              | 279.20 ms  |
| hashmap_im_big/remove_mut_5000          | 7.4017 ms  |
| hashmap_im_big/remove_10000             | 792.99 ms  |
| hashmap_im_big/remove_mut_10000         | 45.893 ms  |
| hashmap_im_big/iter_100                 | 6.2090 ns  |
| hashmap_im_big/iter_1000                | 6.3539 ns  |
| hashmap_im_big/iter_5000                | 5.7284 µs |
| hashmap_im_big/iter_10000               | 75.595 µs |
| ordmap_big/lookup_100                   | 11.472 µs |
| ordmap_big/lookup_1000                  | 117.55 µs |
| ordmap_big/lookup_5000                  | 2.6823 ms  |
| ordmap_big/lookup_10000                 | 15.273 ms  |
| ordmap_big/lookup_one_100               | 99.276 ns  |
| ordmap_big/lookup_one_1000              | 98.506 ns  |
| ordmap_big/lookup_one_5000              | 270.34 ns  |
| ordmap_big/lookup_one_10000             | 745.11 ns  |
| ordmap_big/lookup_one_50000             | 1.0473 µs |
| ordmap_big/lookup_one_100000            | 1.1715 µs |
| ordmap_big/insert_100                   | 600.14 µs |
| ordmap_big/insert_1000                  | 6.5770 ms  |
| ordmap_big/insert_5000                  | 86.632 ms  |
| ordmap_big/insert_10000                 | 287.77 ms  |
| ordmap_big/insert_one_100               | 5.2731 µs |
| ordmap_big/insert_one_1000              | 5.3469 µs |
| ordmap_big/insert_one_5000              | 31.370 µs |
| ordmap_big/insert_one_10000             | 47.451 µs |
| ordmap_big/insert_mut_100               | 200.79 µs |
| ordmap_big/insert_mut_1000              | 2.3980 ms  |
| ordmap_big/insert_mut_5000              | 17.611 ms  |
| ordmap_big/insert_mut_10000             | 63.231 ms  |
| ordmap_big/reinsert_mut_one_100         | 876.28 ns  |
| ordmap_big/reinsert_mut_one_1000        | 894.76 ns  |
| ordmap_big/reinsert_mut_one_5000        | 2.7872 µs |
| ordmap_big/reinsert_mut_one_10000       | 5.6768 µs |
| ordmap_big/reinsert_mut_one_50000       | 7.5608 µs |
| ordmap_big/reinsert_mut_one_100000      | 7.8761 µs |
| ordmap_big/remove_100                   | 356.56 µs |
| ordmap_big/remove_mut_100               | 15.089 µs |
| ordmap_big/remove_1000                  | 3.6679 ms  |
| ordmap_big/remove_mut_1000              | 112.21 µs |
| ordmap_big/remove_5000                  | 77.937 ms  |
| ordmap_big/remove_mut_5000              | 5.2889 ms  |
| ordmap_big/remove_10000                 | 338.50 ms  |
| ordmap_big/remove_mut_10000             | 40.966 ms  |
| ordmap_big/iter_100                     | 11.480 ns  |
| ordmap_big/iter_1000                    | 11.590 ns  |
| ordmap_big/iter_5000                    | 3.2357 µs |
| ordmap_big/iter_10000                   | 22.375 µs |
| indexmap_specific_i64/get_index_100     | 211.39 ns  |
| indexmap_specific_i64/front_100         | 1.0480 ns  |
| indexmap_specific_i64/back_100          | 1.5699 ns  |
| indexmap_specific_i64/get_index_1000    | 1.8051 µs |
| indexmap_specific_i64/front_1000        | 1.6928 ns  |
| indexmap_specific_i64/back_1000         | 2.6651 ns  |
| indexmap_specific_i64/get_index_5000    | 8.6553 µs |
| indexmap_specific_i64/front_5000        | 2.4302 ns  |
| indexmap_specific_i64/back_5000         | 3.8360 ns  |
| indexmap_specific_i64/get_index_10000   | 17.282 µs |
| indexmap_specific_i64/front_10000       | 2.4209 ns  |
| indexmap_specific_i64/back_10000        | 3.8483 ns  |
| indexmap_specific_i64/get_index_50000   | 89.121 µs |
| indexmap_specific_i64/front_50000       | 3.3578 ns  |
| indexmap_specific_i64/back_50000        | 5.1246 ns  |
| indexmap_specific_i64/get_index_100000  | 188.56 µs |
| indexmap_specific_i64/front_100000      | 3.3614 ns  |
| indexmap_specific_i64/back_100000       | 5.1786 ns  |
| indexmap_specific_i64/get_index_500000  | 1.0918 ms  |
| indexmap_specific_i64/front_500000      | 4.2383 ns  |
| indexmap_specific_i64/back_500000       | 6.5517 ns  |
| indexmap_specific_i64/get_index_1000000 | 2.6469 ms  |
| indexmap_specific_i64/front_1000000     | 5.3746 ns  |
| indexmap_specific_i64/back_1000000      | 8.3259 ns  |
| hashmap_std_i64/insert_mut_5000         | 182.11 µs |
| hashmap_std_i64/insert_mut_10000        | 364.90 µs |
| hashmap_std_i64/remove_mut_5000         | 78.309 µs |
| hashmap_std_i64/remove_mut_10000        | 160.77 µs |
| hashmap_std_i64/iter_5000               | 4.1492 µs |
| hashmap_std_i64/iter_10000              | 9.5853 µs |
| hashmap_std_str/insert_mut_5000         | 661.03 µs |
| hashmap_std_str/insert_mut_10000        | 1.4271 ms  |
| hashmap_std_str/remove_mut_5000         | 592.09 µs |
| hashmap_std_str/remove_mut_10000        | 1.2500 ms  |
| hashmap_std_str/iter_5000               | 4.9830 µs |
| hashmap_std_str/iter_10000              | 11.094 µs |
| hashmap_std_big/insert_mut_5000         | 24.665 ms  |
| hashmap_std_big/insert_mut_10000        | 80.239 ms  |
| hashmap_std_big/remove_mut_5000         | 8.2428 ms  |
| hashmap_std_big/remove_mut_10000        | 34.730 ms  |
| hashmap_std_big/iter_5000               | 811.15 ns  |
| hashmap_std_big/iter_10000              | 4.2559 µs |

Scripts used to generate summaries: (nushell)
```nushell
let bench_times = open bench.stdout | collect | parse -r '(?m)^(.+?)\s+time:\s+\[\S+ \S+ ([\d\.]+ \S+)' | rename bench time

$bench_times | to md --pretty
```

```nushell
let subtitles = {
    insert: "(linear) Time to create a map of size N by repeatedly inserting into it via immutable (cloned) updates",
    insert_one: "(constant) Given a map of size N, time to insert one element immutably (cloning the map before inserting)",
    insert_mut: "(linear) Time to create a map of size N by repeatedly inserting into it via mutable inserts",
    reinsert_mut_one: "(constant) Given a map of size N, time to clone, remove mutably, and reinsert a key to the map.\n\nAs IndexMap tracks insertion order, it is the only one that ultimately produces a different (!=) result after the operation.",
    remove: "(linear) Given a map of size N, time to drain all its items by repeatedly .remove() (that clones) in a random order",
    remove_mut: "(linear) Given a map of size N, time to drain all its items by repeatedly calling .remove(), mutably, in a random order\n\nNote that `.drain()`ing the collection might be considerably faster",
    lookup: "(linear) Given a map of size N, time to lookup every one of its elements in a random order",
    lookup_one: "(constant) Given a map of size N, time to look up a single, random element",
    iter: "(linear) Given a map of size N, time to iterate all its items in the default order by reference",
}

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
    | select dtype batch hashmap_std? hashmap_im? ordmap? indexmap?
    | update cells { default - }

    let subtitle = if $test in $subtitles { $"($subtitles | get $test)\n\n" } else { "" }

    $"## ($test)\n($subtitle)($displayrows | to md --pretty)"
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
(linear) Time to create a map of size N by repeatedly inserting into it via immutable (cloned) updates

| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 8.0567 µs (1.0x) | 25.141 µs (3.1x) | 12.574 µs (1.6x) | 44.440 µs (5.5x) |
| i64   | 1000  | 311.13 µs (1.2x) | 380.23 µs (1.5x) | 252.28 µs (1.0x) | 739.50 µs (2.9x) |
| i64   | 5000  | -                 | 2.9101 ms (1.5x)  | 1.8961 ms (1.0x)  | 5.2003 ms (2.7x)  |
| i64   | 10000 | -                 | 6.0241 ms (1.4x)  | 4.2308 ms (1.0x)  | 10.826 ms (2.6x)  |
| str   | 100   | 379.87 µs (3.1x) | 121.57 µs (1.0x) | 131.72 µs (1.1x) | 188.23 µs (1.5x) |
| str   | 1000  | 34.895 ms (19.2x) | 1.8211 ms (1.0x)  | 1.9043 ms (1.0x)  | 2.3437 ms (1.3x)  |
| str   | 5000  | -                 | 9.0666 ms (1.0x)  | 11.479 ms (1.3x)  | 12.604 ms (1.4x)  |
| str   | 10000 | -                 | 18.116 ms (1.0x)  | 25.308 ms (1.4x)  | 27.103 ms (1.5x)  |
| big   | 100   | 263.09 µs (1.0x) | 3.2327 ms (12.3x) | 600.14 µs (2.3x) | 543.86 µs (2.1x) |
| big   | 1000  | 3.5598 ms (1.0x)  | 31.886 ms (9.0x)  | 6.5770 ms (1.8x)  | 6.9731 ms (2.0x)  |
| big   | 5000  | -                 | 271.17 ms (4.3x)  | 86.632 ms (1.4x)  | 63.175 ms (1.0x)  |
| big   | 10000 | -                 | 669.62 ms (2.3x)  | 287.77 ms (1.0x)  | 352.30 ms (1.2x)  |

## insert_mut
(linear) Time to create a map of size N by repeatedly inserting into it via mutable inserts

| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 3.2507 µs (1.0x) | 3.4218 µs (1.1x) | 3.1733 µs (1.0x) | 7.6923 µs (2.4x) |
| i64   | 1000  | 39.664 µs (1.0x) | 46.224 µs (1.2x) | 46.225 µs (1.2x) | 109.36 µs (2.8x) |
| i64   | 5000  | 182.11 µs (1.0x) | 193.98 µs (1.1x) | 331.52 µs (1.8x) | 559.90 µs (3.1x) |
| i64   | 10000 | 364.90 µs (1.1x) | 342.06 µs (1.0x) | 721.44 µs (2.1x) | 1.1089 ms (3.2x)  |
| str   | 100   | 12.189 µs (1.0x) | 12.138 µs (1.0x) | 13.780 µs (1.1x) | 24.468 µs (2.0x) |
| str   | 1000  | 135.24 µs (1.0x) | 145.59 µs (1.1x) | 164.49 µs (1.2x) | 279.54 µs (2.1x) |
| str   | 5000  | 661.03 µs (1.0x) | 720.02 µs (1.1x) | 944.96 µs (1.4x) | 1.4756 ms (2.2x)  |
| str   | 10000 | 1.4271 ms (1.0x)  | 1.4145 ms (1.0x)  | 1.9325 ms (1.4x)  | 3.0149 ms (2.1x)  |
| big   | 100   | 197.65 µs (1.0x) | 264.22 µs (1.3x) | 200.79 µs (1.0x) | 216.70 µs (1.1x) |
| big   | 1000  | 2.6993 ms (1.1x)  | 3.0612 ms (1.3x)  | 2.3980 ms (1.0x)  | 3.3218 ms (1.4x)  |
| big   | 5000  | 24.665 ms (1.4x)  | 19.055 ms (1.1x)  | 17.611 ms (1.0x)  | 22.296 ms (1.3x)  |
| big   | 10000 | 80.239 ms (1.7x)  | 46.001 ms (1.0x)  | 63.231 ms (1.4x)  | 69.266 ms (1.5x)  |

## insert_one
(constant) Given a map of size N, time to insert one element immutably (cloning the map before inserting)

| dtype | batch | hashmap_std        | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ------------------ | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 233.15 ns (1.0x)   | 525.48 ns (2.3x)  | 279.45 ns (1.2x)  | 927.15 ns (4.0x)  |
| i64   | 1000  | 2.5858 µs (5.0x)  | 915.39 ns (1.8x)  | 520.44 ns (1.0x)  | 1.5553 µs (3.0x) |
| i64   | 5000  | -                  | 998.19 ns (1.4x)  | 739.98 ns (1.0x)  | 1.8646 µs (2.5x) |
| i64   | 10000 | -                  | 992.82 ns (1.1x)  | 875.45 ns (1.0x)  | 1.9254 µs (2.2x) |
| str   | 100   | 9.4765 µs (6.5x)  | 1.4683 µs (1.0x) | 1.7844 µs (1.2x) | 2.5682 µs (1.7x) |
| str   | 1000  | 102.70 µs (40.0x) | 2.7807 µs (1.1x) | 2.5660 µs (1.0x) | 3.8403 µs (1.5x) |
| str   | 5000  | -                  | 2.1140 µs (1.0x) | 3.3604 µs (1.6x) | 3.7564 µs (1.8x) |
| str   | 10000 | -                  | 2.4953 µs (1.0x) | 3.4313 µs (1.4x) | 4.0194 µs (1.6x) |
| big   | 100   | 20.974 µs (4.0x)  | 37.076 µs (7.0x) | 5.2731 µs (1.0x) | 19.549 µs (3.7x) |
| big   | 1000  | 68.891 µs (12.9x) | 35.369 µs (6.6x) | 5.3469 µs (1.0x) | 19.130 µs (3.6x) |
| big   | 5000  | -                  | 84.775 µs (2.7x) | 31.370 µs (1.0x) | 43.709 µs (1.4x) |
| big   | 10000 | -                  | 80.641 µs (1.7x) | 47.451 µs (1.0x) | 63.913 µs (1.3x) |

## iter
(linear) Given a map of size N, time to iterate all its items in the default order by reference

| dtype | batch | hashmap_std       | hashmap_im         | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ------------------ | ----------------- | ----------------- |
| i64   | 100   | 72.652 ns (1.0x)  | 412.42 ns (5.7x)   | 373.56 ns (5.2x)  | 365.04 ns (5.1x)  |
| i64   | 1000  | 713.17 ns (1.0x)  | 4.9301 µs (6.9x)  | 3.5718 µs (5.0x) | 3.5792 µs (5.0x) |
| i64   | 5000  | 4.1492 µs (1.0x) | 25.622 µs (6.2x)  | 18.079 µs (4.4x) | 16.670 µs (4.0x) |
| i64   | 10000 | 9.5853 µs (1.0x) | 45.138 µs (4.7x)  | 35.971 µs (3.8x) | 33.166 µs (3.5x) |
| str   | 100   | 95.154 ns (1.0x)  | 472.35 ns (5.0x)   | 350.50 ns (3.7x)  | 357.38 ns (3.8x)  |
| str   | 1000  | 958.07 ns (1.0x)  | 5.3049 µs (5.5x)  | 3.2479 µs (3.4x) | 3.2336 µs (3.4x) |
| str   | 5000  | 4.9830 µs (1.0x) | 34.319 µs (6.9x)  | 15.930 µs (3.2x) | 16.157 µs (3.2x) |
| str   | 10000 | 11.094 µs (1.0x) | 55.219 µs (5.0x)  | 32.629 µs (2.9x) | 32.394 µs (2.9x) |
| big   | 100   | 2.3706 ns (1.0x)  | 6.2090 ns (3.0x)   | 11.480 ns (5.5x)  | 12.834 ns (6.0x)  |
| big   | 1000  | 28.152 ns (4.7x)  | 6.3539 ns (1.0x)   | 11.590 ns (1.8x)  | 12.865 ns (2.0x)  |
| big   | 5000  | 811.15 ns (1.0x)  | 5.7284 µs (7.1x)  | 3.2357 µs (4.0x) | 3.1720 µs (3.9x) |
| big   | 10000 | 4.2559 µs (1.0x) | 75.595 µs (17.8x) | 22.375 µs (5.3x) | 21.011 µs (4.9x) |

## lookup
(linear) Given a map of size N, time to lookup every one of its elements in a random order

| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 1.1313 µs (1.1x) | 1.2196 µs (1.2x) | 1.0368 µs (1.0x) | 2.2112 µs (2.1x) |
| i64   | 1000  | 11.145 µs (1.0x) | 12.789 µs (1.1x) | 19.349 µs (1.7x) | 35.510 µs (3.2x) |
| i64   | 5000  | -                 | 72.025 µs (1.0x) | 144.60 µs (2.0x) | 352.27 µs (4.9x) |
| i64   | 10000 | -                 | 158.11 µs (1.0x) | 330.26 µs (2.1x) | 843.33 µs (5.3x) |
| str   | 100   | 2.0452 µs (1.0x) | 2.1798 µs (1.1x) | 3.1029 µs (1.5x) | 3.0650 µs (1.5x) |
| str   | 1000  | 22.240 µs (1.0x) | 29.428 µs (1.3x) | 79.484 µs (3.6x) | 54.386 µs (2.4x) |
| str   | 5000  | -                 | 213.91 µs (1.0x) | 653.35 µs (3.1x) | 480.04 µs (2.2x) |
| str   | 10000 | -                 | 458.73 µs (1.0x) | 1.5351 ms (3.3x)  | 1.0883 ms (2.4x)  |
| big   | 100   | 81.331 µs (7.1x) | 79.810 µs (7.0x) | 11.472 µs (1.0x) | 80.410 µs (7.0x) |
| big   | 1000  | 825.56 µs (7.0x) | 801.59 µs (6.8x) | 117.55 µs (1.0x) | 809.99 µs (6.9x) |
| big   | 5000  | -                 | 4.3529 ms (1.6x)  | 2.6823 ms (1.0x)  | 4.5495 ms (1.7x)  |
| big   | 10000 | -                 | 10.374 ms (1.0x)  | 15.273 ms (1.5x)  | 11.623 ms (1.1x)  |

## lookup_one
(constant) Given a map of size N, time to look up a single, random element

| dtype | batch  | hashmap_std      | hashmap_im       | ordmap            | indexmap         |
| ----- | ------ | ---------------- | ---------------- | ----------------- | ---------------- |
| i64   | 100    | 11.213 ns (1.2x) | 12.904 ns (1.3x) | 9.6837 ns (1.0x)  | 23.071 ns (2.6x) |
| i64   | 1000   | 11.154 ns (1.0x) | 13.386 ns (1.2x) | 17.228 ns (1.5x)  | 33.585 ns (3.0x) |
| i64   | 5000   | 11.152 ns (1.0x) | 13.632 ns (1.2x) | 23.092 ns (2.1x)  | 41.951 ns (3.7x) |
| i64   | 10000  | 11.140 ns (1.0x) | 13.768 ns (1.2x) | 26.401 ns (2.4x)  | 44.634 ns (4.0x) |
| i64   | 50000  | 11.161 ns (1.0x) | 13.874 ns (1.2x) | 33.863 ns (3.0x)  | 52.835 ns (4.7x) |
| i64   | 100000 | 11.214 ns (1.0x) | 14.326 ns (1.3x) | 37.796 ns (3.4x)  | 54.579 ns (4.9x) |
| str   | 100    | 20.653 ns (1.0x) | 21.164 ns (1.1x) | 29.764 ns (1.5x)  | 30.792 ns (1.5x) |
| str   | 1000   | 20.553 ns (1.0x) | 21.569 ns (1.1x) | 44.121 ns (2.2x)  | 41.655 ns (2.1x) |
| str   | 5000   | 20.935 ns (1.0x) | 22.403 ns (1.1x) | 54.501 ns (2.7x)  | 48.936 ns (2.4x) |
| str   | 10000  | 20.572 ns (1.0x) | 22.387 ns (1.1x) | 58.974 ns (2.9x)  | 54.372 ns (2.7x) |
| str   | 50000  | 21.068 ns (1.0x) | 23.220 ns (1.1x) | 66.836 ns (3.1x)  | 59.495 ns (2.8x) |
| str   | 100000 | 20.583 ns (1.0x) | 23.307 ns (1.2x) | 72.613 ns (3.6x)  | 61.018 ns (3.1x) |
| big   | 100    | 827.09 ns (8.4x) | 796.61 ns (8.0x) | 99.276 ns (1.0x)  | 806.92 ns (8.1x) |
| big   | 1000   | 810.70 ns (8.3x) | 796.57 ns (8.1x) | 98.506 ns (1.0x)  | 819.77 ns (8.4x) |
| big   | 5000   | 810.42 ns (3.0x) | 802.59 ns (3.0x) | 270.34 ns (1.0x)  | 828.08 ns (3.1x) |
| big   | 10000  | 826.26 ns (1.1x) | 806.33 ns (1.1x) | 745.11 ns (1.0x)  | 834.79 ns (1.1x) |
| big   | 50000  | 822.01 ns (1.0x) | 802.34 ns (1.0x) | 1.0473 µs (1.3x) | 847.91 ns (1.1x) |
| big   | 100000 | 843.61 ns (1.0x) | 808.42 ns (1.0x) | 1.1715 µs (1.4x) | 847.40 ns (1.0x) |

## reinsert_mut_one
(constant) Given a map of size N, time to clone, remove mutably, and reinsert a key to the map.

As IndexMap tracks insertion order, it is the only one that ultimately produces a different (!=) result after the operation.

| dtype | batch  | hashmap_std       | hashmap_im         | ordmap            | indexmap           |
| ----- | ------ | ----------------- | ------------------ | ----------------- | ------------------ |
| i64   | 100    | 38.032 ns (1.0x)  | 49.202 ns (1.3x)   | 73.400 ns (1.9x)  | 143.85 ns (3.8x)   |
| i64   | 1000   | 32.707 ns (1.0x)  | 73.991 ns (2.3x)   | 112.90 ns (3.5x)  | 213.81 ns (6.7x)   |
| i64   | 5000   | 36.044 ns (1.0x)  | 72.559 ns (2.0x)   | 124.17 ns (3.4x)  | 235.55 ns (6.5x)   |
| i64   | 10000  | 38.546 ns (1.0x)  | 65.405 ns (1.7x)   | 133.14 ns (3.5x)  | 252.18 ns (6.6x)   |
| i64   | 50000  | 53.511 ns (1.0x)  | 125.47 ns (2.4x)   | 177.66 ns (3.3x)  | 392.75 ns (7.4x)   |
| i64   | 100000 | 61.325 ns (1.0x)  | 127.28 ns (2.1x)   | 205.23 ns (3.4x)  | 459.31 ns (7.5x)   |
| str   | 100    | 97.538 ns (1.0x)  | 140.52 ns (1.4x)   | 175.79 ns (1.8x)  | 349.74 ns (3.6x)   |
| str   | 1000   | 95.362 ns (1.0x)  | 171.91 ns (1.8x)   | 279.77 ns (2.9x)  | 452.70 ns (4.8x)   |
| str   | 5000   | 116.50 ns (1.0x)  | 185.31 ns (1.6x)   | 376.85 ns (3.2x)  | 514.14 ns (4.4x)   |
| str   | 10000  | 123.62 ns (1.0x)  | 200.74 ns (1.6x)   | 421.07 ns (3.4x)  | 687.49 ns (5.6x)   |
| str   | 50000  | 166.37 ns (1.0x)  | 364.20 ns (2.2x)   | 622.21 ns (3.7x)  | 1.6078 µs (9.7x)  |
| str   | 100000 | 270.89 ns (1.0x)  | 503.41 ns (1.9x)   | 830.81 ns (3.1x)  | 1.7296 µs (6.4x)  |
| big   | 100    | 1.7852 µs (2.0x) | 1.9416 µs (2.2x)  | 876.28 ns (1.0x)  | 3.1892 µs (3.6x)  |
| big   | 1000   | 1.7822 µs (2.0x) | 1.9076 µs (2.1x)  | 894.76 ns (1.0x)  | 3.1655 µs (3.5x)  |
| big   | 5000   | 2.0267 µs (1.0x) | 2.7859 µs (1.4x)  | 2.7872 µs (1.4x) | 7.9692 µs (3.9x)  |
| big   | 10000  | 2.5829 µs (1.0x) | 3.3898 µs (1.3x)  | 5.6768 µs (2.2x) | 9.9066 µs (3.8x)  |
| big   | 50000  | 2.7928 µs (1.0x) | 44.432 µs (15.9x) | 7.5608 µs (2.7x) | 18.632 µs (6.7x)  |
| big   | 100000 | 2.8155 µs (1.0x) | 25.509 µs (9.1x)  | 7.8761 µs (2.8x) | 28.986 µs (10.3x) |

## remove
(linear) Given a map of size N, time to drain all its items by repeatedly .remove() (that clones) in a random order

| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 7.5556 µs (1.0x) | 23.082 µs (3.1x) | 13.661 µs (1.8x) | 39.084 µs (5.2x) |
| i64   | 1000  | 952.30 µs (3.6x) | 457.65 µs (1.7x) | 262.40 µs (1.0x) | 786.99 µs (3.0x) |
| i64   | 5000  | -                 | 3.1237 ms (1.6x)  | 1.8936 ms (1.0x)  | 5.3150 ms (2.8x)  |
| i64   | 10000 | -                 | 6.5044 ms (1.5x)  | 4.3432 ms (1.0x)  | 11.486 ms (2.6x)  |
| str   | 100   | 371.27 µs (3.6x) | 118.89 µs (1.1x) | 104.00 µs (1.0x) | 169.35 µs (1.6x) |
| str   | 1000  | 36.155 ms (22.8x) | 1.5951 ms (1.0x)  | 1.5868 ms (1.0x)  | 1.9964 ms (1.3x)  |
| str   | 5000  | -                 | 8.7068 ms (1.0x)  | 10.019 ms (1.2x)  | 11.446 ms (1.3x)  |
| str   | 10000 | -                 | 17.664 ms (1.0x)  | 22.802 ms (1.3x)  | 24.560 ms (1.4x)  |
| big   | 100   | 904.78 µs (2.5x) | 2.9397 ms (8.2x)  | 356.56 µs (1.0x) | 697.82 µs (2.0x) |
| big   | 1000  | 18.878 ms (5.1x)  | 28.950 ms (7.9x)  | 3.6679 ms (1.0x)  | 7.1701 ms (2.0x)  |
| big   | 5000  | -                 | 279.20 ms (3.6x)  | 77.937 ms (1.0x)  | 95.516 ms (1.2x)  |
| big   | 10000 | -                 | 792.99 ms (2.3x)  | 338.50 ms (1.0x)  | 489.87 ms (1.4x)  |

## remove_mut
(linear) Given a map of size N, time to drain all its items by repeatedly calling .remove(), mutably, in a random order

Note that `.drain()`ing the collection might be considerably faster

| dtype | batch | hashmap_std       | hashmap_im        | ordmap            | indexmap          |
| ----- | ----- | ----------------- | ----------------- | ----------------- | ----------------- |
| i64   | 100   | 1.2943 µs (1.0x) | 3.7433 µs (2.9x) | 2.9135 µs (2.3x) | 6.5872 µs (5.1x) |
| i64   | 1000  | 13.460 µs (1.0x) | 44.098 µs (3.3x) | 44.231 µs (3.3x) | 95.437 µs (7.1x) |
| i64   | 5000  | 78.309 µs (1.0x) | 244.54 µs (3.1x) | 347.88 µs (4.4x) | 692.17 µs (8.8x) |
| i64   | 10000 | 160.77 µs (1.0x) | 434.27 µs (2.7x) | 758.97 µs (4.7x) | 1.4346 ms (8.9x)  |
| str   | 100   | 9.9188 µs (1.0x) | 14.369 µs (1.4x) | 19.585 µs (2.0x) | 28.777 µs (2.9x) |
| str   | 1000  | 99.737 µs (1.0x) | 163.88 µs (1.6x) | 260.93 µs (2.6x) | 322.06 µs (3.2x) |
| str   | 5000  | 592.09 µs (1.0x) | 985.48 µs (1.7x) | 1.6258 ms (2.7x)  | 1.9411 ms (3.3x)  |
| str   | 10000 | 1.2500 ms (1.0x)  | 1.8837 ms (1.5x)  | 3.5936 ms (2.9x)  | 4.1186 ms (3.3x)  |
| big   | 100   | 85.359 µs (5.7x) | 101.83 µs (6.7x) | 15.089 µs (1.0x) | 84.346 µs (5.6x) |
| big   | 1000  | 735.51 µs (6.6x) | 744.02 µs (6.6x) | 112.21 µs (1.0x) | 727.28 µs (6.5x) |
| big   | 5000  | 8.2428 ms (1.6x)  | 7.4017 ms (1.4x)  | 5.2889 ms (1.0x)  | 8.7223 ms (1.6x)  |
| big   | 10000 | 34.730 ms (1.0x)  | 45.893 ms (1.3x)  | 40.966 ms (1.2x)  | 61.714 ms (1.8x)  |

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
- Was it worth it to include the `big` datatype (a 4kb stack-allocated array) in the benchmarks? Not really, all it did was make the 10k insert stdlib benchmark take 20min for the 10 samples
