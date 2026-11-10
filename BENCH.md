# Build-deploy benchmark fixture: Rust (actix-web + Diesel + Postgres)

## Source

- Upstream: https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel at `1b844c094208be54008cd993ef51d87b808eea24` (2023-12-25). MIT, see `LICENSE.md`.
- Private copy: https://github.com/build-deploy-bench/rust-actix-postgres-example. The first commit (`initial commit`) is the unmodified upstream tree.
- Smoke-deployed commit on all four providers: `8e7e450c95ba41df6844a8e0522d033779482be8`.

## Patches

| Commit | Change |
|---|---|
| Listen on PORT | Bind `0.0.0.0:${PORT:-8080}` (upstream hardcoded `0.0.0.0:8080`). Logs `Server running on port <n>`, the same text as the Node fixture. |
| Run embedded migrations at startup | `diesel_migrations::embed_migrations!` (pinned `~2.1.0` to match Diesel 2.1.0 in the lockfile) runs at boot, under a Postgres advisory lock so parallel instances do not race. No provider needs `diesel_cli`. |
| Add /version | `GET /version` returns `{"commit":"<sha>","run":"<id>","deployment":"<id>"}`. `run` is the runtime env var `BENCH_RUN_ID` (else `unknown`). The harness sets it before each deploy, so it identifies the deployment even when the commit is the same. `deployment` is `RAILWAY_DEPLOYMENT_ID` (else `unknown`): Railway gets no `BENCH_RUN_ID`, because a changed variable invalidates part of its build cache. `build.rs` embeds the first non-empty one of `BENCH_COMMIT`, `RENDER_GIT_COMMIT`, `RAILWAY_GIT_COMMIT_SHA`, `VERCEL_GIT_COMMIT_SHA`, else `unknown`. It does not touch the database. |
| Pin Rust, add markers | `rust-toolchain.toml` pins `1.99.1` (profile minimal). `bench/build.sh {install|build|all}` prints the `::bench::` markers. |
| CORS default | Upstream defaulted `FRONTEND_ORIGIN` to `"*"`, which actix-cors rejects, so the server never started without the variable. Unset now means any origin. |
| Fly and Vercel configs | `Dockerfile.fly` and `Dockerfile.vercel` (identical below the header), `fly.toml`, `vercel.json`. The upstream root `Dockerfile` (dev image with cargo-watch) is renamed to `Dockerfile.dev`; `compose.yaml` points to it. |

## Commands

- Install: `cargo fetch --locked` (inside `./bench/build.sh install`)
- Build: `cargo build --release --locked` (inside `./bench/build.sh build`)
- Start: `./target/release/conduit`
- Health path: `/api/tags` (runs a DB query). `/api/healthcheck` also exists and returns `OK` without the DB.
- App env: `DATABASE_URL` (required), `SECRET_KEY` (required, JWT HMAC key), `FRONTEND_ORIGIN` (optional), `PORT` (optional).
- Native builds need `libpq` (Diesel `pq-sys`). Docker images use `libpq-dev` in the build stage and `libpq5` in the runtime stage.

## Providers

### Render

- Workspace `tea-dbmbhbg8v5cc73e4m0jg` (Pro). Service `srv-dbpn9p7f3r2c738mqsf0` `bdb-rust-actix-pg`, https://bdb-rust-actix-pg.onrender.com, virginia.
- Native `rust` runtime. Instance plan `starter`, build plan `starter`, cache profile `no-cache` (the API default, same as the Node service). One instance. Auto-deploy off.
- Build `./bench/build.sh all`, start `./target/release/conduit`, health check `/api/tags`.
- Database: new database `rust_benchmark` in the existing free Postgres `dpg-dbmbo06gekts73an202g-a` (`node-benchmark`, PG 18), same user as `node_benchmark`. `DATABASE_URL` is the internal URL with the database name changed. The free instance expires 2026-12-05.
- libpq is present in the Render native environment: `pq-sys` links and the binary runs with no extra packages.
- Toolchain: Render installs `stable` before the build command, then rustup auto-installs the pinned 1.99.1 on the first `cargo` call. That second install (about 9 s) falls inside the install phase.
- The first service (`srv-dbpn2ojncjis73ejvdh0`, free) was deleted and recreated on `starter`. Service names must be unique, so the old one was deleted first; the URL stayed the same. One request returned 502 right after `live` while the subdomain switched, then 200.
- Smoke deploy `dep-dbpn9pnf3r2c738mqu3g` (the creation deploy): live, `/api/tags` 200, `/version` correct.

### Railway

- Workspace `d535606c-49a7-4a6c-9e29-ce73b5f01428` (Pro). Project `bdb-rust-actix-pg` `e61d3f3e-d882-4c3b-9f68-9ea78c97b714`, environment `production` `025603f4-0ab8-4c7d-a131-9f749266c320`.
- Service `bdb-rust-actix-pg` `07b7dce5-eb61-4fd2-8bba-6b5f8b9d7c7d`, https://bdb-rust-actix-pg-production.up.railway.app, `iad`, 1 replica. Postgres template service `8dd65b61-0c36-40f3-b223-a5237aecb45e`.
- Builder: native Railpack (0.43.0, Rust provider; it reads `rust-toolchain.toml`). The Railpack builder image already has `libpq-dev`.
- Variables: `DATABASE_URL=${{Postgres.DATABASE_URL}}`, `SECRET_KEY` (random), `RAILPACK_INSTALL_CMD=./bench/build.sh install`, `RAILPACK_BUILD_APT_PACKAGES=libpq-dev`, `RAILPACK_DEPLOY_APT_PACKAGES=libpq5`.
- Build command `./bench/build.sh build && mkdir -p bin && cp target/release/conduit bin/`, start `./bin/conduit`, health check `/api/tags`. Railpack's Rust runtime image keeps only `bin/`, so the binary path differs from the other providers.
- Auto-deploy disabled with `serviceInstanceAutoDeployUpdate`; `serviceInstanceAutoDeployStatus` reports `enabled: false`.
- The creation deploy `dc9a54c0-c462-435b-be66-80d3e4a2e5e7` failed (it ran before the settings were complete). Smoke deploy `e3c83135-f481-40a1-abfe-86b1a8d2bda8` (`railway redeploy --commit`): SUCCESS, `/api/tags` 200, `/version` correct.

### Vercel

- Team `build-deploy-bench` `team_oVjpdXNb0OhSfAzc6FSNjen8`. Project `bdb-rust-actix-pg` `prj_4mMmwwF6HMyvDeROweAGsGskQ3nn`, https://bdb-rust-actix-pg.vercel.app, function region `iad1`, build machine standard (elastic), Fluid. No Git link (same as the Node project); deploys are CLI uploads of a clean `git archive`.
- Container Runtime via `Dockerfile.vercel` and `vercel.json` (`framework: container`, `git.deploymentEnabled: false`).
- Database: new database `rust_conduit` in the existing Neon project of the `bdb-express-react-pg-db` resource (created with SQL as `neondb_owner`). `DATABASE_URL` (sensitive, production) is the unpooled URL with the database name changed. `SECRET_KEY` (sensitive, production).
- The build log shows `Restoring the Buildah store cache ... miss`, so the store cache is active on this project without `VERCEL_BUILDAH_STORE_CACHE`.
- Smoke deploy `bdb-rust-actix-9d3gauk97-build-deploy-bench.vercel.app`: READY, `/api/tags` 200. `/version` returns `unknown`: on a CLI upload Buildah received neither `--build-env BENCH_GIT_COMMIT` nor `VERCEL_GIT_COMMIT_SHA` (log: `missing "BENCH_GIT_COMMIT" build argument`; the argument is now `BENCH_COMMIT`). Follow-up: a Git-source deployment or another verified way to pass the commit. The probe now identifies a deployment by `run` (`BENCH_RUN_ID`, set per deployment with `vercel deploy --env BENCH_RUN_ID=<id>`), so the commit is no longer needed for that.

### Fly.io

- Org `personal`. App `bdb-rust-actix-pg`, https://bdb-rust-actix-pg.fly.dev, `iad`, one Machine `845d23f2d00448` shared-cpu-1x 512 MB, always on (`fly.toml`).
- Database: the existing `bdb-express-react-pg-db` postgres-flex cluster, attached with `flyctl postgres attach ... --database-name rust_conduit --variable-name DATABASE_URL`. The cluster Machine `8747907be4e508` was stopped and was started for this setup. `SECRET_KEY` set with `secrets import --stage`.
- Deploy from a clean `git archive` with `--build-arg BENCH_COMMIT=<sha>` (the harness must pass it for `/version`).
- The default Depot builder failed twice (`connecting to Depot builder: ... authentication handshake failed: EOF`). The smoke deploy used `--depot=false`, which created the org builder app `fly-builder-rosy-headland-8386`. Its first attempt built but failed at the image push (`unable to upgrade to h2c, received 500`); the retry used cached layers and succeeded.
- Smoke deploy: Machine healthy, `/api/tags` 200, `/version` correct.
- 2026-11-10: deploy of `548d90c` with `--depot=false --build-arg BENCH_COMMIT=<sha>`: `/version` returned that SHA, `/api/tags` 200.
