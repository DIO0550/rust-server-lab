// ライブラリを使わない（tokio の TcpStream だけ使う）HTTPクライアント
//
// 目的: HTTP/1.1 のリクエスト・レスポンスをバイト列レベルで理解する。
//
// 注意: TLS は実装しないので、対象は平文HTTPのみ。
//   例: http://example.com/    （ポート80、HTTP/1.1）
//
// HTTP/1.1 のリクエスト最小形:
//   GET / HTTP/1.1\r\n
//   Host: example.com\r\n
//   Connection: close\r\n
//   \r\n
//
// レスポンスの構造:
//   HTTP/1.1 200 OK\r\n
//   Header: value\r\n
//   ...\r\n
//   \r\n
//   <body>
//
// Connection: close を付けておくと、サーバがEOFで本文の終わりを伝えてくれるので
// 学習時のボディ読み出しが簡単になる（chunked / Content-Length のパースを避けられる）。

/// 平文HTTPのページを取得する。
/// 例: fetch("example.com", 80, "/")  -> "HTTP/1.1 200 OK ..." 全文
pub async fn fetch(_host: &str, _port: u16, _path: &str) -> anyhow::Result<String> {
    // TODO: tokio::net::TcpStream::connect((host, port)).await? で接続

    // TODO: リクエスト文字列を組み立てる
    //   format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n")

    // TODO: stream.write_all(req.as_bytes()).await?
    //       （use tokio::io::AsyncWriteExt が必要）

    // TODO: レスポンスをEOFまで読む
    //   let mut buf = Vec::new();
    //   stream.read_to_end(&mut buf).await?;
    //   （use tokio::io::AsyncReadExt が必要）

    // TODO: String::from_utf8(buf)? で返す（学習用にヘッダごと丸ごと返してOK）

    // TODO（発展）: ステータスライン / ヘッダ / ボディに自前でパース分割する

    Ok(String::from("TODO: raw fetch not implemented"))
}

/// 学習用のショートカット。example.com のトップを取りに行く。
pub async fn fetch_example_com() -> anyhow::Result<String> {
    // TODO: fetch("example.com", 80, "/").await
    Ok(String::from("TODO: fetch_example_com not implemented"))
}
