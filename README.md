A persistent (through immutability and structural sharing) IndexMap type for the [`imbl`](https://docs.rs/imbl) crate.

# Benchmarks
<details>
<summary>
Expand raw data
</summary>

| bench                                         | time       |
| --------------------------------------------- | ---------- |
| indexmap_im_i64/lookup_100                    | 2.2171 µs |
| indexmap_im_i64/lookup_1000                   | 36.202 µs |
| indexmap_im_i64/lookup_5000                   | 351.47 µs |
| indexmap_im_i64/lookup_10000                  | 822.60 µs |
| indexmap_im_i64/lookup_one_100                | 41.020 ns  |
| indexmap_im_i64/lookup_one_1000               | 58.836 ns  |
| indexmap_im_i64/lookup_one_5000               | 77.556 ns  |
| indexmap_im_i64/lookup_one_10000              | 92.410 ns  |
| indexmap_im_i64/lookup_one_50000              | 148.94 ns  |
| indexmap_im_i64/lookup_one_100000             | 180.91 ns  |
| indexmap_im_i64/insert_100                    | 42.380 µs |
| indexmap_im_i64/insert_1000                   | 696.94 µs |
| indexmap_im_i64/insert_5000                   | 5.1512 ms  |
| indexmap_im_i64/insert_10000                  | 10.728 ms  |
| indexmap_im_i64/insert_one_100                | 312.90 ns  |
| indexmap_im_i64/insert_one_1000               | 625.93 ns  |
| indexmap_im_i64/insert_one_5000               | 720.41 ns  |
| indexmap_im_i64/insert_one_10000              | 911.57 ns  |
| indexmap_im_i64/insert_mut_100                | 7.4663 µs |
| indexmap_im_i64/insert_mut_1000               | 106.94 µs |
| indexmap_im_i64/insert_mut_5000               | 555.26 µs |
| indexmap_im_i64/insert_mut_10000              | 1.0898 ms  |
| indexmap_im_i64/reinsert_mut_one_100          | 145.14 ns  |
| indexmap_im_i64/reinsert_mut_one_1000         | 216.73 ns  |
| indexmap_im_i64/reinsert_mut_one_5000         | 232.90 ns  |
| indexmap_im_i64/reinsert_mut_one_10000        | 253.31 ns  |
| indexmap_im_i64/reinsert_mut_one_50000        | 422.88 ns  |
| indexmap_im_i64/reinsert_mut_one_100000       | 521.35 ns  |
| indexmap_im_i64/remove_100                    | 41.110 µs |
| indexmap_im_i64/remove_1000                   | 846.52 µs |
| indexmap_im_i64/remove_5000                   | 5.5550 ms  |
| indexmap_im_i64/remove_10000                  | 11.677 ms  |
| indexmap_im_i64/remove_mut_100                | 7.0278 µs |
| indexmap_im_i64/remove_mut_1000               | 99.424 µs |
| indexmap_im_i64/remove_mut_5000               | 709.92 µs |
| indexmap_im_i64/remove_mut_10000              | 1.4830 ms  |
| indexmap_im_i64/iter_100                      | 356.32 ns  |
| indexmap_im_i64/iter_1000                     | 3.1698 µs |
| indexmap_im_i64/iter_5000                     | 15.915 µs |
| indexmap_im_i64/iter_10000                    | 31.806 µs |
| indexmap_crate_i64/lookup_100                 | 1.6420 µs |
| indexmap_crate_i64/lookup_1000                | 18.837 µs |
| indexmap_crate_i64/lookup_5000                | 102.79 µs |
| indexmap_crate_i64/lookup_10000               | 228.51 µs |
| indexmap_crate_i64/lookup_one_100             | 17.735 ns  |
| indexmap_crate_i64/lookup_one_1000            | 18.126 ns  |
| indexmap_crate_i64/lookup_one_5000            | 20.097 ns  |
| indexmap_crate_i64/lookup_one_10000           | 24.624 ns  |
| indexmap_crate_i64/lookup_one_50000           | 46.911 ns  |
| indexmap_crate_i64/lookup_one_100000          | 53.734 ns  |
| indexmap_crate_i64/insert_100                 | 19.577 µs |
| indexmap_crate_i64/insert_1000                | 833.47 µs |
| indexmap_crate_i64/insert_one_100             | 202.35 ns  |
| indexmap_crate_i64/insert_one_1000            | 1.5547 µs |
| indexmap_crate_i64/insert_mut_100             | 3.7500 µs |
| indexmap_crate_i64/insert_mut_1000            | 35.593 µs |
| indexmap_crate_i64/insert_mut_5000            | 160.58 µs |
| indexmap_crate_i64/insert_mut_10000           | 327.58 µs |
| indexmap_crate_i64/reinsert_mut_one_100       | 188.44 ns  |
| indexmap_crate_i64/reinsert_mut_one_1000      | 901.06 ns  |
| indexmap_crate_i64/reinsert_mut_one_5000      | 5.3931 µs |
| indexmap_crate_i64/reinsert_mut_one_10000     | 12.663 µs |
| indexmap_crate_i64/reinsert_mut_one_50000     | 91.878 µs |
| indexmap_crate_i64/reinsert_mut_one_100000    | 229.46 µs |
| indexmap_crate_i64/remove_100                 | 19.978 µs |
| indexmap_crate_i64/remove_1000                | 1.6563 ms  |
| indexmap_crate_i64/remove_mut_100             | 6.6191 µs |
| indexmap_crate_i64/remove_mut_1000            | 446.30 µs |
| indexmap_crate_i64/iter_100                   | 27.907 ns  |
| indexmap_crate_i64/iter_1000                  | 236.26 ns  |
| indexmap_crate_i64/iter_5000                  | 1.1756 µs |
| indexmap_crate_i64/iter_10000                 | 2.3203 µs |
| hashmap_std_i64/lookup_100                    | 1.1682 µs |
| hashmap_std_i64/lookup_1000                   | 11.456 µs |
| hashmap_std_i64/lookup_5000                   | 60.143 µs |
| hashmap_std_i64/lookup_10000                  | 118.40 µs |
| hashmap_std_i64/lookup_one_100                | 14.053 ns  |
| hashmap_std_i64/lookup_one_1000               | 14.117 ns  |
| hashmap_std_i64/lookup_one_5000               | 14.707 ns  |
| hashmap_std_i64/lookup_one_10000              | 15.870 ns  |
| hashmap_std_i64/lookup_one_50000              | 23.690 ns  |
| hashmap_std_i64/lookup_one_100000             | 27.946 ns  |
| hashmap_std_i64/insert_100                    | 8.4958 µs |
| hashmap_std_i64/insert_1000                   | 300.90 µs |
| hashmap_std_i64/insert_one_100                | 80.599 ns  |
| hashmap_std_i64/insert_one_1000               | 806.92 ns  |
| hashmap_std_i64/insert_mut_100                | 3.5235 µs |
| hashmap_std_i64/insert_mut_1000               | 40.574 µs |
| hashmap_std_i64/insert_mut_5000               | 179.58 µs |
| hashmap_std_i64/insert_mut_10000              | 369.42 µs |
| hashmap_std_i64/reinsert_mut_one_100          | 43.710 ns  |
| hashmap_std_i64/reinsert_mut_one_1000         | 37.025 ns  |
| hashmap_std_i64/reinsert_mut_one_5000         | 38.963 ns  |
| hashmap_std_i64/reinsert_mut_one_10000        | 40.865 ns  |
| hashmap_std_i64/reinsert_mut_one_50000        | 55.443 ns  |
| hashmap_std_i64/reinsert_mut_one_100000       | 62.903 ns  |
| hashmap_std_i64/remove_100                    | 7.3617 µs |
| hashmap_std_i64/remove_1000                   | 843.37 µs |
| hashmap_std_i64/remove_mut_100                | 1.3146 µs |
| hashmap_std_i64/remove_mut_1000               | 13.847 µs |
| hashmap_std_i64/remove_mut_5000               | 74.163 µs |
| hashmap_std_i64/remove_mut_10000              | 159.96 µs |
| hashmap_std_i64/iter_100                      | 70.627 ns  |
| hashmap_std_i64/iter_1000                     | 712.67 ns  |
| hashmap_std_i64/iter_5000                     | 3.7449 µs |
| hashmap_std_i64/iter_10000                    | 8.9011 µs |
| hashmap_im_i64/lookup_100                     | 1.2479 µs |
| hashmap_im_i64/lookup_1000                    | 13.583 µs |
| hashmap_im_i64/lookup_5000                    | 73.628 µs |
| hashmap_im_i64/lookup_10000                   | 173.06 µs |
| hashmap_im_i64/lookup_one_100                 | 16.427 ns  |
| hashmap_im_i64/lookup_one_1000                | 22.375 ns  |
| hashmap_im_i64/lookup_one_5000                | 19.370 ns  |
| hashmap_im_i64/lookup_one_10000               | 20.728 ns  |
| hashmap_im_i64/lookup_one_50000               | 38.256 ns  |
| hashmap_im_i64/lookup_one_100000              | 46.291 ns  |
| hashmap_im_i64/insert_100                     | 25.902 µs |
| hashmap_im_i64/insert_1000                    | 377.55 µs |
| hashmap_im_i64/insert_5000                    | 2.8991 ms  |
| hashmap_im_i64/insert_10000                   | 5.9935 ms  |
| hashmap_im_i64/insert_one_100                 | 293.94 ns  |
| hashmap_im_i64/insert_one_1000                | 597.01 ns  |
| hashmap_im_i64/insert_one_5000                | 540.55 ns  |
| hashmap_im_i64/insert_one_10000               | 538.76 ns  |
| hashmap_im_i64/insert_mut_100                 | 3.4990 µs |
| hashmap_im_i64/insert_mut_1000                | 47.485 µs |
| hashmap_im_i64/insert_mut_5000                | 197.35 µs |
| hashmap_im_i64/insert_mut_10000               | 353.21 µs |
| hashmap_im_i64/reinsert_mut_one_100           | 54.240 ns  |
| hashmap_im_i64/reinsert_mut_one_1000          | 79.050 ns  |
| hashmap_im_i64/reinsert_mut_one_5000          | 66.983 ns  |
| hashmap_im_i64/reinsert_mut_one_10000         | 67.341 ns  |
| hashmap_im_i64/reinsert_mut_one_50000         | 127.94 ns  |
| hashmap_im_i64/reinsert_mut_one_100000        | 128.00 ns  |
| hashmap_im_i64/remove_100                     | 23.659 µs |
| hashmap_im_i64/remove_1000                    | 464.66 µs |
| hashmap_im_i64/remove_5000                    | 3.0635 ms  |
| hashmap_im_i64/remove_10000                   | 6.2295 ms  |
| hashmap_im_i64/remove_mut_100                 | 3.6345 µs |
| hashmap_im_i64/remove_mut_1000                | 43.986 µs |
| hashmap_im_i64/remove_mut_5000                | 245.16 µs |
| hashmap_im_i64/remove_mut_10000               | 437.05 µs |
| hashmap_im_i64/iter_100                       | 473.48 ns  |
| hashmap_im_i64/iter_1000                      | 5.1771 µs |
| hashmap_im_i64/iter_5000                      | 26.549 µs |
| hashmap_im_i64/iter_10000                     | 48.020 µs |
| ordmap_i64/lookup_100                         | 1.0600 µs |
| ordmap_i64/lookup_1000                        | 19.535 µs |
| ordmap_i64/lookup_5000                        | 144.08 µs |
| ordmap_i64/lookup_10000                       | 336.52 µs |
| ordmap_i64/lookup_one_100                     | 14.462 ns  |
| ordmap_i64/lookup_one_1000                    | 23.373 ns  |
| ordmap_i64/lookup_one_5000                    | 31.088 ns  |
| ordmap_i64/lookup_one_10000                   | 37.530 ns  |
| ordmap_i64/lookup_one_50000                   | 58.591 ns  |
| ordmap_i64/lookup_one_100000                  | 68.430 ns  |
| ordmap_i64/insert_100                         | 12.724 µs |
| ordmap_i64/insert_1000                        | 250.13 µs |
| ordmap_i64/insert_5000                        | 1.8464 ms  |
| ordmap_i64/insert_10000                       | 4.3690 ms  |
| ordmap_i64/insert_one_100                     | 157.97 ns  |
| ordmap_i64/insert_one_1000                    | 272.01 ns  |
| ordmap_i64/insert_one_5000                    | 394.37 ns  |
| ordmap_i64/insert_one_10000                   | 420.76 ns  |
| ordmap_i64/insert_mut_100                     | 3.1782 µs |
| ordmap_i64/insert_mut_1000                    | 45.724 µs |
| ordmap_i64/insert_mut_5000                    | 331.64 µs |
| ordmap_i64/insert_mut_10000                   | 725.69 µs |
| ordmap_i64/reinsert_mut_one_100               | 74.409 ns  |
| ordmap_i64/reinsert_mut_one_1000              | 102.85 ns  |
| ordmap_i64/reinsert_mut_one_5000              | 124.71 ns  |
| ordmap_i64/reinsert_mut_one_10000             | 136.92 ns  |
| ordmap_i64/reinsert_mut_one_50000             | 173.87 ns  |
| ordmap_i64/reinsert_mut_one_100000            | 189.81 ns  |
| ordmap_i64/remove_100                         | 13.116 µs |
| ordmap_i64/remove_1000                        | 274.00 µs |
| ordmap_i64/remove_5000                        | 1.9400 ms  |
| ordmap_i64/remove_10000                       | 4.6772 ms  |
| ordmap_i64/remove_mut_100                     | 2.9447 µs |
| ordmap_i64/remove_mut_1000                    | 45.059 µs |
| ordmap_i64/remove_mut_5000                    | 349.43 µs |
| ordmap_i64/remove_mut_10000                   | 776.89 µs |
| ordmap_i64/iter_100                           | 373.98 ns  |
| ordmap_i64/iter_1000                          | 3.4944 µs |
| ordmap_i64/iter_5000                          | 17.809 µs |
| ordmap_i64/iter_10000                         | 35.402 µs |
| indexmap_im_str/lookup_100                    | 4.2906 µs |
| indexmap_im_str/lookup_1000                   | 67.250 µs |
| indexmap_im_str/lookup_5000                   | 532.15 µs |
| indexmap_im_str/lookup_10000                  | 1.1960 ms  |
| indexmap_im_str/lookup_one_100                | 49.332 ns  |
| indexmap_im_str/lookup_one_1000               | 72.456 ns  |
| indexmap_im_str/lookup_one_5000               | 105.43 ns  |
| indexmap_im_str/lookup_one_10000              | 116.85 ns  |
| indexmap_im_str/lookup_one_50000              | 277.92 ns  |
| indexmap_im_str/lookup_one_100000             | 409.19 ns  |
| indexmap_im_str/insert_100                    | 196.12 µs |
| indexmap_im_str/insert_1000                   | 2.4051 ms  |
| indexmap_im_str/insert_5000                   | 13.130 ms  |
| indexmap_im_str/insert_10000                  | 27.994 ms  |
| indexmap_im_str/insert_one_100                | 1.5608 µs |
| indexmap_im_str/insert_one_1000               | 2.2796 µs |
| indexmap_im_str/insert_one_5000               | 2.2142 µs |
| indexmap_im_str/insert_one_10000              | 2.6012 µs |
| indexmap_im_str/insert_mut_100                | 22.195 µs |
| indexmap_im_str/insert_mut_1000               | 253.57 µs |
| indexmap_im_str/insert_mut_5000               | 1.3439 ms  |
| indexmap_im_str/insert_mut_10000              | 2.8051 ms  |
| indexmap_im_str/reinsert_mut_one_100          | 325.14 ns  |
| indexmap_im_str/reinsert_mut_one_1000         | 426.81 ns  |
| indexmap_im_str/reinsert_mut_one_5000         | 482.78 ns  |
| indexmap_im_str/reinsert_mut_one_10000        | 515.00 ns  |
| indexmap_im_str/reinsert_mut_one_50000        | 1.1780 µs |
| indexmap_im_str/reinsert_mut_one_100000       | 1.3900 µs |
| indexmap_im_str/remove_100                    | 155.20 µs |
| indexmap_im_str/remove_1000                   | 1.9469 ms  |
| indexmap_im_str/remove_5000                   | 11.355 ms  |
| indexmap_im_str/remove_10000                  | 24.762 ms  |
| indexmap_im_str/remove_mut_100                | 26.261 µs |
| indexmap_im_str/remove_mut_1000               | 316.29 µs |
| indexmap_im_str/remove_mut_5000               | 1.9002 ms  |
| indexmap_im_str/remove_mut_10000              | 4.0522 ms  |
| indexmap_im_str/iter_100                      | 373.05 ns  |
| indexmap_im_str/iter_1000                     | 3.3810 µs |
| indexmap_im_str/iter_5000                     | 17.290 µs |
| indexmap_im_str/iter_10000                    | 33.298 µs |
| indexmap_crate_str/lookup_100                 | 2.0439 µs |
| indexmap_crate_str/lookup_1000                | 24.478 µs |
| indexmap_crate_str/lookup_5000                | 209.45 µs |
| indexmap_crate_str/lookup_10000               | 487.99 µs |
| indexmap_crate_str/lookup_one_100             | 26.088 ns  |
| indexmap_crate_str/lookup_one_1000            | 28.909 ns  |
| indexmap_crate_str/lookup_one_5000            | 45.255 ns  |
| indexmap_crate_str/lookup_one_10000           | 54.772 ns  |
| indexmap_crate_str/lookup_one_50000           | 127.42 ns  |
| indexmap_crate_str/lookup_one_100000          | 168.36 ns  |
| indexmap_crate_str/insert_100                 | 371.48 µs |
| indexmap_crate_str/insert_1000                | 32.734 ms  |
| indexmap_crate_str/insert_one_100             | 7.8976 µs |
| indexmap_crate_str/insert_one_1000            | 78.581 µs |
| indexmap_crate_str/insert_mut_100             | 11.614 µs |
| indexmap_crate_str/insert_mut_1000            | 122.75 µs |
| indexmap_crate_str/insert_mut_5000            | 613.74 µs |
| indexmap_crate_str/insert_mut_10000           | 1.3460 ms  |
| indexmap_crate_str/reinsert_mut_one_100       | 280.08 ns  |
| indexmap_crate_str/reinsert_mut_one_1000      | 1.2445 µs |
| indexmap_crate_str/reinsert_mut_one_5000      | 6.7717 µs |
| indexmap_crate_str/reinsert_mut_one_10000     | 15.541 µs |
| indexmap_crate_str/reinsert_mut_one_50000     | 160.30 µs |
| indexmap_crate_str/reinsert_mut_one_100000    | 351.00 µs |
| indexmap_crate_str/remove_100                 | 398.91 µs |
| indexmap_crate_str/remove_1000                | 35.646 ms  |
| indexmap_crate_str/remove_mut_100             | 14.701 µs |
| indexmap_crate_str/remove_mut_1000            | 607.27 µs |
| indexmap_crate_str/iter_100                   | 45.005 ns  |
| indexmap_crate_str/iter_1000                  | 446.07 ns  |
| indexmap_crate_str/iter_5000                  | 2.2660 µs |
| indexmap_crate_str/iter_10000                 | 4.5314 µs |
| hashmap_std_str/lookup_100                    | 1.7991 µs |
| hashmap_std_str/lookup_1000                   | 19.244 µs |
| hashmap_std_str/lookup_5000                   | 152.79 µs |
| hashmap_std_str/lookup_10000                  | 343.90 µs |
| hashmap_std_str/lookup_one_100                | 23.213 ns  |
| hashmap_std_str/lookup_one_1000               | 25.795 ns  |
| hashmap_std_str/lookup_one_5000               | 46.694 ns  |
| hashmap_std_str/lookup_one_10000              | 51.470 ns  |
| hashmap_std_str/lookup_one_50000              | 79.630 ns  |
| hashmap_std_str/lookup_one_100000             | 199.56 ns  |
| hashmap_std_str/insert_100                    | 408.29 µs |
| hashmap_std_str/insert_1000                   | 38.068 ms  |
| hashmap_std_str/insert_one_100                | 8.0109 µs |
| hashmap_std_str/insert_one_1000               | 78.599 µs |
| hashmap_std_str/insert_mut_100                | 13.046 µs |
| hashmap_std_str/insert_mut_1000               | 156.99 µs |
| hashmap_std_str/insert_mut_5000               | 792.94 µs |
| hashmap_std_str/insert_mut_10000              | 2.0844 ms  |
| hashmap_std_str/reinsert_mut_one_100          | 102.47 ns  |
| hashmap_std_str/reinsert_mut_one_1000         | 101.76 ns  |
| hashmap_std_str/reinsert_mut_one_5000         | 131.88 ns  |
| hashmap_std_str/reinsert_mut_one_10000        | 126.48 ns  |
| hashmap_std_str/reinsert_mut_one_50000        | 340.05 ns  |
| hashmap_std_str/reinsert_mut_one_100000       | 371.47 ns  |
| hashmap_std_str/remove_100                    | 399.44 µs |
| hashmap_std_str/remove_1000                   | 37.626 ms  |
| hashmap_std_str/remove_mut_100                | 10.596 µs |
| hashmap_std_str/remove_mut_1000               | 105.66 µs |
| hashmap_std_str/remove_mut_5000               | 608.14 µs |
| hashmap_std_str/remove_mut_10000              | 1.3607 ms  |
| hashmap_std_str/iter_100                      | 98.367 ns  |
| hashmap_std_str/iter_1000                     | 1.0069 µs |
| hashmap_std_str/iter_5000                     | 5.4940 µs |
| hashmap_std_str/iter_10000                    | 12.348 µs |
| hashmap_im_str/lookup_100                     | 2.2419 µs |
| hashmap_im_str/lookup_1000                    | 31.219 µs |
| hashmap_im_str/lookup_5000                    | 218.35 µs |
| hashmap_im_str/lookup_10000                   | 511.94 µs |
| hashmap_im_str/lookup_one_100                 | 24.697 ns  |
| hashmap_im_str/lookup_one_1000                | 34.134 ns  |
| hashmap_im_str/lookup_one_5000                | 48.009 ns  |
| hashmap_im_str/lookup_one_10000               | 53.716 ns  |
| hashmap_im_str/lookup_one_50000               | 108.68 ns  |
| hashmap_im_str/lookup_one_100000              | 205.23 ns  |
| hashmap_im_str/insert_100                     | 114.73 µs |
| hashmap_im_str/insert_1000                    | 1.7635 ms  |
| hashmap_im_str/insert_5000                    | 8.7512 ms  |
| hashmap_im_str/insert_10000                   | 17.986 ms  |
| hashmap_im_str/insert_one_100                 | 1.1482 µs |
| hashmap_im_str/insert_one_1000                | 2.3543 µs |
| hashmap_im_str/insert_one_5000                | 1.7225 µs |
| hashmap_im_str/insert_one_10000               | 2.0179 µs |
| hashmap_im_str/insert_mut_100                 | 12.330 µs |
| hashmap_im_str/insert_mut_1000                | 148.37 µs |
| hashmap_im_str/insert_mut_5000                | 740.86 µs |
| hashmap_im_str/insert_mut_10000               | 1.4000 ms  |
| hashmap_im_str/reinsert_mut_one_100           | 129.20 ns  |
| hashmap_im_str/reinsert_mut_one_1000          | 157.79 ns  |
| hashmap_im_str/reinsert_mut_one_5000          | 175.89 ns  |
| hashmap_im_str/reinsert_mut_one_10000         | 181.79 ns  |
| hashmap_im_str/reinsert_mut_one_50000         | 422.89 ns  |
| hashmap_im_str/reinsert_mut_one_100000        | 544.64 ns  |
| hashmap_im_str/remove_100                     | 116.95 µs |
| hashmap_im_str/remove_1000                    | 1.5271 ms  |
| hashmap_im_str/remove_5000                    | 8.2255 ms  |
| hashmap_im_str/remove_10000                   | 17.010 ms  |
| hashmap_im_str/remove_mut_100                 | 15.352 µs |
| hashmap_im_str/remove_mut_1000                | 173.97 µs |
| hashmap_im_str/remove_mut_5000                | 975.90 µs |
| hashmap_im_str/remove_mut_10000               | 1.7802 ms  |
| hashmap_im_str/iter_100                       | 429.25 ns  |
| hashmap_im_str/iter_1000                      | 4.6459 µs |
| hashmap_im_str/iter_5000                      | 33.943 µs |
| hashmap_im_str/iter_10000                     | 55.191 µs |
| ordmap_str/lookup_100                         | 3.0671 µs |
| ordmap_str/lookup_1000                        | 74.624 µs |
| ordmap_str/lookup_5000                        | 631.07 µs |
| ordmap_str/lookup_10000                       | 1.5052 ms  |
| ordmap_str/lookup_one_100                     | 52.880 ns  |
| ordmap_str/lookup_one_1000                    | 86.761 ns  |
| ordmap_str/lookup_one_5000                    | 134.37 ns  |
| ordmap_str/lookup_one_10000                   | 162.04 ns  |
| ordmap_str/lookup_one_50000                   | 252.93 ns  |
| ordmap_str/lookup_one_100000                  | 391.94 ns  |
| ordmap_str/insert_100                         | 115.00 µs |
| ordmap_str/insert_1000                        | 1.7559 ms  |
| ordmap_str/insert_5000                        | 10.955 ms  |
| ordmap_str/insert_10000                       | 25.911 ms  |
| ordmap_str/insert_one_100                     | 1.6104 µs |
| ordmap_str/insert_one_1000                    | 2.2359 µs |
| ordmap_str/insert_one_5000                    | 2.9382 µs |
| ordmap_str/insert_one_10000                   | 3.0738 µs |
| ordmap_str/insert_mut_100                     | 13.111 µs |
| ordmap_str/insert_mut_1000                    | 189.28 µs |
| ordmap_str/insert_mut_5000                    | 1.2542 ms  |
| ordmap_str/insert_mut_10000                   | 2.8106 ms  |
| ordmap_str/reinsert_mut_one_100               | 168.47 ns  |
| ordmap_str/reinsert_mut_one_1000              | 266.07 ns  |
| ordmap_str/reinsert_mut_one_5000              | 360.05 ns  |
| ordmap_str/reinsert_mut_one_10000             | 401.13 ns  |
| ordmap_str/reinsert_mut_one_50000             | 578.33 ns  |
| ordmap_str/reinsert_mut_one_100000            | 799.62 ns  |
| ordmap_str/remove_100                         | 98.817 µs |
| ordmap_str/remove_1000                        | 1.4766 ms  |
| ordmap_str/remove_5000                        | 9.5179 ms  |
| ordmap_str/remove_10000                       | 21.614 ms  |
| ordmap_str/remove_mut_100                     | 15.428 µs |
| ordmap_str/remove_mut_1000                    | 217.93 µs |
| ordmap_str/remove_mut_5000                    | 1.4221 ms  |
| ordmap_str/remove_mut_10000                   | 3.1568 ms  |
| ordmap_str/iter_100                           | 362.36 ns  |
| ordmap_str/iter_1000                          | 3.3644 µs |
| ordmap_str/iter_5000                          | 17.022 µs |
| ordmap_str/iter_10000                         | 34.000 µs |
| indexmap_im_big/lookup_100                    | 82.191 µs |
| indexmap_im_big/lookup_1000                   | 822.32 µs |
| indexmap_im_big/lookup_5000                   | 4.1792 ms  |
| indexmap_im_big/lookup_10000                  | 8.3462 ms  |
| indexmap_im_big/lookup_one_100                | 807.03 ns  |
| indexmap_im_big/lookup_one_1000               | 825.16 ns  |
| indexmap_im_big/lookup_one_5000               | 812.10 ns  |
| indexmap_im_big/lookup_one_10000              | 808.05 ns  |
| indexmap_im_big/lookup_one_50000              | 810.77 ns  |
| indexmap_im_big/lookup_one_100000             | 821.06 ns  |
| indexmap_im_big/insert_100                    | 557.51 µs |
| indexmap_im_big/insert_1000                   | 7.1817 ms  |
| indexmap_im_big/insert_5000                   | 36.840 ms  |
| indexmap_im_big/insert_10000                  | 71.600 ms  |
| indexmap_im_big/insert_one_100                | 4.9734 µs |
| indexmap_im_big/insert_one_1000               | 4.6083 µs |
| indexmap_im_big/insert_one_5000               | 5.0561 µs |
| indexmap_im_big/insert_one_10000              | 4.7188 µs |
| indexmap_im_big/insert_mut_100                | 220.85 µs |
| indexmap_im_big/insert_mut_1000               | 5.3462 ms  |
| indexmap_im_big/insert_mut_5000               | 27.600 ms  |
| indexmap_im_big/insert_mut_10000              | 35.997 ms  |
| indexmap_im_big/reinsert_mut_one_100          | 3.2089 µs |
| indexmap_im_big/reinsert_mut_one_1000         | 3.3190 µs |
| indexmap_im_big/reinsert_mut_one_5000         | 3.1713 µs |
| indexmap_im_big/reinsert_mut_one_10000        | 3.2081 µs |
| indexmap_im_big/reinsert_mut_one_50000        | 3.1907 µs |
| indexmap_im_big/reinsert_mut_one_100000       | 3.2254 µs |
| indexmap_im_big/remove_100                    | 648.15 µs |
| indexmap_im_big/remove_1000                   | 6.2712 ms  |
| indexmap_im_big/remove_5000                   | 31.208 ms  |
| indexmap_im_big/remove_10000                  | 69.438 ms  |
| indexmap_im_big/remove_mut_100                | 84.506 µs |
| indexmap_im_big/remove_mut_1000               | 742.11 µs |
| indexmap_im_big/remove_mut_5000               | 3.6890 ms  |
| indexmap_im_big/remove_mut_10000              | 7.3721 ms  |
| indexmap_im_big/iter_100                      | 11.714 ns  |
| indexmap_im_big/iter_1000                     | 12.163 ns  |
| indexmap_im_big/iter_5000                     | 11.925 ns  |
| indexmap_im_big/iter_10000                    | 11.841 ns  |
| indexmap_crate_big/lookup_100                 | 11.415 µs |
| indexmap_crate_big/lookup_1000                | 117.76 µs |
| indexmap_crate_big/lookup_5000                | 914.51 µs |
| indexmap_crate_big/lookup_10000               | 1.9003 ms  |
| indexmap_crate_big/lookup_one_100             | 101.38 ns  |
| indexmap_crate_big/lookup_one_1000            | 103.02 ns  |
| indexmap_crate_big/lookup_one_5000            | 102.24 ns  |
| indexmap_crate_big/lookup_one_10000           | 101.81 ns  |
| indexmap_crate_big/lookup_one_50000           | 104.39 ns  |
| indexmap_crate_big/lookup_one_100000          | 102.66 ns  |
| indexmap_crate_big/insert_100                 | 193.64 µs |
| indexmap_crate_big/insert_1000                | 3.1541 ms  |
| indexmap_crate_big/insert_one_100             | 1.6482 µs |
| indexmap_crate_big/insert_one_1000            | 1.6422 µs |
| indexmap_crate_big/insert_mut_100             | 149.96 µs |
| indexmap_crate_big/insert_mut_1000            | 2.7409 ms  |
| indexmap_crate_big/insert_mut_5000            | 14.547 ms  |
| indexmap_crate_big/insert_mut_10000           | 29.845 ms  |
| indexmap_crate_big/reinsert_mut_one_100       | 1.5409 µs |
| indexmap_crate_big/reinsert_mut_one_1000      | 1.5453 µs |
| indexmap_crate_big/reinsert_mut_one_5000      | 1.5493 µs |
| indexmap_crate_big/reinsert_mut_one_10000     | 3.5970 µs |
| indexmap_crate_big/reinsert_mut_one_50000     | 3.5717 µs |
| indexmap_crate_big/reinsert_mut_one_100000    | 3.6026 µs |
| indexmap_crate_big/remove_100                 | 7.7669 µs |
| indexmap_crate_big/remove_1000                | 211.24 µs |
| indexmap_crate_big/remove_mut_100             | 2.2999 µs |
| indexmap_crate_big/remove_mut_1000            | 26.126 µs |
| indexmap_crate_big/iter_100                   | 461.93 ps  |
| indexmap_crate_big/iter_1000                  | 474.42 ps  |
| indexmap_crate_big/iter_5000                  | 469.16 ps  |
| indexmap_crate_big/iter_10000                 | 460.85 ps  |
| hashmap_std_big/lookup_100                    | 80.889 µs |
| hashmap_std_big/lookup_1000                   | 822.07 µs |
| hashmap_std_big/lookup_5000                   | 4.1294 ms  |
| hashmap_std_big/lookup_10000                  | 8.4620 ms  |
| hashmap_std_big/lookup_one_100                | 812.92 ns  |
| hashmap_std_big/lookup_one_1000               | 824.92 ns  |
| hashmap_std_big/lookup_one_5000               | 801.40 ns  |
| hashmap_std_big/lookup_one_10000              | 802.99 ns  |
| hashmap_std_big/lookup_one_50000              | 797.28 ns  |
| hashmap_std_big/lookup_one_100000             | 802.95 ns  |
| hashmap_std_big/insert_100                    | 207.97 µs |
| hashmap_std_big/insert_1000                   | 3.3396 ms  |
| hashmap_std_big/insert_one_100                | 3.3339 µs |
| hashmap_std_big/insert_one_1000               | 3.3747 µs |
| hashmap_std_big/insert_mut_100                | 151.82 µs |
| hashmap_std_big/insert_mut_1000               | 2.7925 ms  |
| hashmap_std_big/insert_mut_5000               | 14.811 ms  |
| hashmap_std_big/insert_mut_10000              | 30.241 ms  |
| hashmap_std_big/reinsert_mut_one_100          | 1.7825 µs |
| hashmap_std_big/reinsert_mut_one_1000         | 1.7680 µs |
| hashmap_std_big/reinsert_mut_one_5000         | 1.7614 µs |
| hashmap_std_big/reinsert_mut_one_10000        | 1.8350 µs |
| hashmap_std_big/reinsert_mut_one_50000        | 1.7826 µs |
| hashmap_std_big/reinsert_mut_one_100000       | 1.7476 µs |
| hashmap_std_big/remove_100                    | 914.70 µs |
| hashmap_std_big/remove_1000                   | 17.943 ms  |
| hashmap_std_big/remove_mut_100                | 86.366 µs |
| hashmap_std_big/remove_mut_1000               | 741.19 µs |
| hashmap_std_big/remove_mut_5000               | 3.6612 ms  |
| hashmap_std_big/remove_mut_10000              | 7.1725 ms  |
| hashmap_std_big/iter_100                      | 2.2972 ns  |
| hashmap_std_big/iter_1000                     | 31.814 ns  |
| hashmap_std_big/iter_5000                     | 110.15 ns  |
| hashmap_std_big/iter_10000                    | 259.02 ns  |
| hashmap_im_big/lookup_100                     | 80.320 µs |
| hashmap_im_big/lookup_1000                    | 805.17 µs |
| hashmap_im_big/lookup_5000                    | 4.0663 ms  |
| hashmap_im_big/lookup_10000                   | 8.2241 ms  |
| hashmap_im_big/lookup_one_100                 | 808.74 ns  |
| hashmap_im_big/lookup_one_1000                | 800.36 ns  |
| hashmap_im_big/lookup_one_5000                | 802.26 ns  |
| hashmap_im_big/lookup_one_10000               | 794.94 ns  |
| hashmap_im_big/lookup_one_50000               | 798.10 ns  |
| hashmap_im_big/lookup_one_100000              | 807.45 ns  |
| hashmap_im_big/insert_100                     | 1.4236 ms  |
| hashmap_im_big/insert_1000                    | 15.144 ms  |
| hashmap_im_big/insert_5000                    | 75.530 ms  |
| hashmap_im_big/insert_10000                   | 151.50 ms  |
| hashmap_im_big/insert_one_100                 | 13.270 µs |
| hashmap_im_big/insert_one_1000                | 13.604 µs |
| hashmap_im_big/insert_one_5000                | 13.219 µs |
| hashmap_im_big/insert_one_10000               | 13.317 µs |
| hashmap_im_big/insert_mut_100                 | 198.40 µs |
| hashmap_im_big/insert_mut_1000                | 2.9823 ms  |
| hashmap_im_big/insert_mut_5000                | 15.762 ms  |
| hashmap_im_big/insert_mut_10000               | 32.388 ms  |
| hashmap_im_big/reinsert_mut_one_100           | 1.9615 µs |
| hashmap_im_big/reinsert_mut_one_1000          | 1.9746 µs |
| hashmap_im_big/reinsert_mut_one_5000          | 1.9808 µs |
| hashmap_im_big/reinsert_mut_one_10000         | 1.9407 µs |
| hashmap_im_big/reinsert_mut_one_50000         | 1.9568 µs |
| hashmap_im_big/reinsert_mut_one_100000        | 4.2006 µs |
| hashmap_im_big/remove_100                     | 1.2273 ms  |
| hashmap_im_big/remove_1000                    | 11.796 ms  |
| hashmap_im_big/remove_5000                    | 62.555 ms  |
| hashmap_im_big/remove_10000                   | 124.49 ms  |
| hashmap_im_big/remove_mut_100                 | 85.629 µs |
| hashmap_im_big/remove_mut_1000                | 727.70 µs |
| hashmap_im_big/remove_mut_5000                | 3.6265 ms  |
| hashmap_im_big/remove_mut_10000               | 7.2270 ms  |
| hashmap_im_big/iter_100                       | 6.5006 ns  |
| hashmap_im_big/iter_1000                      | 6.5838 ns  |
| hashmap_im_big/iter_5000                      | 6.3988 ns  |
| hashmap_im_big/iter_10000                     | 6.1630 ns  |
| ordmap_big/lookup_100                         | 11.435 µs |
| ordmap_big/lookup_1000                        | 115.97 µs |
| ordmap_big/lookup_5000                        | 935.39 µs |
| ordmap_big/lookup_10000                       | 1.9507 ms  |
| ordmap_big/lookup_one_100                     | 103.34 ns  |
| ordmap_big/lookup_one_1000                    | 103.97 ns  |
| ordmap_big/lookup_one_5000                    | 103.37 ns  |
| ordmap_big/lookup_one_10000                   | 101.71 ns  |
| ordmap_big/lookup_one_50000                   | 102.92 ns  |
| ordmap_big/lookup_one_100000                  | 102.96 ns  |
| ordmap_big/insert_100                         | 431.96 µs |
| ordmap_big/insert_1000                        | 6.0183 ms  |
| ordmap_big/insert_5000                        | 30.325 ms  |
| ordmap_big/insert_10000                       | 64.232 ms  |
| ordmap_big/insert_one_100                     | 4.3298 µs |
| ordmap_big/insert_one_1000                    | 3.8997 µs |
| ordmap_big/insert_one_5000                    | 4.3498 µs |
| ordmap_big/insert_one_10000                   | 4.1184 µs |
| ordmap_big/insert_mut_100                     | 125.40 µs |
| ordmap_big/insert_mut_1000                    | 2.4052 ms  |
| ordmap_big/insert_mut_5000                    | 13.126 ms  |
| ordmap_big/insert_mut_10000                   | 26.922 ms  |
| ordmap_big/reinsert_mut_one_100               | 742.39 ns  |
| ordmap_big/reinsert_mut_one_1000              | 742.27 ns  |
| ordmap_big/reinsert_mut_one_5000              | 760.47 ns  |
| ordmap_big/reinsert_mut_one_10000             | 737.56 ns  |
| ordmap_big/reinsert_mut_one_50000             | 752.74 ns  |
| ordmap_big/reinsert_mut_one_100000            | 792.79 ns  |
| ordmap_big/remove_100                         | 324.07 µs |
| ordmap_big/remove_1000                        | 3.1751 ms  |
| ordmap_big/remove_5000                        | 15.539 ms  |
| ordmap_big/remove_10000                       | 33.316 ms  |
| ordmap_big/remove_mut_100                     | 22.699 µs |
| ordmap_big/remove_mut_1000                    | 190.76 µs |
| ordmap_big/remove_mut_5000                    | 968.45 µs |
| ordmap_big/remove_mut_10000                   | 1.8726 ms  |
| ordmap_big/iter_100                           | 13.277 ns  |
| ordmap_big/iter_1000                          | 13.181 ns  |
| ordmap_big/iter_5000                          | 13.075 ns  |
| ordmap_big/iter_10000                         | 12.882 ns  |
| indexmap_im_specific_i64/get_index_100        | 197.56 ns  |
| indexmap_im_specific_i64/first_100            | 1.0296 ns  |
| indexmap_im_specific_i64/last_100             | 1.6279 ns  |
| indexmap_im_specific_i64/get_index_1000       | 1.6329 µs |
| indexmap_im_specific_i64/first_1000           | 1.7731 ns  |
| indexmap_im_specific_i64/last_1000            | 2.5832 ns  |
| indexmap_im_specific_i64/get_index_5000       | 7.9645 µs |
| indexmap_im_specific_i64/first_5000           | 2.4099 ns  |
| indexmap_im_specific_i64/last_5000            | 3.8439 ns  |
| indexmap_im_specific_i64/get_index_10000      | 15.872 µs |
| indexmap_im_specific_i64/first_10000          | 2.4101 ns  |
| indexmap_im_specific_i64/last_10000           | 3.8213 ns  |
| indexmap_im_specific_i64/get_index_50000      | 85.359 µs |
| indexmap_im_specific_i64/first_50000          | 3.3255 ns  |
| indexmap_im_specific_i64/last_50000           | 5.0754 ns  |
| indexmap_im_specific_i64/get_index_100000     | 183.45 µs |
| indexmap_im_specific_i64/first_100000         | 3.3311 ns  |
| indexmap_im_specific_i64/last_100000          | 5.0534 ns  |
| indexmap_im_specific_i64/get_index_500000     | 1.1513 ms  |
| indexmap_im_specific_i64/first_500000         | 4.2679 ns  |
| indexmap_im_specific_i64/last_500000          | 6.5324 ns  |
| indexmap_im_specific_i64/get_index_1000000    | 3.4111 ms  |
| indexmap_im_specific_i64/first_1000000        | 5.3895 ns  |
| indexmap_im_specific_i64/last_1000000         | 8.3279 ns  |
| indexmap_crate_specific_i64/get_index_100     | 3.2253 ns  |
| indexmap_crate_specific_i64/first_100         | 229.99 ps  |
| indexmap_crate_specific_i64/last_100          | 230.66 ps  |
| indexmap_crate_specific_i64/get_index_1000    | 3.3279 ns  |
| indexmap_crate_specific_i64/first_1000        | 235.74 ps  |
| indexmap_crate_specific_i64/last_1000         | 237.04 ps  |
| indexmap_crate_specific_i64/get_index_5000    | 3.2000 ns  |
| indexmap_crate_specific_i64/first_5000        | 229.86 ps  |
| indexmap_crate_specific_i64/last_5000         | 229.22 ps  |
| indexmap_crate_specific_i64/get_index_10000   | 3.1915 ns  |
| indexmap_crate_specific_i64/first_10000       | 230.39 ps  |
| indexmap_crate_specific_i64/last_10000        | 229.60 ps  |
| indexmap_crate_specific_i64/get_index_50000   | 3.2352 ns  |
| indexmap_crate_specific_i64/first_50000       | 231.06 ps  |
| indexmap_crate_specific_i64/last_50000        | 229.45 ps  |
| indexmap_crate_specific_i64/get_index_100000  | 3.2378 ns  |
| indexmap_crate_specific_i64/first_100000      | 235.56 ps  |
| indexmap_crate_specific_i64/last_100000       | 233.66 ps  |
| indexmap_crate_specific_i64/get_index_500000  | 3.2539 ns  |
| indexmap_crate_specific_i64/first_500000      | 235.47 ps  |
| indexmap_crate_specific_i64/last_500000       | 236.24 ps  |
| indexmap_crate_specific_i64/get_index_1000000 | 3.2689 ns  |
| indexmap_crate_specific_i64/first_1000000     | 232.04 ps  |
| indexmap_crate_specific_i64/last_1000000      | 230.06 ps  |

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
    reinsert_mut_one: "(constant) Given a map of size N, time to clone, remove mutably, and reinsert a key to the map.\n\nAs indexmaps track insertion order, they are the only ones that ultimately produce a different (!=) result after the operation.",
    remove: "(linear) Given a map of size N, time to drain all its items by repeatedly .remove() (that clones) in a random order",
    remove_mut: "(linear) Given a map of size N, time to drain all its items by repeatedly calling .remove(), mutably, in a random order\n\nNote that `.drain()`ing the collection might be considerably faster. `indexmap_crate`'s big type appears much faster due to a highly optimized memcpy.",
    lookup: "(linear) Given a map of size N, time to lookup every one of its elements in a random order",
    lookup_one: "(constant) Given a map of size N, time to look up a single, random element",
    iter: "(linear) Given a map of size N, time to iterate all its items in the default order by reference\n\n`indexmap_crate` has a monstrously fast iteration due to backing its values in a linear array.",
    get_index: "imbl-index and OrdMap do not support constant-time access by index"
}

$bench_times
| insert timeval { |row|
    $row.time
    | str replace -r '([\d.]+) (\S+)' { |n, ty|
        let d = if $ty != 'ps' { ($n | into float) * 1000 } else { $n };
        $"-($d)(match $ty { ps => 'ns', s => 'sec', _ => $ty })"
    } | into duration
}
| each { |row| $row | merge ($row.bench | parse -r '(?<struct>.+)_(?<cat>(?<dtype>...)/(?<test>.+?)_(?<batch>\d+))' | first) }
| group-by test
| values
| where { let len = uniq-by struct | length; $len > 2 }
| sort-by { |rows| $rows.0.cat }
| each { |rows|
    let test = $rows.0.test

    let displayrows = $rows | insert grouper { |r| $"($r.batch)-($r.dtype)" }
    | group-by grouper
    | update cells { |rows|
        let fastest = $rows.timeval | math max;
        let batch = $rows.0.batch;
        let dtype = $rows.0.dtype;

        $rows | update time { |row|
            $"(
                $row.time | str replace µs us | str replace ' ' '' | str replace -r '(\d+\.\d{2})\d+' '$1'
            ) \((
                $row.timeval / $fastest | math round -p 1
            )x\)"
        }
        | select struct time
        | transpose --header-row
        | insert batch $batch
        | insert dtype $dtype
    }
    | values
    | flatten
    | select dtype batch hashmap_std? hashmap_im? ordmap? indexmap_im? indexmap_crate?
    | update cells { default - }

    let subtitle = if $test in $subtitles { $"($subtitles | get $test)\n\n" } else { "" }

    $"## ($test)\n($subtitle)($displayrows | to md --pretty)" | str replace -ar '(\d)us' '${1}µs'
}
| str join "\n\n"

# For specific
$bench_times
| insert timeval { |row|
    $row.time
    | str replace -r '([\d.]+) (\S+)' { |n, ty|
        let d = if $ty != 'ps' { ($n | into float) * 1000 } else { $n };
        $"-($d)(match $ty { ps => 'ns', s => 'sec', _ => $ty })"
    } | into duration
}
| each { |row| $row | merge ($row.bench | parse -r '(?<struct>.+)_(?<cat>(?<dtype>...)/(?<test>.+?)_(?<batch>\d+))' | first) }
| group-by test
| values
| where { let len = uniq-by struct | length; $len == 2 }
| sort-by { |rows| $rows.0.cat }
| each { |rows|
    let test = $rows.0.test

    let displayrows = $rows | insert grouper { |r| $"($r.batch)-($r.dtype)" }
    | group-by grouper
    | update cells { |rows|
        let fastest = $rows.timeval | math max;
        let batch = $rows.0.batch;
        let dtype = $rows.0.dtype;

        $rows | update time { |row|
            $"(
                $row.time | str replace µs us | str replace ' ' '' | str replace -r '(\d+\.\d{2})\d+' '$1'
            ) \((
                $row.timeval / $fastest | math round -p 1
            )x\)"
        }
        | select struct time
        | transpose --header-row
        | insert batch $batch
        | insert dtype $dtype
    }
    | values
    | flatten
    | select dtype batch indexmap_im_specific? indexmap_crate_specific?
    | update cells { default - }

    let subtitle = if $test in $subtitles { $"($subtitles | get $test)\n\n" } else { "" }

    $"### ($test)\n($subtitle)($displayrows | to md --pretty)" | str replace -ar '(\d)us' '${1}µs'
}
| str join "\n\n"
```
</details>

## insert
(linear) Time to create a map of size N by repeatedly inserting into it via immutable (cloned) updates

| dtype | batch | hashmap_std     | hashmap_im      | ordmap          | indexmap_im     | indexmap_crate  |
| ----- | ----- | --------------- | --------------- | --------------- | --------------- | --------------- |
| i64   | 100   | 8.49µs (1.0x)   | 25.90µs (3.0x)  | 12.72µs (1.5x)  | 42.38µs (5.0x)  | 19.57µs (2.3x)  |
| i64   | 1000  | 300.90µs (1.2x) | 377.55µs (1.5x) | 250.13µs (1.0x) | 696.94µs (2.8x) | 833.47µs (3.3x) |
| i64   | 5000  | -               | 2.89ms (1.6x)   | 1.84ms (1.0x)   | 5.15ms (2.8x)   | -               |
| i64   | 10000 | -               | 5.99ms (1.4x)   | 4.36ms (1.0x)   | 10.72ms (2.5x)  | -               |
| str   | 100   | 408.29µs (3.6x) | 114.73µs (1.0x) | 115.00µs (1.0x) | 196.12µs (1.7x) | 371.48µs (3.2x) |
| str   | 1000  | 38.06ms (21.7x) | 1.76ms (1.0x)   | 1.75ms (1.0x)   | 2.40ms (1.4x)   | 32.73ms (18.6x) |
| str   | 5000  | -               | 8.75ms (1.0x)   | 10.95ms (1.3x)  | 13.13ms (1.5x)  | -               |
| str   | 10000 | -               | 17.98ms (1.0x)  | 25.91ms (1.4x)  | 27.99ms (1.6x)  | -               |
| big   | 100   | 207.97µs (1.1x) | 1.42ms (7.4x)   | 431.96µs (2.2x) | 557.51µs (2.9x) | 193.64µs (1.0x) |
| big   | 1000  | 3.33ms (1.1x)   | 15.14ms (4.8x)  | 6.01ms (1.9x)   | 7.18ms (2.3x)   | 3.15ms (1.0x)   |
| big   | 5000  | -               | 75.53ms (2.5x)  | 30.32ms (1.0x)  | 36.84ms (1.2x)  | -               |
| big   | 10000 | -               | 151.50ms (2.4x) | 64.23ms (1.0x)  | 71.60ms (1.1x)  | -               |

## insert_mut
(linear) Time to create a map of size N by repeatedly inserting into it via mutable inserts

| dtype | batch | hashmap_std     | hashmap_im      | ordmap          | indexmap_im     | indexmap_crate  |
| ----- | ----- | --------------- | --------------- | --------------- | --------------- | --------------- |
| i64   | 100   | 3.52µs (1.1x)   | 3.49µs (1.1x)   | 3.17µs (1.0x)   | 7.46µs (2.3x)   | 3.75µs (1.2x)   |
| i64   | 1000  | 40.57µs (1.1x)  | 47.48µs (1.3x)  | 45.72µs (1.3x)  | 106.94µs (3.0x) | 35.59µs (1.0x)  |
| i64   | 5000  | 179.58µs (1.1x) | 197.35µs (1.2x) | 331.64µs (2.1x) | 555.26µs (3.5x) | 160.58µs (1.0x) |
| i64   | 10000 | 369.42µs (1.1x) | 353.21µs (1.1x) | 725.69µs (2.2x) | 1.08ms (3.3x)   | 327.58µs (1.0x) |
| str   | 100   | 13.04µs (1.1x)  | 12.33µs (1.1x)  | 13.11µs (1.1x)  | 22.19µs (1.9x)  | 11.61µs (1.0x)  |
| str   | 1000  | 156.99µs (1.3x) | 148.37µs (1.2x) | 189.28µs (1.5x) | 253.57µs (2.1x) | 122.75µs (1.0x) |
| str   | 5000  | 792.94µs (1.3x) | 740.86µs (1.2x) | 1.25ms (2.0x)   | 1.34ms (2.2x)   | 613.74µs (1.0x) |
| str   | 10000 | 2.08ms (1.5x)   | 1.40ms (1.0x)   | 2.81ms (2.1x)   | 2.80ms (2.1x)   | 1.34ms (1.0x)   |
| big   | 100   | 151.82µs (1.2x) | 198.40µs (1.6x) | 125.40µs (1.0x) | 220.85µs (1.8x) | 149.96µs (1.2x) |
| big   | 1000  | 2.79ms (1.2x)   | 2.98ms (1.2x)   | 2.40ms (1.0x)   | 5.34ms (2.2x)   | 2.74ms (1.1x)   |
| big   | 5000  | 14.81ms (1.1x)  | 15.76ms (1.2x)  | 13.12ms (1.0x)  | 27.60ms (2.1x)  | 14.54ms (1.1x)  |
| big   | 10000 | 30.24ms (1.1x)  | 32.38ms (1.2x)  | 26.92ms (1.0x)  | 35.99ms (1.3x)  | 29.84ms (1.1x)  |

## insert_one
(constant) Given a map of size N, time to insert one element immutably (cloning the map before inserting)

| dtype | batch | hashmap_std     | hashmap_im      | ordmap          | indexmap_im     | indexmap_crate  |
| ----- | ----- | --------------- | --------------- | --------------- | --------------- | --------------- |
| i64   | 100   | 80.59ns (1.0x)  | 293.94ns (3.6x) | 157.97ns (2.0x) | 312.90ns (3.9x) | 202.35ns (2.5x) |
| i64   | 1000  | 806.92ns (3.0x) | 597.01ns (2.2x) | 272.01ns (1.0x) | 625.93ns (2.3x) | 1.55µs (5.7x)   |
| i64   | 5000  | -               | 540.55ns (1.4x) | 394.37ns (1.0x) | 720.41ns (1.8x) | -               |
| i64   | 10000 | -               | 538.76ns (1.3x) | 420.76ns (1.0x) | 911.57ns (2.2x) | -               |
| str   | 100   | 8.01µs (7.0x)   | 1.14µs (1.0x)   | 1.61µs (1.4x)   | 1.56µs (1.4x)   | 7.89µs (6.9x)   |
| str   | 1000  | 78.59µs (35.2x) | 2.35µs (1.1x)   | 2.23µs (1.0x)   | 2.27µs (1.0x)   | 78.58µs (35.1x) |
| str   | 5000  | -               | 1.72µs (1.0x)   | 2.93µs (1.7x)   | 2.21µs (1.3x)   | -               |
| str   | 10000 | -               | 2.01µs (1.0x)   | 3.07µs (1.5x)   | 2.60µs (1.3x)   | -               |
| big   | 100   | 3.33µs (2.0x)   | 13.27µs (8.1x)  | 4.32µs (2.6x)   | 4.97µs (3.0x)   | 1.64µs (1.0x)   |
| big   | 1000  | 3.37µs (2.1x)   | 13.60µs (8.3x)  | 3.89µs (2.4x)   | 4.60µs (2.8x)   | 1.64µs (1.0x)   |
| big   | 5000  | -               | 13.21µs (3.0x)  | 4.34µs (1.0x)   | 5.05µs (1.2x)   | -               |
| big   | 10000 | -               | 13.31µs (3.2x)  | 4.11µs (1.0x)   | 4.71µs (1.1x)   | -               |

## iter
(linear) Given a map of size N, time to iterate all its items in the default order by reference

`indexmap_crate` has a monstrously fast iteration due to backing its values in a linear array.

| dtype | batch | hashmap_std       | hashmap_im       | ordmap           | indexmap_im      | indexmap_crate  |
| ----- | ----- | ----------------- | ---------------- | ---------------- | ---------------- | --------------- |
| i64   | 100   | 70.62ns (2.5x)    | 473.48ns (17.0x) | 373.98ns (13.4x) | 356.32ns (12.8x) | 27.90ns (1.0x)  |
| i64   | 1000  | 712.67ns (3.0x)   | 5.17µs (21.9x)   | 3.49µs (14.8x)   | 3.16µs (13.4x)   | 236.26ns (1.0x) |
| i64   | 5000  | 3.74µs (3.2x)     | 26.54µs (22.6x)  | 17.80µs (15.1x)  | 15.91µs (13.5x)  | 1.17µs (1.0x)   |
| i64   | 10000 | 8.90µs (3.8x)     | 48.02µs (20.7x)  | 35.40µs (15.3x)  | 31.80µs (13.7x)  | 2.32µs (1.0x)   |
| str   | 100   | 98.36ns (2.2x)    | 429.25ns (9.5x)  | 362.36ns (8.1x)  | 373.05ns (8.3x)  | 45.00ns (1.0x)  |
| str   | 1000  | 1.00µs (2.3x)     | 4.64µs (10.4x)   | 3.36µs (7.5x)    | 3.38µs (7.6x)    | 446.07ns (1.0x) |
| str   | 5000  | 5.49µs (2.4x)     | 33.94µs (15.0x)  | 17.02µs (7.5x)   | 17.29µs (7.6x)   | 2.26µs (1.0x)   |
| str   | 10000 | 12.34µs (2.7x)    | 55.19µs (12.2x)  | 34.00µs (7.5x)   | 33.29µs (7.3x)   | 4.53µs (1.0x)   |
| big   | 100   | 2.29ns (5.0x)     | 6.50ns (14.1x)   | 13.27ns (28.8x)  | 11.71ns (25.4x)  | 461.93ps (1.0x) |
| big   | 1000  | 31.81ns (67.1x)   | 6.58ns (13.9x)   | 13.18ns (27.8x)  | 12.16ns (25.7x)  | 474.42ps (1.0x) |
| big   | 5000  | 110.15ns (234.9x) | 6.39ns (13.6x)   | 13.07ns (27.9x)  | 11.92ns (25.4x)  | 469.16ps (1.0x) |
| big   | 10000 | 259.02ns (563.1x) | 6.16ns (13.4x)   | 12.88ns (28.0x)  | 11.84ns (25.7x)  | 460.85ps (1.0x) |

## lookup
(linear) Given a map of size N, time to lookup every one of its elements in a random order

| dtype | batch | hashmap_std     | hashmap_im      | ordmap          | indexmap_im     | indexmap_crate  |
| ----- | ----- | --------------- | --------------- | --------------- | --------------- | --------------- |
| i64   | 100   | 1.16µs (1.1x)   | 1.24µs (1.2x)   | 1.06µs (1.0x)   | 2.21µs (2.1x)   | 1.64µs (1.5x)   |
| i64   | 1000  | 11.45µs (1.0x)  | 13.58µs (1.2x)  | 19.53µs (1.7x)  | 36.20µs (3.2x)  | 18.83µs (1.6x)  |
| i64   | 5000  | 60.14µs (1.0x)  | 73.62µs (1.2x)  | 144.08µs (2.4x) | 351.47µs (5.8x) | 102.79µs (1.7x) |
| i64   | 10000 | 118.40µs (1.0x) | 173.06µs (1.5x) | 336.52µs (2.8x) | 822.60µs (6.9x) | 228.51µs (1.9x) |
| str   | 100   | 1.79µs (1.0x)   | 2.24µs (1.2x)   | 3.06µs (1.7x)   | 4.29µs (2.4x)   | 2.04µs (1.1x)   |
| str   | 1000  | 19.24µs (1.0x)  | 31.21µs (1.6x)  | 74.62µs (3.9x)  | 67.25µs (3.5x)  | 24.47µs (1.3x)  |
| str   | 5000  | 152.79µs (1.0x) | 218.35µs (1.4x) | 631.07µs (4.1x) | 532.15µs (3.5x) | 209.45µs (1.4x) |
| str   | 10000 | 343.90µs (1.0x) | 511.94µs (1.5x) | 1.50ms (4.4x)   | 1.19ms (3.5x)   | 487.99µs (1.4x) |
| big   | 100   | 80.88µs (7.1x)  | 80.32µs (7.0x)  | 11.43µs (1.0x)  | 82.19µs (7.2x)  | 11.41µs (1.0x)  |
| big   | 1000  | 822.07µs (7.1x) | 805.17µs (6.9x) | 115.97µs (1.0x) | 822.32µs (7.1x) | 117.76µs (1.0x) |
| big   | 5000  | 4.12ms (4.5x)   | 4.06ms (4.4x)   | 935.39µs (1.0x) | 4.17ms (4.6x)   | 914.51µs (1.0x) |
| big   | 10000 | 8.46ms (4.5x)   | 8.22ms (4.3x)   | 1.95ms (1.0x)   | 8.34ms (4.4x)   | 1.90ms (1.0x)   |

## lookup_one
(constant) Given a map of size N, time to look up a single, random element

| dtype | batch  | hashmap_std     | hashmap_im      | ordmap          | indexmap_im     | indexmap_crate  |
| ----- | ------ | --------------- | --------------- | --------------- | --------------- | --------------- |
| i64   | 100    | 14.05ns (1.0x)  | 16.42ns (1.2x)  | 14.46ns (1.0x)  | 41.02ns (2.9x)  | 17.73ns (1.3x)  |
| i64   | 1000   | 14.11ns (1.0x)  | 22.37ns (1.6x)  | 23.37ns (1.7x)  | 58.83ns (4.2x)  | 18.12ns (1.3x)  |
| i64   | 5000   | 14.70ns (1.0x)  | 19.37ns (1.3x)  | 31.08ns (2.1x)  | 77.55ns (5.3x)  | 20.09ns (1.4x)  |
| i64   | 10000  | 15.87ns (1.0x)  | 20.72ns (1.3x)  | 37.53ns (2.4x)  | 92.41ns (5.8x)  | 24.62ns (1.6x)  |
| i64   | 50000  | 23.69ns (1.0x)  | 38.25ns (1.6x)  | 58.59ns (2.5x)  | 148.94ns (6.3x) | 46.91ns (2.0x)  |
| i64   | 100000 | 27.94ns (1.0x)  | 46.29ns (1.7x)  | 68.43ns (2.4x)  | 180.91ns (6.5x) | 53.73ns (1.9x)  |
| str   | 100    | 23.21ns (1.0x)  | 24.69ns (1.1x)  | 52.88ns (2.3x)  | 49.33ns (2.1x)  | 26.08ns (1.1x)  |
| str   | 1000   | 25.79ns (1.0x)  | 34.13ns (1.3x)  | 86.76ns (3.4x)  | 72.45ns (2.8x)  | 28.90ns (1.1x)  |
| str   | 5000   | 46.69ns (1.0x)  | 48.00ns (1.1x)  | 134.37ns (3.0x) | 105.43ns (2.3x) | 45.25ns (1.0x)  |
| str   | 10000  | 51.47ns (1.0x)  | 53.71ns (1.0x)  | 162.04ns (3.1x) | 116.85ns (2.3x) | 54.77ns (1.1x)  |
| str   | 50000  | 79.63ns (1.0x)  | 108.68ns (1.4x) | 252.93ns (3.2x) | 277.92ns (3.5x) | 127.42ns (1.6x) |
| str   | 100000 | 199.56ns (1.2x) | 205.23ns (1.2x) | 391.94ns (2.3x) | 409.19ns (2.4x) | 168.36ns (1.0x) |
| big   | 100    | 812.92ns (8.0x) | 808.74ns (8.0x) | 103.34ns (1.0x) | 807.03ns (8.0x) | 101.38ns (1.0x) |
| big   | 1000   | 824.92ns (8.0x) | 800.36ns (7.8x) | 103.97ns (1.0x) | 825.16ns (8.0x) | 103.02ns (1.0x) |
| big   | 5000   | 801.40ns (7.8x) | 802.26ns (7.8x) | 103.37ns (1.0x) | 812.10ns (7.9x) | 102.24ns (1.0x) |
| big   | 10000  | 802.99ns (7.9x) | 794.94ns (7.8x) | 101.71ns (1.0x) | 808.05ns (7.9x) | 101.81ns (1.0x) |
| big   | 50000  | 797.28ns (7.7x) | 798.10ns (7.8x) | 102.92ns (1.0x) | 810.77ns (7.9x) | 104.39ns (1.0x) |
| big   | 100000 | 802.95ns (7.8x) | 807.45ns (7.9x) | 102.96ns (1.0x) | 821.06ns (8.0x) | 102.66ns (1.0x) |

## reinsert_mut_one
(constant) Given a map of size N, time to clone, remove mutably, and reinsert a key to the map.

As indexmaps track insertion order, they are the only ones that ultimately produce a different (!=) result after the operation.

| dtype | batch  | hashmap_std     | hashmap_im      | ordmap          | indexmap_im     | indexmap_crate     |
| ----- | ------ | --------------- | --------------- | --------------- | --------------- | ------------------ |
| i64   | 100    | 43.71ns (1.0x)  | 54.24ns (1.2x)  | 74.40ns (1.7x)  | 145.14ns (3.3x) | 188.44ns (4.3x)    |
| i64   | 1000   | 37.02ns (1.0x)  | 79.05ns (2.1x)  | 102.85ns (2.8x) | 216.73ns (5.9x) | 901.06ns (24.3x)   |
| i64   | 5000   | 38.96ns (1.0x)  | 66.98ns (1.7x)  | 124.71ns (3.2x) | 232.90ns (6.0x) | 5.39µs (138.4x)    |
| i64   | 10000  | 40.86ns (1.0x)  | 67.34ns (1.6x)  | 136.92ns (3.4x) | 253.31ns (6.2x) | 12.66µs (309.9x)   |
| i64   | 50000  | 55.44ns (1.0x)  | 127.94ns (2.3x) | 173.87ns (3.1x) | 422.88ns (7.6x) | 91.87µs (1657.2x)  |
| i64   | 100000 | 62.90ns (1.0x)  | 128.00ns (2.0x) | 189.81ns (3.0x) | 521.35ns (8.3x) | 229.46µs (3647.8x) |
| str   | 100    | 102.47ns (1.0x) | 129.20ns (1.3x) | 168.47ns (1.6x) | 325.14ns (3.2x) | 280.08ns (2.7x)    |
| str   | 1000   | 101.76ns (1.0x) | 157.79ns (1.6x) | 266.07ns (2.6x) | 426.81ns (4.2x) | 1.24µs (12.2x)     |
| str   | 5000   | 131.88ns (1.0x) | 175.89ns (1.3x) | 360.05ns (2.7x) | 482.78ns (3.7x) | 6.77µs (51.3x)     |
| str   | 10000  | 126.48ns (1.0x) | 181.79ns (1.4x) | 401.13ns (3.2x) | 515.00ns (4.1x) | 15.54µs (122.9x)   |
| str   | 50000  | 340.05ns (1.0x) | 422.89ns (1.2x) | 578.33ns (1.7x) | 1.17µs (3.5x)   | 160.30µs (471.4x)  |
| str   | 100000 | 371.47ns (1.0x) | 544.64ns (1.5x) | 799.62ns (2.2x) | 1.39µs (3.7x)   | 351.00µs (944.9x)  |
| big   | 100    | 1.78µs (2.4x)   | 1.96µs (2.6x)   | 742.39ns (1.0x) | 3.20µs (4.3x)   | 1.54µs (2.1x)      |
| big   | 1000   | 1.76µs (2.4x)   | 1.97µs (2.7x)   | 742.27ns (1.0x) | 3.31µs (4.5x)   | 1.54µs (2.1x)      |
| big   | 5000   | 1.76µs (2.3x)   | 1.98µs (2.6x)   | 760.47ns (1.0x) | 3.17µs (4.2x)   | 1.54µs (2.0x)      |
| big   | 10000  | 1.83µs (2.5x)   | 1.94µs (2.6x)   | 737.56ns (1.0x) | 3.20µs (4.3x)   | 3.59µs (4.9x)      |
| big   | 50000  | 1.78µs (2.4x)   | 1.95µs (2.6x)   | 752.74ns (1.0x) | 3.19µs (4.2x)   | 3.57µs (4.7x)      |
| big   | 100000 | 1.74µs (2.2x)   | 4.20µs (5.3x)   | 792.79ns (1.0x) | 3.22µs (4.1x)   | 3.60µs (4.5x)      |

## remove
(linear) Given a map of size N, time to drain all its items by repeatedly .remove() (that clones) in a random order

| dtype | batch | hashmap_std       | hashmap_im      | ordmap           | indexmap_im      | indexmap_crate  |
| ----- | ----- | ----------------- | --------------- | ---------------- | ---------------- | --------------- |
| i64   | 100   | 7.36µs (1.0x)     | 23.65µs (3.2x)  | 13.11µs (1.8x)   | 41.11µs (5.6x)   | 19.97µs (2.7x)  |
| i64   | 1000  | 843.37µs (3.1x)   | 464.66µs (1.7x) | 274.00µs (1.0x)  | 846.52µs (3.1x)  | 1.65ms (6.0x)   |
| i64   | 5000  | -                 | 3.06ms (1.6x)   | 1.94ms (1.0x)    | 5.55ms (2.9x)    | -               |
| i64   | 10000 | -                 | 6.22ms (1.3x)   | 4.67ms (1.0x)    | 11.67ms (2.5x)   | -               |
| str   | 100   | 399.44µs (4.0x)   | 116.95µs (1.2x) | 98.81µs (1.0x)   | 155.20µs (1.6x)  | 398.91µs (4.0x) |
| str   | 1000  | 37.62ms (25.5x)   | 1.52ms (1.0x)   | 1.47ms (1.0x)    | 1.94ms (1.3x)    | 35.64ms (24.1x) |
| str   | 5000  | -                 | 8.22ms (1.0x)   | 9.51ms (1.2x)    | 11.35ms (1.4x)   | -               |
| str   | 10000 | -                 | 17.01ms (1.0x)  | 21.61ms (1.3x)   | 24.76ms (1.5x)   | -               |
| big   | 100   | 914.70µs (117.8x) | 1.22ms (158.0x) | 324.07µs (41.7x) | 648.15µs (83.5x) | 7.76µs (1.0x)   |
| big   | 1000  | 17.94ms (84.9x)   | 11.79ms (55.8x) | 3.17ms (15.0x)   | 6.27ms (29.7x)   | 211.24µs (1.0x) |
| big   | 5000  | -                 | 62.55ms (4.0x)  | 15.53ms (1.0x)   | 31.20ms (2.0x)   | -               |
| big   | 10000 | -                 | 124.49ms (3.7x) | 33.31ms (1.0x)   | 69.43ms (2.1x)   | -               |

## remove_mut
(linear) Given a map of size N, time to drain all its items by repeatedly calling .remove(), mutably, in a random order

Note that `.drain()`ing the collection might be considerably faster. `indexmap_crate`'s big type appears much faster due to a highly optimized memcpy.

| dtype | batch | hashmap_std      | hashmap_im       | ordmap          | indexmap_im      | indexmap_crate   |
| ----- | ----- | ---------------- | ---------------- | --------------- | ---------------- | ---------------- |
| i64   | 100   | 1.31µs (1.0x)    | 3.63µs (2.8x)    | 2.94µs (2.2x)   | 7.02µs (5.3x)    | 6.61µs (5.0x)    |
| i64   | 1000  | 13.84µs (1.0x)   | 43.98µs (3.2x)   | 45.05µs (3.3x)  | 99.42µs (7.2x)   | 446.30µs (32.2x) |
| i64   | 5000  | 74.16µs (1.0x)   | 245.16µs (3.3x)  | 349.43µs (4.7x) | 709.92µs (9.6x)  | -                |
| i64   | 10000 | 159.96µs (1.0x)  | 437.05µs (2.7x)  | 776.89µs (4.9x) | 1.48ms (9.3x)    | -                |
| str   | 100   | 10.59µs (1.0x)   | 15.35µs (1.4x)   | 15.42µs (1.5x)  | 26.26µs (2.5x)   | 14.70µs (1.4x)   |
| str   | 1000  | 105.66µs (1.0x)  | 173.97µs (1.6x)  | 217.93µs (2.1x) | 316.29µs (3.0x)  | 607.27µs (5.7x)  |
| str   | 5000  | 608.14µs (1.0x)  | 975.90µs (1.6x)  | 1.42ms (2.3x)   | 1.90ms (3.1x)    | -                |
| str   | 10000 | 1.36ms (1.0x)    | 1.78ms (1.3x)    | 3.15ms (2.3x)   | 4.05ms (3.0x)    | -                |
| big   | 100   | 86.36µs (37.6x)  | 85.62µs (37.2x)  | 22.69µs (9.9x)  | 84.50µs (36.7x)  | 2.29µs (1.0x)    |
| big   | 1000  | 741.19µs (28.4x) | 727.70µs (27.9x) | 190.76µs (7.3x) | 742.11µs (28.4x) | 26.12µs (1.0x)   |
| big   | 5000  | 3.66ms (3.8x)    | 3.62ms (3.7x)    | 968.45µs (1.0x) | 3.68ms (3.8x)    | -                |
| big   | 10000 | 7.17ms (3.8x)    | 7.22ms (3.9x)    | 1.87ms (1.0x)   | 7.37ms (3.9x)    | -                |

## IndexMap-specific
### first
| dtype | batch   | indexmap_im_specific | indexmap_crate_specific |
| ----- | ------- | -------------------- | ----------------------- |
| i64   | 100     | 1.02ns (4.5x)        | 229.99ps (1.0x)         |
| i64   | 1000    | 1.77ns (7.5x)        | 235.74ps (1.0x)         |
| i64   | 5000    | 2.40ns (10.5x)       | 229.86ps (1.0x)         |
| i64   | 10000   | 2.41ns (10.5x)       | 230.39ps (1.0x)         |
| i64   | 50000   | 3.32ns (14.4x)       | 231.06ps (1.0x)         |
| i64   | 100000  | 3.33ns (14.2x)       | 235.56ps (1.0x)         |
| i64   | 500000  | 4.26ns (18.2x)       | 235.47ps (1.0x)         |
| i64   | 1000000 | 5.38ns (23.2x)       | 232.04ps (1.0x)         |

### get_index
imbl-index and OrdMap do not support constant-time access by index

| dtype | batch   | indexmap_im_specific | indexmap_crate_specific |
| ----- | ------- | -------------------- | ----------------------- |
| i64   | 100     | 197.56ns (61.3x)     | 3.22ns (1.0x)           |
| i64   | 1000    | 1.63µs (490.8x)      | 3.32ns (1.0x)           |
| i64   | 5000    | 7.96µs (2488.9x)     | 3.20ns (1.0x)           |
| i64   | 10000   | 15.87µs (4974.0x)    | 3.19ns (1.0x)           |
| i64   | 50000   | 85.35µs (26386.1x)   | 3.23ns (1.0x)           |
| i64   | 100000  | 183.45µs (56672.8x)  | 3.23ns (1.0x)           |
| i64   | 500000  | 1.15ms (353919.5x)   | 3.25ns (1.0x)           |
| i64   | 1000000 | 3.41ms (1043788.2x)  | 3.26ns (1.0x)           |

### last
| dtype | batch   | indexmap_im_specific | indexmap_crate_specific |
| ----- | ------- | -------------------- | ----------------------- |
| i64   | 100     | 1.62ns (7.1x)        | 230.66ps (1.0x)         |
| i64   | 1000    | 2.58ns (10.9x)       | 237.04ps (1.0x)         |
| i64   | 5000    | 3.84ns (16.8x)       | 229.22ps (1.0x)         |
| i64   | 10000   | 3.82ns (16.7x)       | 229.60ps (1.0x)         |
| i64   | 50000   | 5.07ns (22.2x)       | 229.45ps (1.0x)         |
| i64   | 100000  | 5.05ns (21.7x)       | 233.66ps (1.0x)         |
| i64   | 500000  | 6.53ns (27.7x)       | 236.24ps (1.0x)         |
| i64   | 1000000 | 8.32ns (36.2x)       | 230.06ps (1.0x)         |

## Takeaways
- Immutable IndexMap offers worst-in-class performance across the immutable structures, but not by an outrageous amount. It only really beats out the standard library's HashMap on clone ops (remove, insert)
- It also performs poorly (O(n)) on get_index due to the OrdMap backing; it's implemented as .iter().nth()
- The exact numbers above should be taken with a grain of salt due to the low sample count
- Should I have included the `indexmap` crate in the benchmarks? Probably
- Was it worth it to include the `big` datatype (a 4kb stack-allocated array) in the benchmarks? Not really, all it did was make the 10k insert stdlib benchmark take 20min for the 10 samples
