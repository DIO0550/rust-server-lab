// reqwest を使った非同期HTTPクライアント
//
// 目的: 高レベルAPIで HTTPS + JSON を扱う流れを掴む。
//
// 対象URL（statuspage.io 形式）:
//   https://status.anthropic.com/api/v2/status.json
//   レスポンス例:
//     { "status": { "indicator": "none", "description": "All Systems Operational" }, ... }
//
// ポイント:
//   - TLS は reqwest（hyper + rustls/native-tls）が面倒を見てくれる
//   - .json::<T>().await で serde::Deserialize な型に直接デシリアライズできる

/// Claudeのステータスを取得する。
pub async fn fetch_claude_status() -> anyhow::Result<String> {
    // TODO: reqwest::Client::new() でクライアントを作る
    //       （実運用ではコネクション再利用のために使い回すのが定石）

    // TODO: client.get("https://status.anthropic.com/api/v2/status.json").send().await?

    // TODO: ステータスコードをチェック（resp.error_for_status()?）

    // TODO: resp.json::<StatusResponse>().await? で構造体に落とす
    //       もしくは resp.text().await? でいったん文字列で受け取る

    // TODO: description を取り出して返す

    Ok(String::from("TODO: fetch_claude_status not implemented"))
}
