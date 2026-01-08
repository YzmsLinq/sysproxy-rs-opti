use criterion::{Criterion, criterion_group, criterion_main};
use std::process::Command;
use sysproxy::{Autoproxy, Sysproxy};

fn get_valid_service() -> String {
    let output = Command::new("networksetup")
        .args(&["-listallnetworkservices"])
        .output()
        .expect("failed to execute networksetup");
    let stdout = String::from_utf8(output.stdout).expect("invalid utf8");
    let mut lines = stdout.split('\n');
    lines.next(); // skip header
    lines.next().expect("no network services found").to_string()
}

fn bench_get_system_proxy(c: &mut Criterion) {
    c.bench_function("get_system_proxy", |b| {
        b.iter(|| Sysproxy::get_system_proxy())
    });
}

fn bench_get_http(c: &mut Criterion) {
    let service = get_valid_service();
    c.bench_function("get_http", |b| b.iter(|| Sysproxy::get_http(&service)));
}

fn bench_get_https(c: &mut Criterion) {
    let service = get_valid_service();
    c.bench_function("get_https", |b| b.iter(|| Sysproxy::get_https(&service)));
}

fn bench_get_socks(c: &mut Criterion) {
    let service = get_valid_service();
    c.bench_function("get_socks", |b| b.iter(|| Sysproxy::get_socks(&service)));
}

fn bench_get_bypass(c: &mut Criterion) {
    let service = get_valid_service();
    c.bench_function("get_bypass", |b| b.iter(|| Sysproxy::get_bypass(&service)));
}

fn bench_has_permission(c: &mut Criterion) {
    c.bench_function("has_permission", |b| b.iter(|| Sysproxy::has_permission()));
}

fn bench_set_http(c: &mut Criterion) {
    let service = get_valid_service();
    let proxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 8080,
        bypass: "".into(),
    };
    c.bench_function("set_http", |b| b.iter(|| proxy.set_http(&service)));
}

fn bench_set_https(c: &mut Criterion) {
    let service = get_valid_service();
    let proxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 8080,
        bypass: "".into(),
    };
    c.bench_function("set_https", |b| b.iter(|| proxy.set_https(&service)));
}

fn bench_set_socks(c: &mut Criterion) {
    let service = get_valid_service();
    let proxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 8080,
        bypass: "".into(),
    };
    c.bench_function("set_socks", |b| b.iter(|| proxy.set_socks(&service)));
}

fn bench_set_bypass(c: &mut Criterion) {
    let service = get_valid_service();
    let proxy = Sysproxy {
        enable: false,
        host: "".into(),
        port: 0,
        bypass: "127.0.0.1,localhost".into(),
    };
    c.bench_function("set_bypass", |b| b.iter(|| proxy.set_bypass(&service)));
}

fn bench_set_system_proxy(c: &mut Criterion) {
    let proxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 8080,
        bypass: "".into(),
    };
    c.bench_function("set_system_proxy", |b| b.iter(|| proxy.set_system_proxy()));
}

fn bench_get_auto_proxy(c: &mut Criterion) {
    c.bench_function("get_auto_proxy", |b| b.iter(|| Autoproxy::get_auto_proxy()));
}

fn bench_set_auto_proxy(c: &mut Criterion) {
    let proxy = Autoproxy {
        enable: false,
        url: "".into(),
    };
    c.bench_function("set_auto_proxy", |b| b.iter(|| proxy.set_auto_proxy()));
}

fn custom_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .warm_up_time(std::time::Duration::from_secs(1))
        .measurement_time(std::time::Duration::from_secs(120))
}

criterion_group! {
    name = benches;
    config = custom_config();
    targets =
        bench_get_system_proxy,
        bench_get_http,
        bench_get_https,
        bench_get_socks,
        bench_get_bypass,
        bench_has_permission,
        bench_set_http,
        bench_set_https,
        bench_set_socks,
        bench_set_bypass,
        bench_set_system_proxy,
        bench_get_auto_proxy,
        bench_set_auto_proxy
}
criterion_main!(benches);
