/*
 * Copyright (c) 2023, Tobias Müller <git@tsmr.eu>
 *
 */

#![allow(unused_must_use)]
use anothertls::{ServerConfigBuilder, ServerConnection};
use std::net::TcpListener;

fn main() {
    anothertls::log::init();
    let config = ServerConfigBuilder::new()
        .add_cert_pem("./timing_issues/src/server.cert".to_string())
        .add_privkey_pem("./timing_issues/src/server.key".to_string())
        .build()
        .unwrap();

    println!("Listening on 127.0.0.1:4000");

    let tcp = TcpListener::bind("127.0.0.1:4000").expect("Error binding to tcp socket.");
    let listener = ServerConnection::new(tcp, config);

    loop {

        let mut socket = match listener.accept() {
            Ok((s, _)) => s,
            Err(_) => continue,
        };

        socket.tls_write(b"\
HTTP/1.1 200\r\n\
Server: VulnTLS/1.0\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Content-Length: 12\r\n\
\r\n\
Hello world!"
);

    }
}
