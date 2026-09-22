use super::InteractiveRemoteOptions;
use super::resolve_remote_endpoint;
use clap::CommandFactory;

#[test]
fn remote_options_help() {
    let help = InteractiveRemoteOptions::command()
        .name("codex")
        .term_width(/*width*/ 80)
        .render_long_help()
        .to_string()
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    insta::assert_snapshot!(help);
}

#[test]
fn invalid_remote_address_error() {
    let error = resolve_remote_endpoint(
        Some("https://example.invalid/codex-app-server".to_string()),
        /*remote_auth_token_env*/ None,
    )
    .expect_err("HTTPS is not a WebSocket endpoint");
    insta::assert_snapshot!(error.to_string());
}
