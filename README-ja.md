# Turn-Based Boardgame Engine (Rust)

Rust 製のターン制ボードゲーム用エンジンです。  
1人以上で遊べ、ローカルプレイ・オンラインプレイ双方に対応します。  

## 特徴

- **ターン制ゲーム専用**
- **Rust 製ワークスペース構成**
  - `engine-core` : ゲーム状態管理（決定的な状態遷移、スナップショット、リプレイ）
  - `engine-net`  : WebRTC 通信抽象（STUN/TURN フォールバック、再接続対応）
  - `engine-ui`   : UI 抽象（egui、WASM 対応）
  - `engine-proto`: ネットワークメッセージ定義
  - `signaling-server`: 軽量シグナリングサーバ（WebSocket + TURN 資格情報）
  - `game-logic-othello` : サンプルゲーム（オセロ）
  - `game-ui-othello`    : オセロ UI（egui 実装）
- **ローカル多人数**：同一 PC 上で複数プレイヤー操作が可能
- **オンライン**：ルーム番号によるプライベートマッチ（将来的にランダムマッチ対応予定）
- **再接続対応**：切断後にスナップショット＋差分再送で続行可能
- **リプレイ機能**：対局の一手ずつ確認可能

## 動作環境

- Rust 1.80+
- ネイティブ: Linux / macOS / Windows
- ブラウザ: WebAssembly (wasm32-unknown-unknown, trunk)
