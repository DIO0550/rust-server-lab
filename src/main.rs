// エントリポイント
//
// 全体の流れ（予定）:
// 1. HTTPサーバを起動して、ポート 5300 で待ち受ける
// 2. サーバ側で以下のエンドポイントを公開する
//      GET /status   -> reqwest 版クライアントで Claude のステータスを取得して返す
//      GET /example  -> 生TCP版クライアントで http://example.com を取得して返す
// 3. メインタスクはサーバの終了を待つ

mod client;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: ロガーの初期化（必要なら）

    // TODO: HTTPサーバを起動して終了を待つ
    //   例: server::run("0.0.0.0:5300").await?;

    Ok(())
}
