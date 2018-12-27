# walkbench

A tiny throwaway benchmarking tool to quickly compare Rust's [`ignore`][ignore]
and [`walkdir`][walkdir] crates with Ruby's native [`Find`][find] and my
[`fast_find`][fast_find] gem.

My use-case for this is walking CVS trees to find updated files for
the [FreshBSD][freshbsd] backend to process, so that's what I tested with.

`ignore` and `fast_find` are both multi-threaded, while `walkdir` and `Find` are
both sequential.


## Results

FreeBSD 12.0-RELEASE, 11.2-RELEASE jail environment (due to ongoing JRuby stat
breakage on 12), JRuby 9.2.5.0, on a 24-way Xeon L5639 with mirrored SSDs on
ZFS and huge gobs of RAM.

```
-% find /cvs/openbsd -mtime +0 |wc -l
  377634
5.866 real, 0.520 user, 5.346 sys

-% target/release/walkbench /cvs/openbsd
 ignore: 6.83s 6.63s 6.25s 6.52s 8.31s [avg best 4: 6.56s]
walkdir: 9.13s 9.13s 9.10s 9.13s 9.11s [avg best 4: 9.12s]
1:20.14 real, 14.339 user, 1:40.24 sys

-% ruby walkbench.rb /cvs/openbsd
    Find: 24.68s 23.42s 22.99s 22.89s 22.87s [ avg best 4: 23.04s ]
FastFind:  4.11s  4s     3.74s  3.46s  3.77s [ avg best 4:  3.74s ]
2:19.95 real, 1:44.57 user, 3:22.52 sys
```

(Manually aligned for readability)


ignore: https://crates.io/crates/ignore
walkdir: https://crates.io/crates/walkdir
find: https://ruby-doc.org/stdlib-2.6/libdoc/find/rdoc/Find.html
fast_find: https://rubygems.org/gems/fast_find
freshbsd: https://freshbsd.org
