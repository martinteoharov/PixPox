use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pixpox_app::{App, AppConfig};
use winit::dpi::LogicalPosition;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn ecs(c: &mut Criterion) {
    // TODO: read config from file
    let config = AppConfig {
        WINDOW_TITLE: "Conway",
        WINDOW_HEIGHT: 250,
        WINDOW_WIDTH: 500,
        WINDOW_FULLSCREEN: false,
        DEBUG: true,
    };

    let mut app = App::new(config);

    let entity = app.world.spawn();

    let pos = LogicalPosition::new(0, 0);
    let alive = true;

    let cell_component = Cell::new(entity.id, pos, alive);

    app.world.add_component_to_entity(entity, cell_component);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
