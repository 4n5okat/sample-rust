#########################################################
# dockerfileはマルチステージビルドを利用します。
# docker composeまたはdocker buildするときに、(--target development, --target builder, --target staging, --target production)でビルドステージを指定することができます。
#########################################################

########################
## ベースイメージの作成
########################
FROM rust:1.85.1-slim-bullseye AS base

# 必要なツールをインストール
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを指定
WORKDIR /app

# ソースコードをコピー
COPY ./app ./app

########################
## 開発環境のビルド
########################
FROM base AS development

# 開発用のcargo-watchをインストール
RUN cargo install cargo-watch

# ログレベルの設定
ENV RUST_LOG=info

# ポートを開放
EXPOSE 8080

# 開発環境用に cargo watch 実行
CMD [ "cargo", "watch", "-x", "run" ]

########################
## コンパイル(コンパイル言語時には記載する)
########################
FROM base AS builder

# 依存関係を先に解決してキャッシュを活用
COPY ./app/Cargo.toml ./Cargo.toml
COPY ./app/Cargo.lock ./Cargo.lock
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# 実際のソースをコピー
COPY ./app/src ./src

# アプリケーションをビルド
RUN cargo build --release

########################
## ステージング環境のビルド
########################
FROM debian:bullseye-slim AS staging

# 必要なランタイムライブラリのみインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# builderステージからバイナリをコピー（ここでは app-name に置き換えてください）
COPY --from=builder /app/target/release/app .

# 実行ファイルの権限を確認
RUN chmod +x app
EXPOSE 8080

CMD ["./app"]

########################
## 本番環境環境のビルド
########################
FROM staging AS production

# 本番とステージングの違いがある場合はここで調整
# たとえばログレベルや環境変数など

ENV RUST_LOG=info
