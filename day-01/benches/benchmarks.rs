use day_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box("../.input_data/day1a.tsv")).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(
        "../input2.txt",
    ))
    .unwrap();
}
