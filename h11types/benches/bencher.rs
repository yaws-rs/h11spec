use criterion::{black_box, criterion_group, criterion_main, Criterion};

use h11types::H11RequestMeta;

fn criterion_benchmark(c: &mut Criterion) {
    let input_status = "POST / HTTP/1.1\r\n".as_bytes();
    let input_headers = "Host: foo.bar\r\nAccept: */*\r\nContent-Length: 42\r\n".as_bytes();

    let input_headers_fuller = "User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\nHost: antarctica.net\r\nContent-Type: text/xml; charset=utf-8\r\nContent-Length: 100\r\nAccept-Language: en-us\r\nAccept-Encoding: gzip, deflate\r\nConnection: Keep-Alive\r\n".as_bytes();

    c.bench_function("h11types parse headers - three headers", |b| {
        let mut meta = H11RequestMeta::default();

        b.iter(|| {
            let _advanced = meta.advance_headers_with(black_box(input_headers)).unwrap();
        })
    });

    c.bench_function("h11types parse headers - nine headers", |b| {
        let mut meta = H11RequestMeta::default();

        b.iter(|| {
            let _advanced = meta
                .advance_headers_with(black_box(input_headers_fuller))
                .unwrap();
        })
    });

    c.bench_function("httparse equivalent parse_headers - three headers", |b| {
        let mut headers = [httparse::EMPTY_HEADER; 3];

        b.iter(|| {
            httparse::parse_headers(black_box(&input_headers), &mut headers).unwrap();
            let mut content_length = 0;
            for header in headers {
                if header.name == "Content-Length" {
                    content_length = core::str::from_utf8(header.value)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
            }
        })
    });

    c.bench_function("httparse equivalent parse_headers - nine headers", |b| {
        let mut headers = [httparse::EMPTY_HEADER; 9];

        b.iter(|| {
            httparse::parse_headers(black_box(&input_headers_fuller), &mut headers).unwrap();
            let mut content_length = 0;
            for header in headers {
                if header.name == "Content-Length" {
                    content_length = core::str::from_utf8(header.value)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
