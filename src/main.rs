// エントリポイント
//
// 全体の流れ（予定）:
// 1. HTTPサーバを起動（バックグラウンドタスク）
// 2. 起動直後にClaudeのステータスを非同期で取得し、ログ出力
// 3. メインタスクはサーバの終了を待つ
//
// devcontainer.json でポート 5300 が公開されているので、サーバはそのポートを使う想定

mod client;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: ロガーの初期化（必要なら）

    // TODO: 起動時にClaudeステータスを取りに行く非同期処理を呼び出す
    //   例: let status = client::fetch_claude_status().await?;
    //       println!("Claude status: {status:?}");

    // TODO: HTTPサーバを起動して終了を待つ
    //   例: server::run("0.0.0.0:5300").await?;

    Ok(())
}
