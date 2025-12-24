use criterion::{Criterion, criterion_group, criterion_main};
use merge_alternately::Solution;

const ALL_INPUTS: [(&str, &str); 3] =
    [("abcccc", "def"), ("abc", "defffff"), ("abc", "def")];

fn bench_merge_alternately(c: &mut Criterion) {
    c.bench_function("bench_merge_alternately", |b| {
        b.iter(|| {
            let _: () = for (word1, word2) in ALL_INPUTS.iter() {
                Solution::merge_alternately(word1.to_string(), word2.to_string());
            };
        })
    });
}

criterion_group!(benches, bench_merge_alternately,);
criterion_main!(benches);
