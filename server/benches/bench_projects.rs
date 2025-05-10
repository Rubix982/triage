use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use triage::db::get_stored_project_ids;

fn bench_get_stored_project_ids(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("get_stored_project_ids", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = get_stored_project_ids().await;
            })
        })
    });
}

criterion_group!(benches, bench_get_stored_project_ids);
criterion_main!(benches);
