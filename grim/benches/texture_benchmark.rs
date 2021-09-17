use grim::texture::{decode_dx_image, DXGI_Encoding};
use criterion::{BenchmarkId, black_box, Criterion, criterion_group, criterion_main};

fn from_elem(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_dxt1");
    for width in [32, 64, 128, 256, 512, 1024, 2048] {
        let packed_data = vec![0u8; (width * width) / 2]; // 4bpp
        let mut rgba = vec![0u8; (width * width) * 4];

        group.bench_function(width.to_string().as_str(), |b| {
            b.iter(|| {
                decode_dx_image(
                    packed_data.as_slice(),
                    rgba.as_mut_slice(),
                    width as u32,
                    DXGI_Encoding::DXGI_FORMAT_BC1_UNORM,
                    false
                );
            });
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);