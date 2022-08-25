FROM rust:1.63 as chef
WORKDIR /app

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# 代码编译
RUN cargo build --release

FROM ubuntu AS runtime
WORKDIR /app
# 资源拷贝
COPY --from=builder /app/target/release/kuth /usr/local/bin/
COPY --from=builder /app/entrypoint.sh /usr/local/bin/
# 赋予执行权限
RUN chmod +x /usr/local/bin/kuth /usr/local/bin/entrypoint.sh

EXPOSE 30050
STOPSIGNAL 2

ENTRYPOINT ["entrypoint.sh"]
CMD ["kuth"]
