cd benchmarking
cargo bench

cargo bench stand_alone_and & target\criterion\stand_alone_and\report\index.html

python summary.py > summary.tsv