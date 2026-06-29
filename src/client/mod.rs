// HTTPクライアントの実装場所
//
// 学習目的で、同じ「外部にHTTPで取りに行く」処理を2通りで書く。
//
// - reqwest_client : 高レベルライブラリ（reqwest）を使う。HTTPS / JSON の取り回しが楽。
//                    Claude (Anthropic) のステータス取得に利用。
//
// - raw_client     : 標準ライブラリ + tokio のみで、HTTPプロトコルを手で組み立てる。
//                    TLS は実装外なので、平文HTTP（例: http://example.com）が対象。
//
// サーバ側でエンドポイントを分けて呼び出し比較できるようにする想定:
//   GET /status  -> reqwest_client::fetch_claude_status
//   GET /example -> raw_client::fetch_example_com

pub mod raw_client;
pub mod reqwest_client;
