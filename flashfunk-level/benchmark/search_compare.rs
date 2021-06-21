use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flashfunk_level::util::hash::HashMap;


fn find_hashmap(map: &HashMap<String, usize>, key: &str) -> usize {
    *(map.get(key).unwrap_or(&0))
}

fn find_vec(vec: &Vec<(String, usize)>, key: &str) -> usize {
    vec.iter().find(|(kk, _)| kk.eq(key)).map(|(_, v)| *v).unwrap_or(0)
}

fn build_hashmap(count: i32) -> HashMap<String, usize> {
    let mut hash = HashMap::new();
    for i in 0..count {
        hash.insert(i.to_string(), i as usize);
    }
    hash
}

fn build_vec(count: i32) -> Vec<(String, usize)> {
    let mut vec = vec![];
    for i in 0..count {
        vec.push((i.to_string(), i as usize));
    }
    vec
}

fn criterion_small(c: &mut Criterion) {
    let hashmap = build_hashmap(50);
    let vec = build_vec(50);
    c.bench_function("small_hash", |b| b.iter(|| find_hashmap(&hashmap, "20")));
    c.bench_function("small_vec", |b| b.iter(|| find_vec(&vec, "20")));
}

fn criterion_middle(c: &mut Criterion) {
    let hashmap = build_hashmap(1500);
    let vec = build_vec(1500);
    c.bench_function("middle_hash", |b| b.iter(|| find_hashmap(&hashmap, "150")));
    c.bench_function("middle_vec", |b| b.iter(|| find_vec(&vec,"150")));
}

fn criterion_big(c: &mut Criterion) {
    let hashmap = build_hashmap(100000);
    let vec = build_vec(100000);
    c.bench_function("big_hash", |b| b.iter(|| find_hashmap(&hashmap, &("150".to_string()))));
    c.bench_function("big_vec", |b| b.iter(|| find_vec(&vec, "150")));
}
criterion_group!(benches, criterion_small, criterion_middle, criterion_big);
criterion_main!(benches);
