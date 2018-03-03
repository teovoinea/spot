extern crate futures;
extern crate hyper;
extern crate hyper_staticfile;
extern crate tokio_core;

// This example serves the docs from target/doc/hyper_staticfile at /doc/
//
// Run `cargo doc && cargo run --example doc_server`, then
// point your browser to http://localhost:3000/

use futures::{Future, Stream, future};
use hyper::Error;
use hyper::server::{Http, Request, Response, Service};
use hyper_staticfile::Static;
use std::path::Path;
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;

type ResponseFuture = Box<Future<Item=Response, Error=Error>>;

struct MainService {
    static_: Static,
}

impl MainService {
    fn new(handle: &Handle) -> MainService {
        MainService {
            static_: Static::new(handle, Path::new("src/static/")),
        }
    }
}

impl Service for MainService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = ResponseFuture;

    fn call(&self, req: Request) -> Self::Future {
        self.static_.call(req)
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();

    let http = Http::new();
    let server = listener.incoming().for_each(|(sock, addr)| {
        let s = MainService::new(&handle);
        http.bind_connection(&handle, sock, addr, s);
        Ok(())
    });

    println!("Doc server running on http://localhost:3000/");
    core.run(server).unwrap();
}
