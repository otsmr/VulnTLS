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
        .set_prng(anothertls::PRNG::TripleEc)
        .add_cert_pem("./triple_ec/src/server.cert".to_string())
        .add_privkey_pem("./triple_ec/src/server.key".to_string())
        .enable_keylog()
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

        let flag = "flag_XXXXXXXXXXXXXXXXXXXXXX_";
        socket.tls_write(
            format!(
                "\
HTTP/1.1 200\r\n\
Server: VulnTLS/1.0\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
                flag.len(),
                flag
            )
            .as_bytes(),
        );
    }
}
