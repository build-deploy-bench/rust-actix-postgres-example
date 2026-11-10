// Embeds the deployed commit for the /version endpoint.
// Each provider exposes the commit at build time under its own name. Docker builds without
// such a variable (Fly) get the commit as the BENCH_GIT_COMMIT build argument.
const COMMIT_VARS: [&str; 4] = [
    "BENCH_GIT_COMMIT",
    "RENDER_GIT_COMMIT",
    "RAILWAY_GIT_COMMIT_SHA",
    "VERCEL_GIT_COMMIT_SHA",
];

fn main() {
    for var in COMMIT_VARS {
        println!("cargo:rerun-if-env-changed={var}");
    }
    let commit = COMMIT_VARS
        .iter()
        .find_map(|var| std::env::var(var).ok().filter(|value| !value.is_empty()))
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=CONDUIT_COMMIT={commit}");
}
