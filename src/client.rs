// 非同期HTTPクライアントの実装場所
//
// 学習目的: reqwest を使った async/await の基本を確認する。
// 例として Claude (Anthropic) のステータスページから状態を取得する。
//
// 参考URL（statuspage.io 形式）:
//   https://status.anthropic.com/api/v2/status.json
//   レスポンス例: { "status": { "indicator": "none", "description": "All Systems Operational" }, ... }

/// Claudeのステータスを取得する。
/// 戻り値は学習が進んだら structured な型（serde の derive）に置き換える想定。
pub async fn fetch_claude_status() -> anyhow::Result<String> {
    // TODO: reqwest::Client を作る（必要なら使い回すために OnceCell などに入れる）

    // TODO: GET https://status.anthropic.com/api/v2/status.json を投げる
    //   let resp = client.get(URL).send().await?;

    // TODO: ステータスコードチェック → resp.text().await? もしくは resp.json::<T>().await?

    // TODO: 必要な部分（description など）だけ抽出して返す

    Ok(String::from("TODO: not implemented"))
}
