extern crate alloc;
use alloc::format;
use core::{
    error::{Error, Request},
    intrinsics::mir::Return,
    net::SocketAddr,
    str::Bytes,
};
use create::alloc::string::ToString;
use noli::net::lookup_host;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        // ロードバランサや複数のサーバが同じドメイン名に対して複数の
        // IPアドレスを持つ可能性があるため戻り値はベクタにする
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        // IP アドレスが1つもない場合はエラー
        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        // TCP/IPネットワーク上で通信する際に送信元や送信先を識別するために使用
        // IP アドレス + ポート番号の組み合わせでできる
        let socket_addr: SocketAddr = (ips[0], port).into();

        // TCP コネクション接続を実行
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ))
            }
        };

        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // ホストの追加
        // リクエスト先のホストとポート番号を指定する
        request.push_str("Hots: ");
        request.push_str(&host);
        request.push('\n');

        // Accept
        // クライアントが受け入れ可能な応答のコンテンツタイプを指定する
        request.push_str("Accept: test/html\n");

        // クライアントとサーバ間の接続に関する情報を指定する
        request.push_str("Connection: close\n");

        request.push('\n');

        // リクエスト送信
        // 使う予定のない変数を定義する時はアンダースコア始まりにする
        let _bytes_written = match stream.write(request.as_bytess()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ))
            }
        };

        // レスポンスの受信
        // レスポンスが長い場合、分割して送られてくるため
        // loop で読み込むバイトが無くなるまで処理を実行する
        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(butes) => bytes,
                Err(_) => {
                    return Err(Error::Network(
                        "Failed to receive a request from TCP stream".to_string(),
                    ))
                }
            };
            if bytes_read == 0 {
                break;
            }
            // 分割したストリームを繋ぎ合わせる
            received.extend_from_slice(&buf[..bytes_read]);
        }

        // HTTP レスポンスの構築
        match core::str::from_utf8(&received) {
            // レスポンスデータは UTF-8 のバイト列なので、str 型に変換
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => Err(Error::Network(format!("Invalid received response: {}", e))),
        }
    }
}
