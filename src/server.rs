// シンプルなHTTPサーバの実装場所
//
// 学習目的なので、フレームワーク（axum, actix-web 等）は使わずに
// tokio の TcpListener / TcpStream でHTTPプロトコルを手書きする方針。
//
// 大まかな構造:
//   run(addr)
//     ├─ TcpListener::bind でアドレスをバインド
//     └─ ループで accept → tokio::spawn で handle_connection を起動
//
//   handle_connection(stream)
//     ├─ リクエスト行 + ヘッダを読み取り
//     ├─ ルーティング
//     │     GET /         -> "Hello"
//     │     GET /status   -> client::reqwest_client::fetch_claude_status() の結果
//     │     GET /example  -> client::raw_client::fetch_example_com() の結果
//     │     その他          -> 404
//     └─ ステータスライン + ヘッダ + ボディを書き戻す

/// サーバを起動する。指定アドレスで listen し、接続を捌き続ける。
pub async fn run(_addr: &str) -> anyhow::Result<()> {
    // TODO: TcpListener::bind(addr).await でリスナを作る

    // TODO: ループで accept し、各接続を tokio::spawn で handle_connection に渡す
    //   loop {
    //       let (stream, _peer) = listener.accept().await?;
    //       tokio::spawn(async move { handle_connection(stream).await });
    //   }

    Ok(())
}

/// 1接続を処理する。HTTPリクエストをパースして、適当なレスポンスを返す。
#[allow(dead_code)]
async fn handle_connection(/* stream: tokio::net::TcpStream */) -> anyhow::Result<()> {
    // TODO: BufReader でリクエスト行（"GET / HTTP/1.1"）を読む

    // TODO: ヘッダを空行までループで読む

    // TODO: メソッドとパスでルーティング
    //   - GET /         -> 200 OK, "Hello, world!"
    //   - GET /status   -> reqwest 版で Claude ステータス取得（HTTPS + JSON）
    //                      crate::client::reqwest_client::fetch_claude_status().await
    //   - GET /example  -> 生TCP版で example.com を取得（平文HTTPの組み立て練習）
    //                      crate::client::raw_client::fetch_example_com().await
    //   - その他          -> 404 Not Found

    // TODO: レスポンスを書き戻す（status line + headers + "\r\n" + body）

    Ok(())
}
