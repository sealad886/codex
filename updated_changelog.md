# Changelog

You can install any of these versions: `npm install -g codex@version`
## [0.2.0-alpha.3] - 2025-09-27
### 🚀 Features

- Add query_params option to ModelProviderInfo to support Azure (#1435)
- Support custom HTTP headers for model providers (#1473)
- Add support for --sandbox flag (#1476)
- Add reasoning fields to profile settings (#1484)
- Honor OPENAI_BASE_URL for the built-in openai provider (#1487)
- Add `codex completion` to generate shell completions (#1491)
- Add new config option: model_supports_reasoning_summaries (#1524)
- Ctrl-d only exits when there is no user input (#1589)
- Add --json flag to `codex exec` (#1603)
- Ensure session ID header is sent in Response API request (#1614)
- Leverage elicitations in the MCP server (#1623)
- Support dotenv (including ~/.codex/.env) (#1653)
- Expand the set of commands that can be safely identified as "trusted" (#1668)
- Map ^U to kill-line-to-head (#1711)
- Stream exec stdout events (#1786)
- Make .git read-only within a writable root when using Seatbelt (#1765)
- Accept custom instructions in profiles (#1803)
- Update launch screen (#1881)
- Interrupt running task on ctrl-z (#1880)
- >_ (#1924)
- Use ctrl c in interrupt hint (#1926)
- Add /tmp by default (#1919)
- Parse info from auth.json and show in /status (#1923)
- Change shell_environment_policy to default to inherit="all" (#1904)
- /prompts slash command (#1937)
- Improve output of /status (#1936)
- Add tip to upgrade to ChatGPT plan (#1938)
- Update system prompt (#1939)
- Include windows binaries in GitHub releases (#2035)
- Include Windows binary of the CLI in the npm release (#2040)
- Allow esc to interrupt session (#2054)
- Add JSON schema sanitization for MCP tools to ensure compatibil… (#1975)
- Add a /mention slash command (#2114)
- *(tui)* Add ctrl-b and ctrl-f shortcuts (#2260)
- Support traditional JSON-RPC request/response in MCP server (#2264)
- Add support for an InterruptConversation request (#2287)
- Introduce TurnContext (#2343)
- Introduce Op:UserTurn (#2329)
- Introduce ClientRequest::SendUserTurn (#2345)
- Move session ID bullet in /status (#2462)
- Copy tweaks (#2502)
- *(gpt5)* Add model_verbosity for GPT‑5 via Responses API (#2108)
- StreamableShell with exec_command and write_stdin tools (#2574)
- Use the arg0 trick with apply_patch (#2646)
- Remove the GitHub action that runs Codex for now (#2729)
- *(core)* Add `remove_conversation` to `ConversationManager` for ma… (#2613)
- Decrease testing when running interactively (#2707)
- Add Android/Termux support by gating arboard dependency (#2895)
- Add stable file locking using std::fs APIs (#2894)
- *(tui)* Show minutes/hours in thinking timer (#3220)
- *(mcp)* Per-server startup timeout (#3182)
- Run cargo shear during CI (#3338)
- POSIX unification and snapshot sessions (#3179)
- Add ArchiveConversation to ClientRequest (#3353)
- *(core)* Re-export InitialHistory from conversation_manager (#3270)
- Add UserInfo request to JSON-RPC server (#3428)
- TUI onboarding (#3398)
- Include reasoning_effort in NewConversationResponse (#3506)
- Added SetDefaultModel to JSON-RPC server (#3512)
- Change the behavior of SetDefaultModel RPC so None clears the value. (#3529)
- Reasoning effort as optional (#3527)
- Context compaction (#3446)
- Update model save (#3589)
- UI animation (#3590)
- Add reasoning level to header (#3622)
- Tighten preset filter, tame storage load logs, enable rollout prompt by default (#3628)
- Update splash (#3631)
- Skip animations on small terminals (#3647)
- Tweak onboarding strings (#3650)
- /review (#3774)
- Add more /review options (#3961)
- Add view stack to BottomPane (#4026)
- Update default (#4076)
- Git tooling for undo (#3914)
- Readiness tool (#4090)
- Add support for CODEX_SECURE_MODE=1 to restrict process observability (#4220)
- Introduce responses-api-proxy (#4246)

### 🪲 Bug Fixes

- Softprops/action-gh-release@v2 should use existing tag instead of creating a new tag (#1436)
- The `completion` subcommand should assume the CLI is named `codex`, not `codex-cli` (#1496)
- Remove reference to /compact until it is implemented (#1503)
- When invoking Codex via MCP, use the request id as the Submission id (#1554)
- Update bin/codex.js so it listens for exit on the child process (#1590)
- Trim MCP tool names to fit into tool name length limit (#1571)
- Address review feedback on #1621 and #1623 (#1631)
- Check flags to ripgrep when deciding whether the invocation is "trusted" (#1644)
- Use PR_SET_PDEATHSIG so to ensure child processes are killed in a timely manner (#1626)
- Create separate test_support crates to eliminate #[allow(dead_code)] (#1667)
- Add true,false,nl to the list of trusted commands (#1676)
- Paste with newlines (#1682)
- Crash on resize (#1683)
- Correctly wrap history items (#1685)
- Move arg0 handling out of codex-linux-sandbox and into its own crate (#1697)
- Use std::env::args_os instead of std::env::args (#1698)
- Support special --codex-run-as-apply-patch arg (#1702)
- Long lines incorrectly wrapped (#1710)
- Fix ci (#1739)

I think this commit broke the CI because it changed the
`McpToolCallBeginEvent` type:
https://github.com/openai/codex/commit/347c81ad0049103c84e0aa2c0d7e2988db18218a
- Run apply_patch calls through the sandbox (#1705)
- Fix git tests (#1747)

the git tests were failing on my local machine due to gpg signing config
in my ~/.gitconfig. tests should not be affected by ~/.gitconfig, so
configure them to ignore it.
- Ensure PatchApplyBeginEvent and PatchApplyEndEvent are dispatched reliably (#1760)
- Always send KeyEvent, we now check kind in the handler (#1772)
- Fix insert_history modifier handling (#1774)

This fixes a bug in insert_history_lines where writing
`Line::From(vec!["A".bold(), "B".into()])` would write "B" as bold,
because "B" didn't explicitly subtract bold.
- Fix command duration display (#1806)

we were always displaying "0ms" before.

<img width="731" height="101" alt="Screenshot 2025-08-02 at 10 51 22 PM"
src="https://github.com/user-attachments/assets/f56814ed-b9a4-4164-9e78-181c60ce19b7"
/>
- Disable reorderArrays in tamasfe.even-better-toml (#1837)
- Correct spelling error that sneaked through (#1855)
- Clean out some ASCII (#1856)
- README ToC did not match contents (#1857)
- When using `--oss`, ensure correct configuration is threaded through correctly (#1859)
- Exit cleanly when ShutdownComplete is received (#1864)
- Fixes no git repo warning (#1863)

Fix broken git warning

<img width="797" height="482" alt="broken-screen"
src="https://github.com/user-attachments/assets/9c52ed9b-13d8-4f1d-bb37-7c51acac615d"
/>
- Fully enumerate EventMsg in chatwidget.rs (#1866)
- Only tag as prerelease when the version has an -alpha or -beta suffix (#1872)
- Add stricter checks and better error messages to create_github_release.sh (#1874)
- Add more instructions to ensure GitHub Action reviews only the necessary code (#1887)
- Support $CODEX_HOME/AGENTS.md instead of $CODEX_HOME/instructions.md (#1891)
- Fix meta+b meta+f (option+left/right) (#1895)

Option+Left or Option+Right should move cursor to beginning/end of the
word.

We weren't listening to what terminals are sending (on MacOS) and were
therefore printing b or f instead of moving cursor. We were actually in
the first match clause and returning char insertion
(https://github.com/openai/codex/pull/1895/files#diff-6bf130cd00438cc27a38c5a4d9937a27cf9a324c191de4b74fc96019d362be6dL209)

Tested on Apple Terminal, iTerm, Ghostty
- Fix cursor file name insert (#1896)

Cursor wasn't moving when inserting a file, resulting in being not at
the end of the filename when inserting the file.
This fixes it by moving the cursor to the end of the file + one trailing
space.


Example screenshot after selecting a file when typing `@`
<img width="823" height="268" alt="image"
src="https://github.com/user-attachments/assets/ec6e3741-e1ba-4752-89d2-11f14a2bd69f"
/>
- Change OPENAI_DEFAULT_MODEL to "gpt-5" (#1943)
- Fix mistaken bitwise OR in #1949 (#1957)
- Public load_auth() fn always called with include_env_var=true (#1961)
- Default to credits from ChatGPT auth, when possible (#1971)
- Stop building codex-exec and codex-linux-sandbox binaries (#2036)
- Try building the npm package in CI (#2043)
- Improve npm release process (#2055)
- Token usage display and context calculation (#2117)
- Change the model used with the GitHub action from o3 to gpt-5 (#2198)
- Remove unused import in release mode (#2201)
- Update ctrl-z to suspend tui (#2113)
- Take ExecToolCallOutput by value to avoid clone() (#2197)
- Display canonical command name in help (#2246)
- Remove behavioral prompting from update_plan tool def (#2261)
- Update `OutgoingMessageSender::send_response()` to take `Serialize` (#2263)
- Skip `cargo test` for release builds on ordinary CI because it is slow, particularly with --all-features set (#2276)
- Verify notifications are sent with the conversationId set (#2278)
- Make all fields of Session private (#2285)
- Add support for exec and apply_patch approvals in the new wire format (#2286)
- Parallelize logic in Session::new() (#2305)
- Do not allow dotenv to create/modify environment variables starting with CODEX_ (#2308)
- Fix bash commands being incorrectly quoted in display (#2313)

The "display format" of commands was sometimes producing incorrect
quoting like `echo foo '>' bar`, which is importantly different from the
actual command that was being run. This refactors ParsedCommand to have
a string in `cmd` instead of a vec, as a `vec` can't accurately capture
a full command.
- Move general sandbox tests to codex-rs/core/tests/sandbox.rs (#2317)
- Run python_multiprocessing_lock_works integration test on Mac and Linux (#2318)
- Add call_id to ApprovalParams in mcp-server/src/wire_format.rs (#2322)
- Ensure rust-ci always "runs" when a PR is submitted (#2324)
- Trying to simplify rust-ci.yml (#2327)
- Tighten up checks against writable folders for SandboxPolicy (#2338)
- Introduce MutexExt::lock_unchecked() so we stop ignoring unwrap() throughout codex.rs (#2340)
- Try to fix flakiness in test_shell_command_approval_triggers_elicitation (#2344)
- Introduce codex-protocol crate (#2355)
- Include an entry for windows-x86_64 in the generated DotSlash file (#2361)
- Refactor login/src/server.rs so process_request() is a separate function (#2388)
- Introduce EventMsg::TurnAborted (#2365)
- Clean up styles & colors and define in styles.md (#2401)
- Stop using ANSI blue (#2421)
- Async-ify login flow (#2393)
- Change `shutdown_flag` from `Arc<AtomicBool>` to `tokio::sync::Notify` (#2394)
- Eliminate ServerOptions.login_timeout and have caller use tokio::time::timeout() instead (#2395)
- Make ShutdownHandle a private field of LoginServer (#2396)
- Reduce references to Server in codex-login crate (#2398)
- Remove shutdown_flag param to run_login_server() (#2399)
- Exclude sysprompt etc from context left % (#2446)
- Fix missing spacing in review decision response (#2457)
- Prefer `cargo check` to `cargo build` to save time and space (#2466)
- Fix apply patch when only one file is rendered (#2468)

<img width="809" height="87" alt="image"
src="https://github.com/user-attachments/assets/6fe69643-10d7-4420-bbf2-e30c092b800f"
/>
- Prefer config var to env var (#2495)
- Update build cache key in .github/workflows/codex.yml (#2534)
- Prefer sending MCP structuredContent as the function call response, if available (#2594)
- Update gpt-5 stats (#2649)
- Build is broken on main; introduce ToolsConfigParams to help fix (#2663)
- Scope ExecSessionManager to Session instead of using global singleton (#2664)
- Use backslash as path separator on Windows (#2684)
- Fix issue #2713: adding support for alt+ctrl+h to delete backward word (#2717)

This pr addresses the fix for
https://github.com/openai/codex/issues/2713

### Changes:
  - Added key handler for `Alt+Ctrl+H` → `delete_backward_word()`
- Added test coverage in `delete_backward_word_alt_keys()` that verifies
both:
    - Standard `Alt+Backspace` binding continues to work
- New `Alt+Ctrl+H` binding works correctly for backward word deletion

### Testing:
  The test ensures both key combinations produce identical behavior:
  - Delete the previous word from "hello world" → "hello "
  - Cursor positioned correctly after deletion

###  Backward Compatibility:
This change is backward compatible - existing `Alt+Backspace`
functionality remains unchanged while adding support for the
terminal-specific `Alt+Ctrl+H` variant
- Fix transcript lines being added to diff view (#2721)

This fixes a bug where if you ran /diff while at turn was running,
transcript lines would be added to the end of the diff view. Also,
refactor to make this kind of issue less likely in future.
- Fix emoji spacing (#2735)

before:
<img width="295" height="266" alt="Screenshot 2025-08-26 at 5 05 03 PM"
src="https://github.com/user-attachments/assets/3e876f08-26d0-407e-a995-28fd072e288f"
/>

after:
<img width="295" height="129" alt="Screenshot 2025-08-26 at 5 05 30 PM"
src="https://github.com/user-attachments/assets/2a019d52-19ed-40ef-8155-4f02c400796a"
/>
- For now, limit the number of deltas sent back to the UI (#2776)
- Fix (most) doubled lines and hanging list markers (#2789)

This was mostly written by codex under heavy guidance via test cases
drawn from logged session data and fuzzing. It also uncovered some bugs
in tui_markdown, which will in some cases split a list marker from the
list item content. We're not addressing those bugs for now.
- Fix cursor after suspend (#2690)

This was supposed to be fixed by #2569, but I think the actual fix got
lost in the refactoring.

Intended behavior: pressing ^Z moves the cursor below the viewport
before suspending.
- Specify --profile to `cargo clippy` in CI (#2871)
- Switch to unbounded channel (#2874)
- Remove unnecessary flush() calls (#2873)
- Drop Mutex before calling tx_approve.send() (#2876)
- Drop Mutexes earlier in MCP server (#2878)
- Try to populate the Windows cache for release builds when PRs are put up for review (#2884)
- Leverage windows-11-arm for Windows ARM builds (#3062)
- Fix config reference table (#3063)

3 quick fixes to docs/config.md

- Fix the reference table so option lists render correctly
- Corrected the default `stream_max_retries` to 5 (Old: 10)
- Update example approval_policy to untrusted (Old: unless-allow-listed)
- Install zstd on the windows-11-arm image used to cut a release (#3066)
- Include arm64 Windows executable in npm module (#3067)
- Add callback to map before sending request to fix race condition (#3146)
- Use a more efficient wire format for ExecCommandOutputDeltaEvent.chunk (#3163)
- Fix serde_as annotation and verify with test (#3170)
- Change create_github_release to take either --publish-alpha or --publish-release (#3231)
- Improve MCP server initialization error handling #3196 #2346 #2555 (#3243)
- Include rollout_path in NewConversationResponse (#3352)
- LoginChatGptCompleteNotification does not need to be listed explicitly in protocol-ts (#3222)
- Remove unnecessary #[allow(dead_code)] annotation (#3357)
- Remove empty file: chatwidget_stream_tests.rs (#3356)
- Fix trampling projects table when accepting trusted dirs (#3434)

Co-authored-by: Codex <199175422+chatgpt-codex-connector[bot]@users.noreply.github.com>
- Ensure output of codex-rs/mcp-types/generate_mcp_types.py matches codex-rs/mcp-types/src/lib.rs (#3439)
- Add check to ensure output of generate_mcp_types.py matches codex-rs/mcp-types/src/lib.rs (#3450)
- Use -F instead of -f for force=true in gh call (#3486)
- Improve handle_sandbox_error timeouts (#3435)
- NIT None reasoning effort (#3536)
- *(core)* Flaky test `completed_commands_do_not_persist_sessions` (#3596)
- *(tui)* Update full-auto to default preset (#3608)
- Model family and apply_patch consistency (#3603)
- Race condition unified exec (#3644)
- Add references (#3633)
- Fix stray login url characters persisting in login (#3639)

<img width="885" height="177" alt="image"
src="https://github.com/user-attachments/assets/d396e0a5-f303-494f-bab1-f7af57b88a3e"
/>


Fixes this.
- Change MIN_ANIMATION_HEIGHT so show_animation is calculated correctly (#3656)
- Fix permissions alignment
- Read-only escalations (#3673)
- Record EnvironmentContext in SendUserTurn (#3678)
- Make GitHub Action publish to npm using trusted publishing (#3431)
- Ensure pnpm is installed before running `npm install` (#3763)
- Make publish-npm its own job with specific permissions (#3767)
- Specify --repo when calling gh (#3806)
- Some nit Rust reference issues (#3849)
- Fix error on missing notifications in [tui] (#3867)

Fixes #3811.
- Ensure cwd for conversation and sandbox are separate concerns (#3874)
- Update try_parse_word_only_commands_sequence() to return commands in order (#3881)
- Fix alignment in slash command popup (#3937)
- Fix codex resume message at end of session (#3957)

This was only being printed when running the codex-tui executable
directly, not via the codex-cli wrapper.
- Usage data tweaks (#4082)
- Vendor ripgrep in the npm module (#3660)
- `npm publish --tag alpha` when building an alpha release (#4112)
- Add tolerance for ambiguous behavior in `gh run list` (#4162)
- Esc w/ queued messages overwrites draft in composer (#4237)
- Fix bug when resizing to a smaller width (#4248)

The composer and key hint lines were using line styles, causing ratatui
to print spaces all the way to the right side of the terminal. this
meant that resizing the terminal to be narrower would result in
rewrapping those lines, causing the bottom area to rerender and push all
content up.

Before


https://github.com/user-attachments/assets/8b14555a-1fc5-4f78-8df7-1410ee25e07a

After


https://github.com/user-attachments/assets/707645ab-89c7-4c7f-b556-02f53cef8a2f
- Fix (#4251)

# External (non-OpenAI) Pull Request Requirements

Before opening this Pull Request, please read the dedicated
"Contributing" markdown file or your PR may be closed:
https://github.com/openai/codex/blob/main/docs/contributing.md

If your PR conforms to our contribution guidelines, replace this text
with a detailed and high quality description of your changes.
- Fix typo in sandbox doc (#4256)

just fixes a simple typo I noticed.
- Token usage for compaction (#4281)

### 💼 Other

- Update documentation to reflect Rust CLI release (#1440)
- Update release scripts for the TypeScript CLI (#1472)
- Fix Unicode handling in chat_composer "@" token detection (#1467)

## Issues Fixed

- **Primary Issue (#1450)**: Unicode cursor positioning was incorrect
due to mixing character positions with byte positions
- **Additional Issue**: Full-width spaces (CJK whitespace like "　")
weren't properly handled as token boundaries
- ref:
https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace

---------

Co-authored-by: Michael Bolin <bolinfest@gmail.com>
- Normalize repository.url in package.json (#1474)
- Update README to include `npm install` again (#1475)
- Create a release script for the Rust CLI (#1479)
- Default to the latest version of the Codex CLI in the GitHub Action (#1485)
- Document support for model_reasoning_effort and model_reasoning_summary in profiles (#1486)
- Add Android platform support for Codex CLI (#1488)

## Summary
Add Android platform support to Codex CLI

## What?
- Added `android` to the list of supported platforms in
`codex-cli/bin/codex.js`
- Treats Android as Linux for binary compatibility

## Why?
- Fixes "Unsupported platform: android (arm64)" error on Termux
- Enables Codex CLI usage on Android devices via Termux
- Improves platform compatibility without affecting other platforms

## How?
- Modified the platform detection switch statement to include `case
"android":`
- Android falls through to the same logic as Linux, using appropriate
ARM64 binaries
- Minimal change with no breaking effects on existing functionality

## Testing
- Tested on Android/Termux environment
- Verified the fix resolves the platform detection error
- Confirmed no impact on other platforms

## Related Issues
Fixes the "Unsupported platform: android (arm64)" error reported by
Termux users
- *(rs)* Update dependencies (#1494)
- Drop codex-cli from dependabot (#1523)
- *(deps)* Bump toml from 0.9.0 to 0.9.1 in /codex-rs (#1514)
- *(deps)* Bump node from 22-slim to 24-slim in /codex-cli (#1505)
- *(deps-dev)* Bump prettier from 3.5.3 to 3.6.2 in /.github/actions/codex (#1508)
- Read model field off of Config instead of maintaining the parallel field (#1525)
- Add `codex apply` to apply a patch created from the Codex remote agent (#1528)

In order to to this, I created a new `chatgpt` crate where we can put
any code that interacts directly with ChatGPT as opposed to the OpenAI
API. I added a disclaimer to the README for it that it should primarily
be modified by OpenAI employees.


https://github.com/user-attachments/assets/bb978e33-d2c9-4d8e-af28-c8c25b1988e8
- *(deps-dev)* Bump @types/node from 22.15.21 to 24.0.12 in /.github/actions/codex (#1507)
- *(deps-dev)* Bump @types/bun from 1.2.13 to 1.2.18 in /.github/actions/codex (#1509)
- Add paste summarization to Codex TUI (#1549)

## Summary
- introduce `Paste` event to avoid per-character paste handling
- collapse large pasted blocks to `[Pasted Content X lines]`
- store the real text so submission still includes it
- wire paste handling through `App`, `ChatWidget`, `BottomPane`, and
`ChatComposer`

## Testing
- `cargo test -p codex-tui`


------
https://chatgpt.com/codex/tasks/task_i_6871e24abf80832184d1f3ca0c61a5ee


https://github.com/user-attachments/assets/eda7412f-da30-4474-9f7c-96b49d48fbf8
- Improve SSE tests (#1546)

## Summary
- support fixture-based SSE data in tests
- add helpers to load SSE JSON fixtures
- add table-driven SSE unit tests
- let integration tests use fixture loading
- fix clippy errors from format! calls

## Testing
- `cargo clippy --tests`
- `cargo test --workspace --exclude codex-linux-sandbox`


------
https://chatgpt.com/codex/tasks/task_i_68717468c3e48321b51c9ecac6ba0f09
- Allow deadcode in test_support (#1555)

#1546 Was pushed while not passing the clippy integration tests. This is
fixing it.
- Add CLI streaming integration tests (#1542)

## Summary
- add integration test for chat mode streaming via CLI using wiremock
- add integration test for Responses API streaming via fixture
- call `cargo run` to invoke the CLI during tests

## Testing
- `cargo test -p codex-core --test cli_stream -- --nocapture`
- `cargo clippy --all-targets --all-features -- -D warnings`


------
https://chatgpt.com/codex/tasks/task_i_68715980bbec8321999534fdd6a013c1
- Clarify the build process for the npm release (#1568)
- Add SSE Response parser tests (#1541)

## Summary
- add `tokio-test` dev dependency
- implement response stream parsing unit tests

## Testing
- `cargo clippy -p codex-core --tests -- -D warnings`
- `cargo test -p codex-core -- --nocapture`

------
https://chatgpt.com/codex/tasks/task_i_687163f3b2208321a6ce2adbef3fbc06
- Support deltas in core (#1587)

- Added support for message and reasoning deltas
- Skipped adding the support in the cli and tui for later
- Commented a failing test (wrong merge) that needs fix in a separate
PR.

Side note: I think we need to disable merge when the CI don't pass.
- Added mcp-server name validation (#1591)

This PR implements server name validation for MCP (Model Context
Protocol) servers to ensure they conform to the required pattern
^[a-zA-Z0-9_-]+$. This addresses the TODO comment in
mcp_connection_manager.rs:82.

+ Added validation before spawning MCP client tasks
+ Invalid server names are added to errors map with descriptive messages

I have read the CLA Document and I hereby sign the CLA

---------

Co-authored-by: Michael Bolin <bolinfest@gmail.com>
- Add streaming to exec and tui (#1594)

Added support for streaming in `tui`
Added support for streaming in `exec`


https://github.com/user-attachments/assets/4215892e-d940-452c-a1d0-416ed0cf14eb
- Storing the sessions in a more organized way for easier look up. (#1596)

now storing the sessions in `~/.codex/sessions/YYYY/MM/DD/<file>`
- Auto format code on save and add more details to AGENTS.md (#1582)
- Implement redraw debounce (#1599)

## Summary
- debouce redraw events so repeated requests don't overwhelm the
terminal
- add `RequestRedraw` event and schedule redraws after 100ms

## Testing
- `cargo clippy --tests`
- `cargo test` *(fails: Sandbox Denied errors in landlock tests)*

------
https://chatgpt.com/codex/tasks/task_i_68792a65b8b483218ec90a8f68746cd8

---------

Co-authored-by: Michael Bolin <mbolin@openai.com>
- Rename toolchain file (#1604)
- Change the default debounce rate to 10ms (#1606)

changed the default debounce rate to 10ms because typing was laggy.

Before:


https://github.com/user-attachments/assets/e5d15fcb-6a2b-4837-b2b4-c3dcb4cc3409

After



https://github.com/user-attachments/assets/6f0005eb-fd49-4130-ba68-635ee0f2831f
- Use AtomicBool instead of Mutex<bool> (#1616)
- Fix ctrl+c interrupt while streaming (#1617)

Interrupting while streaming now causes is broken because we aren't
clearing the delta buffer.
- Refactor env settings into config (#1601)

## Summary
- add OpenAI retry and timeout fields to Config
- inject these settings in tests instead of mutating env vars
- plumb Config values through client and chat completions logic
- document new configuration options

## Testing
- `cargo test -p codex-core --no-run`

------
https://chatgpt.com/codex/tasks/task_i_68792c5b04cc832195c03050c8b6ea94

---------

Co-authored-by: Michael Bolin <mbolin@openai.com>
- Add session loading support to Codex (#1602)

## Summary
- extend rollout format to store all session data in JSON
- add resume/write helpers for rollouts
- track session state after each conversation
- support `LoadSession` op to resume a previous rollout
- allow starting Codex with an existing session via
`experimental_resume` config variable

We need a way later for exploring the available sessions in a user
friendly way.

## Testing
- `cargo test --no-run` *(fails: `cargo: command not found`)*

------
https://chatgpt.com/codex/tasks/task_i_68792a29dd5c832190bf6930d3466fba

This video is outdated. you should use `-c experimental_resume:<full
path>` instead of `--resume <full path>`


https://github.com/user-attachments/assets/7a9975c7-aa04-4f4e-899a-9e87defd947a
- Clean up generate_mcp_types.py so codegen matches existing output (#1620)
- Support MCP schema 2025-06-18 (#1621)
- Introduce OutgoingMessageSender (#1622)
- Don't drop sessions on elicitation responses (#1629)
- Add integration test for MCP server (#1633)
- [mcp-server] Add reply tool call (#1643)

## Summary
Adds a new mcp tool call, `codex-reply`, so we can continue existing
sessions. This is a first draft and does not yet support sessions from
previous processes.

## Testing
- [x] tested with mcp client
- Add an elicitation for approve patch and refactor tool calls (#1642)

1. Added an elicitation for `approve-patch` which is very similar to
`approve-exec`.
2. Extracted both elicitations to their own files to prevent
`codex_tool_runner` from blowing up in size.
- Log response.failed error message and request-id (#1649)

To help with diagnosing failures.
- Add support for custom base instructions (#1645)

Allows providing custom instructions file as a config parameter and
custom instruction text via MCP tool call.
- Install an extension for TOML syntax highlighting in the devcontainer (#1650)
- Adding interrupt Support to MCP (#1646)
- For release build, build specific targets instead of --all-targets (#1656)
- Always send entire request context (#1641)

Always store the entire conversation history.
Request encrypted COT when not storing Responses.
Send entire input context instead of sending previous_response_id
- Improve messages emitted for exec failures (#1659)

1. Emit call_id to exec approval elicitations for mcp client convenience
2. Remove the `-retry` from the call id for the same reason as above but
upstream the reset behavior to the mcp client
- Add call_id to patch approvals and elicitations (#1660)

Builds on https://github.com/openai/codex/pull/1659 and adds call_id to
a few more places for the same reason.
- Flaky CI fix (#1647)

Flushing before sending `TaskCompleteEvent` and ending the submission
loop to avoid race conditions.
- *(deps-dev)* Bump @types/bun from 1.2.18 to 1.2.19 in /.github/actions/codex (#1635)
- *(deps-dev)* Bump @types/node from 24.0.13 to 24.0.15 in /.github/actions/codex (#1636)
- *(deps)* Bump rand from 0.9.1 to 0.9.2 in /codex-rs (#1637)
- Fix flaky test (#1664)

Co-authored-by: aibrahim-oai <aibrahim@openai.com>
- *(deps)* Bump strum from 0.27.1 to 0.27.2 in /codex-rs (#1639)
- *(deps)* Bump strum_macros from 0.27.1 to 0.27.2 in /codex-rs (#1638)
- *(deps)* Bump tree-sitter from 0.25.6 to 0.25.8 in /codex-rs (#1561)
- *(deps)* Bump toml from 0.9.1 to 0.9.2 in /codex-rs (#1562)
- Record Git metadata to rollout (#1598)

# Summary

- Writing effective evals for codex sessions requires context of the
overall repository state at the moment the session began
- This change adds this metadata (git repository, branch, commit hash)
to the top of the rollout of the session (if available - if not it
doesn't add anything)
- Currently, this is only effective on a clean working tree, as we can't
track uncommitted/untracked changes with the current metadata set.
Ideally in the future we may want to track unclean changes somehow, or
perhaps prompt the user to stash or commit them.

# Testing
- Added unit tests
- `cargo test && cargo clippy --tests && cargo fmt -- --config
imports_granularity=Item`

### Resulting Rollout
<img width="1243" height="127" alt="Screenshot 2025-07-17 at 1 50 00 PM"
src="https://github.com/user-attachments/assets/68108941-f015-45b2-985c-ea315ce05415"
/>
- Update render name in tui for approval_policy to match with config values  (#1675)

Currently, codex on start shows the value for the approval policy as
name of
[AskForApproval](https://github.com/openai/codex/blob/2437a8d17a0cf972d1a6e7f303d469b6e2f57eae/codex-rs/core/src/protocol.rs#L128)
enum, which differs from
[approval_policy](https://github.com/openai/codex/blob/2437a8d17a0cf972d1a6e7f303d469b6e2f57eae/codex-rs/config.md#approval_policy)
config values.
E.g. "untrusted" becomes "UnlessTrusted", "on-failure" -> "OnFailure",
"never" -> "Never".
This PR changes render names of the approval policy to match with
configuration values.
- Easily Selectable History (#1672)

This update replaces the previous ratatui history widget with an
append-only log so that the terminal can handle text selection and
scrolling. It also disables streaming responses, which we'll do our best
to bring back in a later PR. It also adds a small summary of token use
after the TUI exits.
- Use one write call per item in rollout_writer() (#1679)
- Optionally run using user profile (#1678)
- Changing method in MCP notifications (#1684)

- Changing the codex/event type
- Remove tab focus switching (#1694)

Previously pressing tab would switch TUI focus to the history scrollbox - no longer necessary.
- Update Codex::spawn() to return a struct instead of a tuple (#1677)
- Split apply_patch logic out of codex.rs and into apply_patch.rs (#1703)
- Serializing the `eventmsg` type to snake_case (#1709)

This was an abrupt change on our clients. We need to serialize as
snake_case.
- Fix approval workflow (#1696)

(Hopefully) temporary solution to the invisible approvals problem -
prints commands to history when they need approval and then also prints
the result of the approval. In the near future we should be able to do
some fancy stuff with updating commands before writing them to permanent
history.

Also, ctr-c while in the approval modal now acts as esc (aborts command)
and puts the TUI in the state where one additional ctr-c will exit.
- [mcp-server] Populate notifications._meta with requestId (#1704)

## Summary
Per the [latest MCP
spec](https://modelcontextprotocol.io/specification/2025-06-18/basic#meta),
the `_meta` field is reserved for metadata. In the [Typescript
Schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/0695a497eb50a804fc0e88c18a93a21a675d6b3e/schema/2025-06-18/schema.ts#L37-L40),
`progressToken` is defined as a value to be attached to subsequent
notifications for that request.

The
[CallToolRequestParams](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/0695a497eb50a804fc0e88c18a93a21a675d6b3e/schema/2025-06-18/schema.ts#L806-L817)
extends this definition but overwrites the params field. This ambiguity
makes our generated type definitions tricky, so I'm going to skip
`progressToken` field for now and just send back the `requestId`
instead.
 
In a future PR, we can clarify, update our `generate_mcp_types.py`
script, and update our progressToken logic accordingly.

## Testing
- [x] Added unit tests
- [x] Manually tested with mcp client
- Replace login screen with a simple prompt (#1713)

Perhaps there was an intention to make the login screen prettier, but it
feels quite silly right now to just have a screen that says "press q",
so replace it with something that lets the user directly login without
having to quit the app.

<img width="1283" height="635" alt="Screenshot 2025-07-28 at 2 54 05 PM"
src="https://github.com/user-attachments/assets/f19e5595-6ef9-4a2d-b409-aa61b30d3628"
/>
- Alternate login wording? (#1723)

Co-authored-by: Jeremy Rose <172423086+nornagon-openai@users.noreply.github.com>
- Relative instruction file (#1722)

Passing in an instruction file with a bad path led to silent failures,
also instruction relative paths were handled in an unintuitive fashion.
- Add an experimental plan tool (#1726)

This adds a tool the model can call to update a plan. The tool doesn't
actually _do_ anything but it gives clients a chance to read and render
the structured plan. We will likely iterate on the prompt and tools
exposed for planning over time.
- Trim bash lc and run with login shell (#1725)

include .zshenv, .zprofile by running with the `-l` flag and don't start
a shell inside a shell when we see the typical `bash -lc` invocation.
- Mcp protocol (#1715)

- Add typed MCP protocol surface in
`codex-rs/mcp-server/src/mcp_protocol.rs` for `requests`, `responses`,
and `notifications`
- Requests: `NewConversation`, `Connect`, `SendUserMessage`,
`GetConversations`
- Message content parts: `Text`, `Image` (`ImageUrl`/`FileId`, optional
`ImageDetail`), File (`Url`/`Id`/`inline Data`)
- Responses: `ToolCallResponseEnvelope` with optional `isError` and
`structuredContent` variants (`NewConversation`, `Connect`,
`SendUserMessageAccepted`, `GetConversations`)
- Notifications: `InitialState`, `ConnectionRevoked`, `CodexEvent`,
`Cancelled`
- Uniform `_meta` on `notifications` via `NotificationMeta`
(`conversationId`, `requestId`)
- Unit tests validate JSON wire shapes for key
`requests`/`responses`/`notifications`
- Remove conversation history widget (#1727)

this widget is no longer used.
- Add support for a separate chatgpt auth endpoint (#1712)

Adds a `CodexAuth` type that encapsulates information about available
auth modes and logic for refreshing the token.
Changes `Responses` API to send requests to different endpoints based on
the auth type.
Updates login_with_chatgpt to support API-less mode and skip the key
exchange.
- Moving input item from MCP Protocol back to core Protocol (#1740)

- Currently we have duplicate input item. Let's have one source of truth
in the core.
- Used Requestid type
- Send AGENTS.md as a separate user message (#1737)
- Add login status command (#1716)

Print the current login mode, sanitized key and return an appropriate
status.
- Resizable viewport (#1732)

Proof of concept for a resizable viewport.

The general approach here is to duplicate the `Terminal` struct from
ratatui, but with our own logic. This is a "light fork" in that we are
still using all the base ratatui functions (`Buffer`, `Widget` and so
on), but we're doing our own bookkeeping at the top level to determine
where to draw everything.

This approach could use improvement—e.g, when the window is resized to a
smaller size, if the UI wraps, we don't correctly clear out the
artifacts from wrapping. This is possible with a little work (i.e.
tracking what parts of our UI would have been wrapped), but this
behavior is at least at par with the existing behavior.


https://github.com/user-attachments/assets/4eb17689-09fd-4daa-8315-c7ebc654986d


cc @joshka who might have Thoughts™
- Add support for a new label, codex-rust-review (#1744)
- Auto format toml (#1745)

Add recommended extension and configure it to auto format prompt.
- Add keyboard enhancements to support shift_return (#1743)

For terminal that supports [keyboard
enhancements](https://docs.rs/libcrossterm/latest/crossterm/enum.KeyboardEnhancementFlags.html),
adds the enhancements (enabling [kitty keyboard
protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)) to
support shift+enter listener.

Those users (users with terminals listed on
[KPP](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)) should be
able to press shift+return for new line

---------

Co-authored-by: easong-openai <easong@openai.com>
- Streamline ui (#1733)

Simplify and improve many UI elements.
* Remove all-around borders in most places. These interact badly with
terminal resizing and look heavy. Prefer left-side-only borders.
* Make the viewport adjust to the size of its contents.
* <kbd>/</kbd> and <kbd>@</kbd> autocomplete boxes appear below the
prompt, instead of above it.
* Restyle the keyboard shortcut hints & move them to the left.
* Restyle the approval dialog.
* Use synchronized rendering to avoid flashing during rerenders.


https://github.com/user-attachments/assets/96f044af-283b-411c-b7fc-5e6b8a433c20

<img width="1117" height="858" alt="Screenshot 2025-07-30 at 5 29 20 PM"
src="https://github.com/user-attachments/assets/0cc0af77-8396-429b-b6ee-9feaaccdbee7"
/>
- Show error message after panic (#1752)

Previously we were swallowing errors and silently exiting, which isn't
great for helping users help us.
- Clamp render area to terminal size (#1758)

this fixes a couple of panics that would happen when trying to render
something larger than the terminal, or insert history lines when the top
of the viewport is at y=0.
- Add codex login --api-key (#1759)

Allow setting the API key via `codex login --api-key`
- Fix double-scrolling in approval model (#1754)

Previously, pressing up or down arrow in the new approval modal would be
the equivalent of two up or down presses.
- Refactor exec.rs: create separate seatbelt.rs and spawn.rs files (#1762)
- Initial planning tool (#1753)

We need to optimize the prompt, but this causes the model to use the new
planning_tool.

<img width="765" height="110" alt="image"
src="https://github.com/user-attachments/assets/45633f7f-3c85-4e60-8b80-902f1b3b508d"
/>
- Send account id when available (#1767)

For users with multiple accounts we need to specify the account to use.
- Do not dispatch key releases (#1771)

when we enabled KKP in https://github.com/openai/codex/pull/1743, we
started receiving keyup events, but didn't expect them anywhere in our
code. for now, just don't dispatch them at all.
- Lighter approval modal (#1768)

The yellow hazard stripes were too scary :)

This also has the added benefit of not rendering anything at the full
width of the terminal, so resizing is a little easier to handle.

<img width="860" height="390" alt="Screenshot 2025-07-31 at 4 03 29 PM"
src="https://github.com/user-attachments/assets/18476e1a-065d-4da9-92fe-e94978ab0fce"
/>

<img width="860" height="390" alt="Screenshot 2025-07-31 at 4 05 03 PM"
src="https://github.com/user-attachments/assets/337db0da-de40-48c6-ae71-0e40f24b87e7"
/>
- Insert history lines with redraw (#1769)

This delays the call to insert_history_lines until a redraw is
happening. Crucially, the new lines are inserted _after the viewport is
resized_. This results in fewer stray blank lines below the viewport
when modals (e.g. user approval) are closed.
- Detect kitty terminals (#1748)

We want to detect kitty terminals so we can preferentially upgrade their UX without degrading older terminals.
- MCP Protocol: Align tool-call response with CallToolResult [Stack 1/3] (#1750)

# Summary
- Align MCP server responses with mcp_types by emitting [CallToolResult,
RequestId] instead of an object.
Update send-message result to a tagged enum: Ok or Error { message }.

# Why
Protocol compliance with current MCP schema.

# Tests
- Updated assertions in mcp_protocol.rs for create/stream/send/list and
error cases.

This is the first PR in a stack.
Stack:
Final: #1686
Intermediate: #1751
First: #1750
- MCP server: route structured tool-call requests and expose mcp_protocol [Stack 2/3] (#1751)

- Expose mcp_protocol from mcp-server for reuse in tests and callers.
- In MessageProcessor, detect structured ToolCallRequestParams in
tools/call and forward to a new handler.
- Add handle_new_tool_calls scaffold (returns error for now).
- Test helper: add send_send_user_message_tool_call to McpProcess to
send ConversationSendMessage requests;

This is the second PR in a stack.
Stack:
Final: #1686
Intermediate: #1751
First: #1750
- Add /compact (#1527)

- Add operation to summarize the context so far.
- The operation runs a compact task that summarizes the context.
- The operation clear the previous context to free the context window
- The operation didn't use `run_task` to avoid corrupting the session
- Add /compact in the tui



https://github.com/user-attachments/assets/e06c24e5-dcfb-4806-934a-564d425a919c
- Ci fix (#1782)
- Add a custom originator setting (#1781)
- Introduce a new function to just send user message [Stack 3/3] (#1686)

- MCP server: add send-user-message tool to send user input to a running
Codex session
- Added an integration tests for the happy and sad paths

Changes:
•	Add tool definition and schema.
•	Expose tool in capabilities.
•	Route and handle tool requests with validation.
•	Tests for success, bad UUID, and missing session.


follow‑ups
• Listen path not implemented yet; the tool is present but marked “don’t
use yet” in code comments.
• Session run flag reset: clear running_session_id_set appropriately
after turn completion/errors.

This is the third PR in a stack.
Stack:
Final: #1686
Intermediate: #1751
First: #1750
- Collabse `stdout` and `stderr` delta events into one (#1787)
- Introduce SandboxPolicy::WorkspaceWrite::include_default_writable_roots (#1785)
- Add Error variant to ConversationCreateResult [Stack 1/2] (#1784)

Switch ConversationCreateResult from a struct to a tagged enum (Ok |
Error)

Stack:
Top: #1783 
Bottom: #1784
- Add conversation.create tool [Stack 2/2] (#1783)
- Update succesfull login page look (#1789)
- Check for updates (#1764)

1. Ping https://api.github.com/repos/openai/codex/releases/latest (at
most once every 20 hrs)
2. Store the result in ~/.codex/version.jsonl
3. If CARGO_PKG_VERSION < latest_version, print a message at boot.

---------

Co-authored-by: easong-openai <easong@openai.com>
- Fix compact (#1798)

We are not recording the summary in the history.
- Fix MacOS multiprocessing by relaxing sandbox (#1808)

The following test script fails in the codex sandbox:
```
import multiprocessing
from multiprocessing import Lock, Process

def f(lock):
    with lock:
        print("Lock acquired in child process")

if __name__ == '__main__':
    lock = Lock()
    p = Process(target=f, args=(lock,))
    p.start()
    p.join()
```

with 
```
Traceback (most recent call last):
  File "/Users/david.hao/code/codex/codex-rs/cli/test.py", line 9, in <module>
    lock = Lock()
           ^^^^^^
  File "/Users/david.hao/.local/share/uv/python/cpython-3.12.9-macos-aarch64-none/lib/python3.12/multiprocessing/context.py", line 68, in Lock
    return Lock(ctx=self.get_context())
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/david.hao/.local/share/uv/python/cpython-3.12.9-macos-aarch64-none/lib/python3.12/multiprocessing/synchronize.py", line 169, in __init__
    SemLock.__init__(self, SEMAPHORE, 1, 1, ctx=ctx)
  File "/Users/david.hao/.local/share/uv/python/cpython-3.12.9-macos-aarch64-none/lib/python3.12/multiprocessing/synchronize.py", line 57, in __init__
    sl = self._semlock = _multiprocessing.SemLock(
                         ^^^^^^^^^^^^^^^^^^^^^^^^^
PermissionError: [Errno 1] Operation not permitted
```

After reading, adding this line to the sandbox configs fixes things -
MacOS multiprocessing appears to use sem_lock(), which opens an IPC
which is considered a disk write even though no file is created. I
interrogated ChatGPT about whether it's okay to loosen, and my
impression after reading is that it is, although would appreciate a
close look


Breadcrumb: You can run `cargo run -- debug seatbelt --full-auto <cmd>`
to test the sandbox
- Fix flaky test_shell_command_approval_triggers_elicitation test (#1802)

This doesn't flake very often but this should fix it.
- Custom textarea (#1794)

This replaces tui-textarea with a custom textarea component.

Key differences:
1. wrapped lines
2. better unicode handling
3. uses the native terminal cursor

This should perhaps be spun out into its own separate crate at some
point, but for now it's convenient to have it in-tree.
- Shimmer on working (#1807)

change the animation on "working" to be a text shimmer


https://github.com/user-attachments/assets/f64529eb-1c64-493a-8d97-0f68b964bdd0
- [sandbox] Filter out certain non-sandbox errors (#1804)

## Summary
Users frequently complain about re-approving commands that have failed
for non-sandbox reasons. We can't diagnose with complete accuracy which
errors happened because of a sandbox failure, but we can start to
eliminate some common simple cases.

This PR captures the most common case I've seen, which is a `command not
found` error.

## Testing
- [x] Added unit tests
- [x] Ran a few cases locally
- Add a TurnDiffTracker to create a unified diff for an entire turn (#1770)

This lets us show an accumulating diff across all patches in a turn.
Refer to the docs for TurnDiffTracker for implementation details.

There are multiple ways this could have been done and this felt like the
right tradeoff between reliability and completeness:
*Pros*
* It will pick up all changes to files that the model touched including
if they prettier or another command that updates them.
* It will not pick up changes made by the user or other agents to files
it didn't modify.

*Cons*
* It will pick up changes that the user made to a file that the model
also touched
* It will not pick up changes to codegen or files that were not modified
with apply_patch
- Update prompt.md (#1819)

The existing prompt is really bad. As a low-hanging fruit, let's correct
the apply_patch instructions - this helps smaller models successfully
apply patches.
- Support more keys in textarea (#1820)

Added:
* C-m for newline (not sure if this is actually treated differently to
Enter, but tui-textarea handles it and it doesn't hurt)
* C-d to delete one char forwards (same as Del)
* A-bksp to delete backwards one word
* A-arrows to navigate by word
- *(deps)* Bump toml from 0.9.2 to 0.9.4 in /codex-rs (#1815)
- *(deps-dev)* Bump typescript from 5.8.3 to 5.9.2 in /.github/actions/codex (#1814)
- *(deps)* Bump serde_json from 1.0.141 to 1.0.142 in /codex-rs (#1817)
- *(deps)* Bump tokio from 1.46.1 to 1.47.1 in /codex-rs (#1816)
- [codex] stop printing error message when --output-last-message is not specified (#1828)

Previously, `codex exec` was printing `Warning: no file to write last
message to` as a warning to stderr even though `--output-last-message`
was not specified, which is wrong. This fixes the code and changes
`handle_last_message()` so that it is only called when
`last_message_path` is `Some`.
- Add raw reasoning
- Unify flag
- Revert to 3f13ebce10209ab3645f51e7606892b3fd71d47e without rewriting history. Wrong merge
- Restore API key and query param overrides (#1826)

Addresses https://github.com/openai/codex/issues/1796
- Request the simplified auth flow (#1834)
- [prompts] Better user_instructions handling (#1836)

## Summary
Our recent change in #1737 can sometimes lead to the model confusing
AGENTS.md context as part of the message. But a little prompting and
formatting can help fix this!

## Testing
- Ran locally with a few different prompts to verify the model
behaves well.
- Updated unit tests
- Stream model responses (#1810)

Stream models thoughts and responses instead of waiting for the whole
thing to come through. Very rough right now, but I'm making the risk call to push through.
- Introduce ModelFamily abstraction (#1838)
- [prompt] Update prompt.md (#1839)

## Summary
Additional clarifications to our prompt. Still very concise, but we'll
continue to add more here.
- Rescue chat completion changes (#1846)

https://github.com/openai/codex/pull/1835 has some messed up history.

This adds support for streaming chat completions, which is useful for ollama. We should probably take a very skeptical eye to the code introduced in this PR.

---------

Co-authored-by: Ahmed Ibrahim <aibrahim@openai.com>
- Introduce `--oss` flag to use gpt-oss models (#1848)

This adds support for easily running Codex backed by a local Ollama
instance running our new open source models. See
https://github.com/openai/gpt-oss for details.

If you pass in `--oss` you'll be prompted to install/launch ollama, and
it will automatically download the 20b model and attempt to use it.

We'll likely want to expand this with some options later to make the
experience smoother for users who can't run the 20b or want to run the
120b.

Co-authored-by: Michael Bolin <mbolin@openai.com>
- Remove unnecessary default_ prefix (#1854)
- [feat] make approval key matching case insensitive (#1862)
- [core] Stop escalating timeouts (#1853)

## Summary
Escalating out of sandbox is (almost always) not going to fix
long-running commands timing out - therefore we should just pass the
failure back to the model instead of asking the user to re-run a command
that took a long time anyway.

## Testing
- [x] Ran locally with a timeout and confirmed this worked as expected
- [core] Separate tools config from openai client (#1858)

## Summary
In an effort to make tools easier to work with and more configurable,
I'm introducing `ToolConfig` and updating `Prompt` to take in a general
list of Tools. I think this is simpler and better for a few reasons:
- We can easily assemble tools from various sources (our own harness,
mcp servers, etc.) and we can consolidate the logic for constructing the
logic in one place that is separate from serialization.
- client.rs no longer needs arbitrary config values, it just takes in a
list of tools to serialize

A hefty portion of the PR is now updating our conversion of
`mcp_types::Tool` to `OpenAITool`, but considering that @bolinfest
accurately called this out as a TODO long ago, I think it's time we
tackled it.

## Testing
- [x] Experimented locally, no changes, as expected
- [x] Added additional unit tests
- [x] Responded to rust-review
- [approval_policy] Add OnRequest approval_policy (#1865)

## Summary
A split-up PR of #1763 , stacked on top of a tools refactor #1858 to
make the change clearer. From the previous summary:

> Let's try something new: tell the model about the sandbox, and let it
decide when it will need to break the sandbox. Some local testing
suggests that it works pretty well with zero iteration on the prompt!

## Testing
- [x] Added unit tests
- [x] Tested locally and it appears to work smoothly!
- Remove Turndiff and Apply patch from the render (#1868)

Make the tui more specific on what to render. Apply patch End and Turn
diff needs special handling.

Avoiding this issue:

<img width="503" height="138" alt="image"
src="https://github.com/user-attachments/assets/4c010ea8-701e-46d2-aa49-88b37fe0e5d9"
/>
- Clear terminal on launch (#1870)
- Add OSS model info (#1860)

Add somewhat arbitrarily chosen context window/output limit.
- Tweak comment (#1871)

Belatedly address CR feedback about a comment.

------
https://chatgpt.com/codex/tasks/task_i_6892e8070be4832cba379f2955f5b8bc
- Add more detail to the codex-rust-review (#1875)
- [feat] add /status slash command (#1873)

- Added a `/status` command, which will be useful when we update the
home screen to print less status.
- Moved `create_config_summary_entries` to common since it's used in a
few places.
- Noticed we inconsistently had periods in slash command descriptions
and just removed them everywhere.
- Noticed the diff description was overflowing so made it shorter.
- [tests] Investigate flakey mcp-server test (#1877)

## Summary
Have seen these tests flaking over the course of today on different
boxes. `wiremock` seems to be generally written with tokio/threads in
mind but based on the weird panics from the tests, let's see if this
helps.
- [prompts] Add <environment_context> (#1869)

## Summary
Includes a new user message in the api payload which provides useful
environment context for the model, so it knows about things like the
current working directory and the sandbox.

## Testing
Updated unit tests
- [env] Remove git config for now (#1884)

## Summary
Forgot to remove this in #1869 last night! Too much of a performance hit
on the main thread. We can bring it back via an async thread on startup.
- Initial implementation of /init (#1822)

Basic /init command that appends an instruction to create AGENTS.md to
the conversation history.
- Rename INIT.md to prompt_for_init_command.md and move closer to usage (#1886)
- Show a transient history cell for commands (#1824)

Adds a new "active history cell" for history bits that need to render
more than once before they're inserted into the history. Only used for
commands right now.


https://github.com/user-attachments/assets/925f01a0-e56d-4613-bc25-fdaa85d8aea5

---------

Co-authored-by: easong-openai <easong@openai.com>
- Prefer env var auth over default codex auth (#1861)

## Summary
- Prioritize provider-specific API keys over default Codex auth when
building requests
- Add test to ensure provider env var auth overrides default auth

## Testing
- `just fmt`
- `just fix` *(fails: `let` expressions in this position are unstable)*
- `cargo test --all-features` *(fails: `let` expressions in this
position are unstable)*

------
https://chatgpt.com/codex/tasks/task_i_68926a104f7483208f2c8fd36763e0e3
- Propagate apply_patch filesystem errors (#1892)

## Summary
We have been returning `exit code 0` from the apply patch command when
writes fail, which causes our `exec` harness to pass back confusing
messages to the model. Instead, we should loudly fail so that the
harness and the model can handle these errors appropriately.

Also adds a test to confirm this behavior.

## Testing
- `cargo test -p codex-apply-patch`
- First pass at a TUI onboarding (#1876)

This sets up the scaffolding and basic flow for a TUI onboarding
experience. It covers sign in with ChatGPT, env auth, as well as some
safety guidance.

Next up:
1. Replace the git warning screen
2. Use this to configure default approval/sandbox modes


Note the shimmer flashes are from me slicing the video, not jank.

https://github.com/user-attachments/assets/0fbe3479-fdde-41f3-87fb-a7a83ab895b8
- Add 2025-08-06 model family (#1899)
- Run command UI (#1897)

Edit how commands show:

<img width="243" height="119" alt="image"
src="https://github.com/user-attachments/assets/13d5608e-3b66-4b8d-8fe7-ce464310d85d"
/>
- Migrate GitWarning to OnboardingScreen (#1915)

This paves the way to do per-directory approval settings
(https://github.com/openai/codex/pull/1912).

This also lets us pass in a Config/ChatWidgetArgs into onboarding which
can then mutate it and emit the ChatWidgetArgs it wants at the end which
may be modified by the said approval settings.

<img width="1180" height="428" alt="CleanShot 2025-08-06 at 19 30 55"
src="https://github.com/user-attachments/assets/4dcfda42-0f5e-4b6d-a16d-2597109cc31c"
/>
- Show timing and token counts in status indicator (#1909)

## Summary
- track start time and cumulative tokens in status indicator
- display dim "(Ns • N tokens • Ctrl z to interrupt)" text after
animated Working header
- propagate token usage updates to status indicator views



https://github.com/user-attachments/assets/b73210c1-1533-40b5-b6c2-3c640029fd54


## Testing
- `just fmt`
- `just fix` *(fails: let expressions in this position are unstable)*
- `cargo test --all-features` *(fails: let expressions in this position
are unstable)*

------
https://chatgpt.com/codex/tasks/task_i_6893ec0d74a883218b94005172d7bc4c
- Scrollable slash commands (#1830)

Scrollable slash commands. Part 1 of the multi PR.
- Change the UI of apply patch (#1907)

<img width="487" height="108" alt="image"
src="https://github.com/user-attachments/assets/3f6ffd56-36f6-40bc-b999-64279705416a"
/>

---------

Co-authored-by: Gabriel Peal <gpeal@users.noreply.github.com>
- Ensure exec command end always emitted (#1908)

## Summary
- defer ExecCommandEnd emission until after sandbox resolution
- make sandbox error handler return final exec output and response
- align sandbox error stderr with response content and rename to
`final_output`
- replace unstable `let` chains in client command header logic

## Testing
- `just fmt`
- `just fix`
- `cargo test --all-features` *(fails: NotPresent in
core/tests/client.rs)*

------
https://chatgpt.com/codex/tasks/task_i_6893e63b0c408321a8e1ff2a052c4c51
- Change todo (#1925)

<img width="746" height="135" alt="image"
src="https://github.com/user-attachments/assets/1605b2fb-aa3a-4337-b9e9-93f6ff1361c5"
/>


<img width="747" height="126" alt="image"
src="https://github.com/user-attachments/assets/6b4366bd-8548-4d29-8cfa-cd484d9a2359"
/>
- Add a UI hint when you press @ (#1903)

This will make @ more discoverable (even though it is currently not
super useful, IMO it should be used to bring files into context from
outside CWD)

---------

Co-authored-by: Gabriel Peal <gpeal@users.noreply.github.com>
- Move used tokens next to the hints (#1930)

Before:

<img width="341" height="58" alt="image"
src="https://github.com/user-attachments/assets/3b209e42-1157-4f7b-8385-825c865969e8"
/>

After:

<img width="490" height="53" alt="image"
src="https://github.com/user-attachments/assets/5d99b9bc-6ac2-4748-b62c-c0c3217622c2"
/>
- Tint chat composer background (#1921)

## Summary
- give the chat composer a subtle custom background and apply it across
the full area drawn

<img width="1008" height="718" alt="composer-bg"
src="https://github.com/user-attachments/assets/4b0f7f69-722a-438a-b4e9-0165ae8865a6"
/>

- update turn interrupted to be more human readable
<img width="648" height="170" alt="CleanShot 2025-08-06 at 22 44 47@2x"
src="https://github.com/user-attachments/assets/8d35e53a-bbfa-48e7-8612-c280a54e01dd"
/>

## Testing
- `cargo test --all-features` *(fails: `let` expressions in
`core/src/client.rs` require newer rustc)*
- `just fix` *(fails: `let` expressions in `core/src/client.rs` require
newer rustc)*

------
https://chatgpt.com/codex/tasks/task_i_68941f32c1008322bbcc39ee1d29a526
- [fix] fix absolute and % token counts (#1931)

- For absolute, use non-cached input + output.
- For estimating what % of the model's context window is used, we need
to account for reasoning output tokens from prior turns being dropped
from the context window. We approximate this here by subtracting
reasoning output tokens from the total. This will be off for the current
turn and pending function calls. We can improve it later.
- Add logout command to CLI and TUI (#1932)

## Summary
- support `codex logout` via new subcommand and helper that removes the
stored `auth.json`
- expose a `logout` function in `codex-login` and test it
- add `/logout` slash command in the TUI; command list is filtered when
not logged in and the handler deletes `auth.json` then exits

## Testing
- `just fix` *(fails: failed to get `diffy` from crates.io)*
- `cargo test --all-features` *(fails: failed to get `diffy` from
crates.io)*

------
https://chatgpt.com/codex/tasks/task_i_68945c3facac832ca83d48499716fb51
- Fix outstanding review comments from the bot on #1919 (#1928)
- Add spinner animation to TUI status indicator (#1917)

## Summary
- add a pulsing dot loader before the shimmering `Working` label in the
status indicator widget and include a small test asserting the spinner
character is rendered
- also fix a small bug in the ran command header by adding a space
between the ⚡ and `Ran command`


https://github.com/user-attachments/assets/6768c9d2-e094-49cb-ad51-44bcac10aa6f

## Testing
- `just fmt`
- `just fix` *(failed: E0658 `let` expressions in core/src/client.rs)*
- `cargo test --all-features` *(failed: E0658 `let` expressions in
core/src/client.rs)*

------
https://chatgpt.com/codex/tasks/task_i_68941bffdb948322b0f4190bc9dbe7f6

---------

Co-authored-by: aibrahim-oai <aibrahim@openai.com>
- Approval ui (#1933)

Asking for approval:

<img width="269" height="41" alt="image"
src="https://github.com/user-attachments/assets/b9ced569-3297-4dae-9ce7-0b015c9e14ea"
/>

Allow:

<img width="400" height="31" alt="image"
src="https://github.com/user-attachments/assets/92056b22-efda-4d49-854d-e2943d5fcf17"
/>

Reject:

<img width="372" height="30" alt="image"
src="https://github.com/user-attachments/assets/be9530a9-7d41-4800-bb42-abb9a24fc3ea"
/>

Always Approve:

<img width="410" height="36" alt="image"
src="https://github.com/user-attachments/assets/acf871ba-4c26-4501-b303-7956d0151754"
/>
- Update copy (#1935)

Updated copy

---------

Co-authored-by: pap-openai <pap@openai.com>
- Calculate remaining context based on last token usage (#1940)

We should only take last request size (in tokens) into account
- Rename the model (#1942)
- [config] Onboarding flow with persistence (#1929)

## Summary
In collaboration with @gpeal: upgrade the onboarding flow, and persist
user settings.

---------

Co-authored-by: Gabriel Peal <gabriel@openai.com>
- Better usage errors (#1941)

<img width="771" height="279" alt="image"
src="https://github.com/user-attachments/assets/e56f967f-bcd7-49f7-8a94-3d88df68b65a"
/>
- Remove composer bg (#1944)

passes local tests
- Use different field for error type (#1945)
- Add capacity error (#1947)
- Update readme (#1948)

Co-authored-by: Alexander Embiricos <ae@openai.com>
- Ctrl+arrows also move words (#1949)

this was removed at some point, but this is a common keybind for word
left/right.
- [client] Tune retries and backoff (#1956)

## Summary
10 is a bit excessive 😅 Also updates our backoff factor to space out
requests further.
- Rename CodexAuth::new() to create_dummy_codex_auth_for_testing() because it is not for general consumption (#1962)
- Make CodexAuth::api_key a private field (#1965)
- Move top-level load_auth() to CodexAuth::from_codex_home() (#1966)
- Change CodexAuth::from_api_key() to take &str instead of String (#1970)
- Adjust error messages (#1969)

<img width="1378" height="285" alt="image"
src="https://github.com/user-attachments/assets/f0283378-f839-4a1f-8331-909694a04b1f"
/>
- Streaming markdown (#1920)

We wait until we have an entire newline, then format it with markdown and stream in to the UI. This reduces time to first token but is the right thing to do with our current rendering model IMO. Also lets us add word wrapping!
- Revert "Streaming markdown (#1920)" (#1981)

This reverts commit 2b7139859ec1edcdfe271b1f7615f308f8e60a53.
- Remove part of the error message (#1983)
- Fix usage limit banner grammar (#2018)

## Summary
- fix typo in usage limit banner text
- update error message tests

## Testing
- `just fmt`
- `RUSTC_BOOTSTRAP=1 just fix` *(fails: `let` expressions in this
position are unstable)*
- `RUSTC_BOOTSTRAP=1 cargo test --all-features` *(fails: `let`
expressions in this position are unstable)*

------
https://chatgpt.com/codex/tasks/task_i_689610fc1fe4832081bdd1118779b60b
- Fix multiline exec command rendering (#2023)

With Ratatui, if a single line contains newlines, it increments y but
not x so each subsequent line continued from the same x position as the
previous line ended on.

Before
<img width="2010" height="376" alt="CleanShot 2025-08-08 at 09 13 13"
src="https://github.com/user-attachments/assets/09feefbd-c5ee-4631-8967-93ab108c352a"
/>
After
<img width="1002" height="364" alt="CleanShot 2025-08-08 at 09 11 54"
src="https://github.com/user-attachments/assets/a58b47cf-777f-436a-93d9-ab277046a577"
/>
- Fix rust build on windows (#2019)

This pull request implements a fix from #2000, as well as fixed an
additional problem with path lengths on windows that prevents the login
from displaying.

---------

Co-authored-by: Michael Bolin <bolinfest@gmail.com>
Co-authored-by: Michael Bolin <mbolin@openai.com>
- Moving the compact prompt near where it's used (#2031)

- Moved the prompt for compact to core
- Renamed it to be more clear
- Use certifi certificate when available (#2042)

certifi has a more consistent set of Mozilla maintained root
certificates
- Update README.md (#1989)

Updates the README to clarify auth vs. api key behavior.
- Remove the TypeScript code from the repository (#2048)
- [exec] Fix exec sandbox arg (#2034)

## Summary
From codex-cli 😁 
`-s/--sandbox` now correctly affects sandbox mode.

What changed
- In `codex-rs/exec/src/cli.rs`:
- Added `value_enum` to the `--sandbox` flag so Clap parses enum values
into `
SandboxModeCliArg`.
- This ensures values like `-s read-only`, `-s workspace-write`, and `-s
dange
r-full-access` are recognized and propagated.

Why this fixes it
- The enum already derives `ValueEnum`, but without `#[arg(value_enum)]`
Clap ma
y not map the string into the enum, leaving the option ineffective at
runtime. W
ith `value_enum`, `sandbox_mode` is parsed and then converted to
`SandboxMode` i
n `run_main`, which feeds into `ConfigOverrides` and ultimately into the
effecti
ve `sandbox_policy`.
- [core] Allow resume after client errors (#2053)

## Summary
Allow tui conversations to resume after the client fails out of retries.
I tested this with exec / mocked api failures as well, and it appears to
be fine. But happy to add an exec integration test as well!

## Testing
- [x] Added integration test
- [x] Tested locally
- Show ChatGPT login URL during onboarding (#2028)

## Summary
- display authentication URL in the ChatGPT sign-in screen while
onboarding

<img width="684" height="151" alt="image"
src="https://github.com/user-attachments/assets/a8c32cb0-77f6-4a3f-ae3b-6695247c994d"
/>
- Update the docs to explain how to authenticate on a headless machine (#2121)
- Middle-truncate tool output and show more lines (#2096)

Command output can contain important bits of information at the
beginning or end. This shows a bit more output and truncates in the
middle.

This will work better paired with
https://github.com/openai/codex/pull/2095 which will omit output for
simple successful reads/searches/etc.

<img width="1262" height="496" alt="CleanShot 2025-08-09 at 13 01 05"
src="https://github.com/user-attachments/assets/9d989eb6-f81e-4118-9745-d20728eeef71"
/>


------
https://chatgpt.com/codex/tasks/task_i_68978cd19f9c832cac4975e44dcd99a0
- *(deps)* Bump tokio-util from 0.7.15 to 0.7.16 in /codex-rs (#2155)
- Trace RAW sse events (#2056)

For easier parsing.
- Show feedback message after /Compact command (#2162)

This PR updates ChatWidget to ensure that when AgentMessage,
AgentReasoning, or AgentReasoningRawContent events arrive without any
streamed deltas, the final text from the event is rendered before the
stream is finalized. Previously, these handlers ignored the event text
in such cases, relying solely on prior deltas.

<img width="603" height="189" alt="image"
src="https://github.com/user-attachments/assets/868516f2-7963-4603-9af4-adb1b1eda61e"
/>
- [1/3] Parse exec commands and format them more nicely in the UI (#2095)

# Note for reviewers
The bulk of this PR is in in the new file, `parse_command.rs`. This file
is designed to be written TDD and implemented with Codex. Do not worry
about reviewing the code, just review the unit tests (if you want). If
any cases are missing, we'll add more tests and have Codex fix them.

I think the best approach will be to land and iterate. I have some
follow-ups I want to do after this lands. The next PR after this will
let us merge (and dedupe) multiple sequential cells of the same such as
multiple read commands. The deduping will also be important because the
model often reads the same file multiple times in a row in chunks

===

This PR formats common commands like reading, formatting, testing, etc
more nicely:

It tries to extract things like file names, tests and falls back to the
cmd if it doesn't. It also only shows stdout/err if the command failed.

<img width="770" height="238" alt="CleanShot 2025-08-09 at 16 05 15"
src="https://github.com/user-attachments/assets/0ead179a-8910-486b-aa3d-7d26264d751e"
/>
<img width="348" height="158" alt="CleanShot 2025-08-09 at 16 05 32"
src="https://github.com/user-attachments/assets/4302681b-5e87-4ff3-85b4-0252c6c485a9"
/>
<img width="834" height="324" alt="CleanShot 2025-08-09 at 16 05 56 2"
src="https://github.com/user-attachments/assets/09fb3517-7bd6-40f6-a126-4172106b700f"
/>

Part 2: https://github.com/openai/codex/pull/2097
Part 3: https://github.com/openai/codex/pull/2110
- [mcp-server] Support CodexToolCallApprovalPolicy::OnRequest (#2187)

## Summary
#1865 added `AskForApproval::OnRequest`, but missed adding it to our
custom struct in `mcp-server`. This adds the missing configuration

## Testing
- [x] confirmed locally
- [2/3] Retain the TUI last exec history cell so that it can be updated by the next tool call (#2097)

Right now, every time an exec ends, we emit it to history which makes it
immutable. In order to be able to update or merge successive tool calls
(which will be useful after https://github.com/openai/codex/pull/2095),
we need to retain it as the active cell.

This also changes the cell to contain the metadata necessary to render
it so it can be updated rather than baking in the final text lines when
the cell is created.


Part 1: https://github.com/openai/codex/pull/2095
Part 3: https://github.com/openai/codex/pull/2110
- Include output truncation message in tool call results (#2183)

To avoid model being confused about incomplete output.
- Refactor approval Patch UI. Stack: [1/2] (#2049)
- [3/3] Merge sequential exec commands (#2110)

This PR merges and dedupes sequential exec cells so they stack neatly on
sequential lines rather than separate blocks.

This is particularly useful because the model will often sed 200 lines
of a file multiple times in a row and this nicely collapses them.


https://github.com/user-attachments/assets/04cccda5-e2ba-4a97-a613-4547587aa15c

Part 1: https://github.com/openai/codex/pull/2095
Part 2: https://github.com/openai/codex/pull/2097
- [apply-patch] Support applypatch command string (#2186)

## Summary
GPT-OSS and `gpt-5-mini` have training artifacts that cause the models
to occasionally use `applypatch` instead of `apply_patch`. I think
long-term we'll want to provide `apply_patch` as a first class tool, but
for now let's silently handle this case to avoid hurting model
performance

## Testing
- [x] Added unit test
- [TUI] Split multiline commands (#2202)

Fixes:
<img width="5084" height="1160" alt="CleanShot 2025-08-11 at 16 02 55"
src="https://github.com/user-attachments/assets/ccdbf39d-dc8b-4214-ab65-39ac89841d1c"
/>
- Send prompt_cache_key (#2200)

To optimize prompt caching performance.
- [prompts] integration test prompt caching (#2189)

## Summary
Our current approach to prompt caching is fragile! The current approach
works, but we are planning to update to a more resilient system (storing
them in the rollout file). Let's start adding some integration tests to
ensure stability while we migrate it.

## Testing
- [x] These are the tests 😎
- *(deps)* Bump toml from 0.9.4 to 0.9.5 in /codex-rs (#2157)
- Update codex-rs/config.md to reflect that gpt-5 is the default model (#2199)
- *(deps)* Bump clap from 4.5.41 to 4.5.43 in /codex-rs (#2159)
- *(deps-dev)* Bump @types/node from 24.1.0 to 24.2.1 in /.github/actions/codex (#2164)
- *(deps)* Bump clap_complete from 4.5.55 to 4.5.56 in /codex-rs (#2158)
- Show apply patch diff. Stack: [2/2] (#2050)
- *(deps-dev)* Bump @types/bun from 1.2.19 to 1.2.20 in /.github/actions/codex (#2163)
- Support truststore when available and add tracing (#2232)

Supports minimal tracing and detection of working ssl cert.
- Set user-agent (#2230)

Use the same well-defined value in all cases when sending user-agent
header
- [prompt] Restore important guidance for shell command usage (#2211)

## Summary
In #1939 we overhauled a lot of our prompt. This was largely good, but
we're seeing some specific points of confusion from the model! This
prompt update attempts to address 3 of them:
- Enforcing the use of `ripgrep`, which is bundled as a dependency when
installed with homebrew. We should do the same on node (in progress)
- Explicit guidance on reading files in chunks.
- Slight adjustment to networking sandbox language. `enabled` /
`restricted` is anecdotally less confusing to the model and requires
less reasoning to escalate for approval.

We are going to continue iterating on shell usage and tools, but this
restores us to best practices for current model snapshots.

## Testing
- [x] evals
- [x] local testing
- Show "Update plan" in TUI plan updates (#2192)

## Summary
- Display "Update plan" instead of "Update to do" when the plan is
updated in the TUI

## Testing
- `just fmt`
- `just fix` *(fails: E0658 `let` expressions in this position are
unstable)*
- `cargo test --all-features` *(fails: E0658 `let` expressions in this
position are unstable)*

------
https://chatgpt.com/codex/tasks/task_i_6897f78fc5908322be488f02db42a5b9
- Fix release build (#2244)

Missing import.
- Better implementation of interrupt on Esc (#2111)

Use existing abstractions
- Fix build break and build release (#2242)

Build release profile for one configuration.
- Re-add markdown streaming (#2029)

Wait for newlines, then render markdown on a line by line basis. Word wrap it for the current terminal size and then spit it out line by line into the UI. Also adds tests and fixes some UI regressions.
- Fix frontend test (#2247)

UI fixtures are brittle! Who knew.
- Update header from Working once batched commands are done (#2249)

Update commands from Working to Complete or Failed after they're done

before:
<img width="725" height="332" alt="image"
src="https://github.com/user-attachments/assets/fb93d21f-5c4a-42bc-a154-14f4fe99d5f9"
/>

after:
<img width="464" height="65" alt="image"
src="https://github.com/user-attachments/assets/15ec7c3b-355f-473e-9a8e-eab359ec5f0d"
/>
- Introduce ConversationManager as a clearinghouse for all conversations (#2240)
- [codex-cli] Add ripgrep as a dependency for node environment (#2237)

## Summary
Ripgrep is our preferred tool for file search. When users install via
`brew install codex`, it's automatically installed as a dependency. We
want to ensure that users running via an npm install also have this
tool! Microsoft has already solved this problem for VS Code - let's not
reinvent the wheel.

This approach of appending to the PATH directly might be a bit
heavy-handed, but feels reasonably robust to a variety of environment
concerns. Open to thoughts on better approaches here!

## Testing
- [x] confirmed this import approach works with `node -e "const { rgPath
} = require('@vscode/ripgrep'); require('child_process').spawn(rgPath,
['--version'], { stdio: 'inherit' })"`
- [x] Ran codex.js locally with `rg` uninstalled, asked it to run `which
rg`. Output below:

```
⚡ Ran command which rg; echo $?
  ⎿ /Users/dylan.hurd/code/dh--npm-rg/node_modules/@vscode/ripgrep/bin/rg
    0

codex
Re-running to confirm the path and exit code.

- Path: `/Users/dylan.hurd/code/dh--npm-rg/node_modules/@vscode/ripgrep/bin/rg`
- Exit code: `0`
```
- Change the diff preview to have color fg not bg (#2270)
- Wait for requested delay in rate limit errors (#2266)

Fixes: https://github.com/openai/codex/issues/2131

Response doesn't have the delay in a separate field (yet) so parse the
message.
- Use modifier dim instead of gray and .dim (#2273)

gray color doesn't work very well with white terminals. `.dim` doesn't
have an effect for some reason.

after:
<img width="1080" height="149" alt="image"
src="https://github.com/user-attachments/assets/26c0f8bb-550d-4d71-bd06-11b3189bc1d7"
/>

Before
<img width="1077" height="186" alt="image"
src="https://github.com/user-attachments/assets/b1fba0c7-bc4d-4da1-9754-6c0a105e8cd1"
/>
- Standardize tree prefix glyphs to └ (#2274)
- Enable reasoning for codex-prefixed models (#2275)

## Summary
- enable reasoning for any model slug starting with `codex-`
- provide default model info for `codex-*` slugs
- test that codex models are detected and support reasoning

## Testing
- `just fmt`
- `just fix` *(fails: E0658 `let` expressions in this position are
unstable)*
- `cargo test --all-features` *(fails: E0658 `let` expressions in this
position are unstable)*

------
https://chatgpt.com/codex/tasks/task_i_689d13f8705483208a6ed21c076868e1
- Parse reasoning text content (#2277)

Sometimes COT is returns as text content instead of `ReasoningText`. We
should parse it but not serialize back on requests.

---------

Co-authored-by: Ahmed Ibrahim <aibrahim@openai.com>
- Clarify PR/Contribution guidelines and issue templates (#2281)

Co-authored-by: Dylan <dylan.hurd@openai.com>
- Use enhancement tag for feature requests (#2282)
- Create Session as part of Codex::spawn() (#2291)
- Tag InputItem (#2304)

Instead of:
```
{ Text: { text: string } }
```

It is now:
```
{ type: "text", data: { text: string } }
```
which makes for cleaner discriminated unions
- HistoryCell is a trait (#2283)

refactors HistoryCell to be a trait instead of an enum. Also collapse
the many "degenerate" HistoryCell enums which were just a store of lines
into a single PlainHistoryCell type.

The goal here is to allow more ways of rendering history cells (e.g.
expanded/collapsed/"live"), and I expect we will return to more varied
types of HistoryCell as we develop this area.
- Remove "status text" in bottom line (#2279)

this used to hold the most recent log line, but it was kinda broken and
not that useful.
- [context] Store context messages in rollouts (#2243)

## Summary
Currently, we use request-time logic to determine the user_instructions
and environment_context messages. This means that neither of these
values can change over time as conversations go on. We want to add in
additional details here, so we're migrating these to save these messages
to the rollout file instead. This is simpler for the client, and allows
us to append additional environment_context messages to each turn if we
want

## Testing
- [x] Integration test coverage
- [x] Tested locally with a few turns, confirmed model could reference
environment context and cached token metrics were reasonably high
- Remove the · animation (#2271)

the pulsing dot felt too noisy to me next to the shimmering "Working"
text. we'll bring it back for streaming response text perhaps?
- Remove logs from composer by default (#2307)

Currently the composer shows `handle_codex_event:<event name>` by
default which feels confusing. Let's make it appear in trace.
- Text elements in textarea for pasted content (#2302)

This improves handling of pasted content in the textarea. It's no longer
possible to partially delete a placeholder (e.g. by ^W or ^D), nor is it
possible to place the cursor inside a placeholder. Also, we now render
placeholders in a different color to make them more clearly
differentiated.


https://github.com/user-attachments/assets/2051b3c3-963d-4781-a610-3afee522ae29
- Use a central animation loop (#2268)

instead of each shimmer needing to have its own animation thread, have
render_ref schedule a new frame if it wants one and coalesce to the
earliest next frame. this also makes the animations
frame-timing-independent, based on start time instead of frame count.
- *(core)* Add seatbelt sem lock tests (#1823)
- Add a timer to running exec commands (#2321)

sometimes i switch back to codex and i don't know how long a command has
been running.

<img width="744" height="462" alt="Screenshot 2025-08-14 at 3 30 07 PM"
src="https://github.com/user-attachments/assets/bd80947f-5a47-43e6-ad19-69c2995a2a29"
/>
- Clear running commands in various places (#2325)

we have a very unclear lifecycle for the chatwidget—this should only
have to be added in one place! but this fixes the "hanging commands"
issue where the active_exec_cell wasn't correctly cleared when commands
finished.

To repro w/o this PR:
1. prompt "run sleep 10"
2. once the command starts running, press <kbd>Esc</kbd>
3. prompt "run echo hi"

Expected: 

```
✓ Completed
  └ ⌨️ echo hi

codex
hi
```

Actual:

```
⚙︎ Working
  └ ⌨️ echo hi

▌ Ask Codex to do anything
```

i.e. the "Working" never changes to "Completed".

The bug is fixed with this PR.
- Port login server to rust (#2294)

Port the login server to rust.

---------

Co-authored-by: pakrym-oai <pakrym@openai.com>
- Fix AF_UNIX, sockpair, recvfrom in linux sandbox (#2309)

When using codex-tui on a linux system I was unable to run `cargo
clippy` inside of codex due to:
```
[pid 3548377] socketpair(AF_UNIX, SOCK_SEQPACKET|SOCK_CLOEXEC, 0,  <unfinished ...>
[pid 3548370] close(8 <unfinished ...>
[pid 3548377] <... socketpair resumed>0x7ffb97f4ed60) = -1 EPERM (Operation not permitted)
```
And
```
3611300 <... recvfrom resumed>0x708b8b5cffe0, 8, 0, NULL, NULL) = -1 EPERM (Operation not permitted)
```

This PR:
* Fixes a bug that disallowed AF_UNIX to allow it on `socket()`
* Adds recvfrom() to the syscall allow list, this should be fine since
we disable opening new sockets. But we should validate there is not a
open socket inheritance issue.
* Allow socketpair to be called for AF_UNIX
* Adds tests for AF_UNIX components
* All of which allows running `cargo clippy` within the sandbox on
linux, and possibly other tooling using a fork server model + AF_UNIX
comms.
- AGENTS.md more strongly suggests running targeted tests first (#2306)
- Added `allow-expect-in-tests` / `allow-unwrap-in-tests` (#2328)

This PR:
* Added the clippy.toml to configure allowable expect / unwrap usage in
tests
* Removed as many expect/allow lines as possible from tests
* moved a bunch of allows to expects where possible

Note: in integration tests, non `#[test]` helper functions are not
covered by this so we had to leave a few lingering `expect(expect_used`
checks around
- Re-implement session id in status (#2332)

Basically the same thing as https://github.com/openai/codex/pull/2297
- Cleanup rust login server a bit more (#2331)

Remove some extra abstractions.

---------

Co-authored-by: easong-openai <easong@openai.com>
- Format multiline commands (#2333)

<img width="966" height="729" alt="image"
src="https://github.com/user-attachments/assets/fa45b7e1-cd46-427f-b2bc-8501e9e4760b"
/>
<img width="797" height="530" alt="image"
src="https://github.com/user-attachments/assets/6993eec5-e157-4df7-b558-15643ad10d64"
/>
- Include optional full command line in history display (#2334)
- [tools] Add apply_patch tool (#2303)

## Summary
We've been seeing a number of issues and reports with our synthetic
`apply_patch` tool, e.g. #802. Let's make this a real tool - in my
anecdotal testing, it's critical for GPT-OSS models, but I'd like to
make it the standard across GPT-5 and codex models as well.

## Testing
- [x] Tested locally
- [x] Integration test
- Align diff display by always showing sign char and keeping fixed gutter (#2353)
- Skip identical consecutive entries in local composer history (#2352)
- Fix #2296 Add "minimal" reasoning effort for GPT 5 models (#2326)

This pull request resolves #2296; I've confirmed if it works by:

1. Add settings to ~/.codex/config.toml:
```toml
model_reasoning_effort = "minimal"
```

2. Run the CLI:
```
cd codex-rs
cargo build && RUST_LOG=trace cargo run --bin codex
/status
tail -f ~/.codex/log/codex-tui.log
```

Co-authored-by: pakrym-oai <pakrym@openai.com>
- Remove duplicated "Successfully logged in message" (#2357)
- Color the status letter in apply patch summary (#2337)

<img width="440" height="77" alt="Screenshot 2025-08-14 at 8 30 30 PM"
src="https://github.com/user-attachments/assets/c6169a3a-2e98-4ace-b7ee-918cf4368b7a"
/>
- Remove duplicated lockfile (#2336)
- Show progress indicator for /diff command (#2245)

## Summary
- Show a temporary Working on diff state in the bottom pan 
- Add `DiffResult` app event and dispatch git diff asynchronously

## Testing
- `just fmt`
- `just fix` *(fails: `let` expressions in this position are unstable)*
- `cargo test --all-features` *(fails: `let` expressions in this
position are unstable)*

------
https://chatgpt.com/codex/tasks/task_i_689a839f32b88321840a893551d5fbef
- Replace /prompts with a rotating placeholder (#2314)
- Added launch profile for attaching to a running codex CLI process (#2372)
- Added MCP server command to enable authentication using ChatGPT (#2373)

This PR adds two new APIs for the MCP server: 1) loginChatGpt, and 2)
cancelLoginChatGpt. The first starts a login server and returns a local
URL that allows for browser-based authentication, and the second
provides a way to cancel the login attempt. If the login attempt
succeeds, a notification (in the form of an event) is sent to a
subscriber.

I also added a timeout mechanism for the existing login server. The
loginChatGpt code path uses a 10-minute timeout by default, so if the
user fails to complete the login flow in that timeframe, the login
server automatically shuts down. I tested the timeout code by manually
setting the timeout to a much lower number and confirming that it works
as expected when used e2e.
- Remove mcp-server/src/mcp_protocol.rs and the code that depends on it (#2360)
- *(deps-dev)* Bump @types/node from 24.2.1 to 24.3.0 in /.github/actions/codex (#2411)
- Move mcp-server/src/wire_format.rs to protocol/src/mcp_protocol.rs (#2423)
- Add TS annotation to generated mcp-types (#2424)
- Consolidate reasoning enums into one (#2428)

We have three enums for each of reasoning summaries and reasoning effort
with same values. They can be consolidated into one.
- Add an operation to override current task context (#2431)

- Added an operation to override current task context
- Added a test to check that cache stays the same
- Protocol-ts (#2425)
- Add cache tests for UserTurn (#2432)
- Fix #2391 Add Ctrl+H as backspace keyboard shortcut (#2412)

This pull request resolves #2391. ctrl + h is not assigned to any other
operations at this moment, and this feature request sounds valid to me.
If we don't prefer having this, please feel free to close this.
- *(deps)* Bump anyhow from 1.0.98 to 1.0.99 in /codex-rs (#2405)
- *(deps)* Bump libc from 0.2.174 to 0.2.175 in /codex-rs (#2406)
- Prefer returning Err to expect() (#2389)
- *(deps)* Bump clap from 4.5.43 to 4.5.45 in /codex-rs (#2404)
- Release zip archived binaries (#2438)

Adds zip archives to release workflow to improve compatibility (mainly
older versions Windows which don't support `tar.gz` or `.zst` out of the
box).

Test release:
https://github.com/UnownPlain/codex/releases/tag/rust-v0.0.0
Test run: https://github.com/UnownPlain/codex/actions/runs/16981943609
- *(deps)* Bump clap_complete from 4.5.56 to 4.5.57 in /codex-rs (#2403)
- Show login options when not signed in with ChatGPT (#2440)

Motivation: we have users who uses their API key although they want to
use ChatGPT account. We want to give them the chance to always login
with their account.

This PR displays login options when the user is not signed in with
ChatGPT. Even if you have set an OpenAI API key as an environment
variable, you will still be prompted to log in with ChatGPT.

We’ve also added a new flag, `always_use_api_key_signing` false by
default, which ensures you are never asked to log in with ChatGPT and
always defaults to using your API key.



https://github.com/user-attachments/assets/b61ebfa9-3c5e-4ab7-bf94-395c23a0e0af

After ChatGPT sign in:


https://github.com/user-attachments/assets/d58b366b-c46a-428f-a22f-2ac230f991c0
- [tui] Support /mcp command (#2430)

## Summary
Adds a `/mcp` command to list active tools. We can extend this command
to allow configuration of MCP tools, but for now a simple list command
will help debug if your config.toml and your tools are working as
expected.
- Fix #2429 Tweak the cursor position after tab completion (#2442)

This pull request resolves #2429; I was also feeling that this is not
great dev experience, so we should fix.
- Support Ghostty Ctrl-b/Ctrl-f fallback (#2427)
- *(deps)* Bump actions/checkout from 4 to 5 (#2407)
- Support changing reasoning effort (#2435)

https://github.com/user-attachments/assets/50198ee8-5915-47a3-bb71-69af65add1ef

Building up on #2431 #2428
- Document writable_roots for sandbox_workspace_write (#2464)
- Upgrade to Rust 1.89 (#2465)
- Rust 1.89 promoted file locking to the standard library, so prefer stdlib to fs2 (#2467)
- Sign in appear even if using other providers. (#2475)
- Enable Dependabot updates for Rust toolchain (#2460)

This change allows Dependabot to update the Rust toolchain version
defined in `rust-toolchain.toml`. See [Dependabot now supports Rust
toolchain updates - GitHub
Changelog](https://github.blog/changelog/2025-08-19-dependabot-now-supports-rust-toolchain-updates/)
for more details.
- Diff command (#2476)
- Client headers (#2487)
- Refresh ChatGPT auth token (#2484)

ChatGPT token's live for only 1 hour. If the session is longer we don't
refresh the token. We should get the expiry timestamp and attempt to
refresh before it.
- Add a slash command to control permissions (#2474)

A slash command to control permissions



https://github.com/user-attachments/assets/c0edafcd-2085-4e09-8009-ba69c4f1c153

---------

Co-authored-by: ae <ae@openai.com>
- Detect terminal and include in request headers (#2437)

This adds the terminal version to the UA header.
- Tab-completing a command moves the cursor to the end (#2362)
- Switch to using tokio + EventStream for processing crossterm events (#2489)
- [apply-patch] Fix applypatch for heredocs (#2477)

## Summary
Follow up to #2186 for #2072 - we added handling for `applypatch` in
default commands, but forgot to add detection to the heredocs logic.

## Testing
- [x] Added unit tests
- Refactor onboarding screen to a separate "app" (#2524)

this is in preparation for adding more separate "modes" to the tui, in
particular, a "transcript mode" to view a full history once #2316 lands.

1. split apart "tui events" from "app events".
2. remove onboarding-related events from AppEvent.
3. move several general drawing tools out of App and into a new Tui
class
- Fix login for internal employees (#2528)

This PR:
- fixes for internal employee because we currently want to prefer SIWC
for them.
- fixes retrying forever on unauthorized access. we need to break
eventually on max retries.
- Link docs when no MCP servers configured (#2516)
- Bridge command generation to powershell when on Windows (#2319)

## What? Why? How?
- When running on Windows, codex often tries to invoke bash commands,
which commonly fail (unless WSL is installed)
- Fix: Detect if powershell is available and, if so, route commands to
it
- Also add a shell_name property to environmental context for codex to
default to powershell commands when running in that environment

## Testing
- Tested within WSL and powershell (e.g. get top 5 largest files within
a folder and validated that commands generated were powershell commands)
- Tested within Zsh
- Updated unit tests

---------

Co-authored-by: Eddy Escardo <eddy@openai.com>
- Add transcript mode (#2525)

this adds a new 'transcript mode' that shows the full event history in a
"pager"-style interface.


https://github.com/user-attachments/assets/52df7a14-adb2-4ea7-a0f9-7f5eb8235182
- Hide CoT by default; show headers in status indicator (#2316)

Plan is for full CoT summaries to be visible in a "transcript view" when
we implement that, but for now they're hidden.


https://github.com/user-attachments/assets/e8a1b0ef-8f2a-48ff-9625-9c3c67d92cdb
- Show thinking in transcript (#2538)

record the full reasoning trace and show it in transcript mode
- Show upgrade banner in history (#2537)
- Added new auth-related methods and events to mcp server (#2496)

This PR adds the following:
* A getAuthStatus method on the mcp server. This returns the auth method
currently in use (chatgpt or apikey) or none if the user is not
authenticated. It also returns the "preferred auth method" which
reflects the `preferred_auth_method` value in the config.
* A logout method on the mcp server. If called, it logs out the user and
deletes the `auth.json` file — the same behavior in the cli's `/logout`
command.
* An `authStatusChange` event notification that is sent when the auth
status changes due to successful login or logout operations.
* Logic to pass command-line config overrides to the mcp server at
startup time. This allows use cases like `codex mcp -c
preferred_auth_method=apikey`.
- Add a serde tag to ParsedItem (#2546)
- [prompt] xml-format EnvironmentContext (#2272)

## Summary
Before we land #2243, let's start printing environment_context in our
preferred format. This struct will evolve over time with new
information, xml gives us a balance of human readable without too much
parsing, llm readable, and extensible.

Also moves us over to an Option-based struct, so we can easily provide
diffs to the model.

## Testing
- [x] Updated tests to reflect new format
- Parse and expose stream errors (#2540)
- Scroll instead of clear on boot (#2535)

this actually works fine already in iterm without this change, but
Terminal.app adds a bunch of excess whitespace when we clear all.


https://github.com/user-attachments/assets/c5bd1809-c2ed-4daa-a148-944d2df52876
- Read all AGENTS.md up to git root (#2532)

This updates our logic for AGENTS.md to match documented behavior, which
is to read all AGENTS.md files from cwd up to git root.
- Show diff hunk headers to separate sections (#2488)
- Move slash command handling into chatwidget (#2536)
- Transcript mode updates live (#2562)
- Update README.md (#2564)

Adding some notes about MCP tool calls are not running within the
sandbox
- Tweak thresholds for shimmer on non-true-color terminals (#2533)

https://github.com/user-attachments/assets/dc7bf820-eeec-4b78-aba9-231e1337921c
- Write explicit [projects] tables for trusted projects (#2523)
- [shell_tool] Small updates to ensure shell consistency (#2571)

## Summary
Small update to hopefully improve some shell edge cases, and make the
function clearer to the model what is going on. Keeping `timeout` as an
alias means that calls with the previous name will still work.

## Test Plan
- [x] Tested locally, model still works
- [apply-patch] Clean up apply-patch tool definitions (#2539)

## Summary
We've experienced a bit of drift in system prompting for `apply_patch`:
- As pointed out in #2030 , our prettier formatting started altering
prompt.md in a few ways
- We introduced a separate markdown file for apply_patch instructions in
#993, but currently duplicate them in the prompt.md file
- We added a first-class apply_patch tool in #2303, which has yet
another definition

This PR starts to consolidate our logic in a few ways:
- We now only use
`apply_patch_tool_instructions.md](https://github.com/openai/codex/compare/dh--apply-patch-tool-definition?expand=1#diff-d4fffee5f85cb1975d3f66143a379e6c329de40c83ed5bf03ffd3829df985bea)
for system instructions
- We no longer include apply_patch system instructions if the tool is
specified

I'm leaving the definition in openai_tools.rs as duplicated text for now
because we're going to be iterated on the first-class tool soon.

## Testing
- [x] Added integration tests to verify prompt stability
- [x] Tested locally with several different models (gpt-5, gpt-oss,
o4-mini)
- Show diff output in the pager (#2568)

this shows `/diff` output in an overlay like the transcript, instead of
dumping it into history.



https://github.com/user-attachments/assets/48e79b65-7f66-45dd-97b3-d5c627ac7349
- Improve suspend behavior (#2569)

This is a somewhat roundabout way to fix the issue that pressing ^Z
would put the shell prompt in the wrong place (overwriting some of the
status area below the composer). While I'm at it, clean up the suspend
logic and fix some suspend-while-in-alt-screen behavior too.
- Ctrl+v image + @file accepts images (#1695)

allow ctrl+v in TUI for images + @file that are images are appended as
raw files (and read by the model) rather than pasted as a path that
cannot be read by the model.

Re-used components and same interface we're using for copying pasted
content in
https://github.com/openai/codex/commit/72504f1d9c6eb17086d86ef1fb0d17676812461b.
@aibrahim-oai as you've implemented this, mind having a look at this
one?


https://github.com/user-attachments/assets/c6c1153b-6b32-4558-b9a2-f8c57d2be710

---------

Co-authored-by: easong-openai <easong@openai.com>
Co-authored-by: Daniel Edrisian <dedrisian@openai.com>
Co-authored-by: Michael Bolin <mbolin@openai.com>
- Fix/tui windows multiline paste (#2544)

Introduce a minimal paste-burst heuristic in the chat composer so Enter
is treated as a newline during paste-like bursts (plain chars arriving
in very short intervals), avoiding premature submit after the first line
on Windows consoles that lack bracketed paste.

- Detect tight sequences of plain Char events; open a short window where
Enter inserts a newline instead of submitting.
- Extend the window on newline to handle blank lines in pasted content.
- No behavior change for terminals that already emit Event::Paste; no
OS/env toggles added.
- Add AuthManager and enhance GetAuthStatus command (#2577)

This PR adds a central `AuthManager` struct that manages the auth
information used across conversations and the MCP server. Prior to this,
each conversation and the MCP server got their own private snapshots of
the auth information, and changes to one (such as a logout or token
refresh) were not seen by others.

This is especially problematic when multiple instances of the CLI are
run. For example, consider the case where you start CLI 1 and log in to
ChatGPT account X and then start CLI 2 and log out and then log in to
ChatGPT account Y. The conversation in CLI 1 is still using account X,
but if you create a new conversation, it will suddenly (and
unexpectedly) switch to account Y.

With the `AuthManager`, auth information is read from disk at the time
the `ConversationManager` is constructed, and it is cached in memory.
All new conversations use this same auth information, as do any token
refreshes.

The `AuthManager` is also used by the MCP server's GetAuthStatus
command, which now returns the auth method currently used by the MCP
server.

This PR also includes an enhancement to the GetAuthStatus command. It
now accepts two new (optional) input parameters: `include_token` and
`refresh_token`. Callers can use this to request the in-use auth token
and can optionally request to refresh the token.

The PR also adds tests for the login and auth APIs that I recently added
to the MCP server.
- [apply_patch] freeform apply_patch tool (#2576)

## Summary
GPT-5 introduced the concept of [custom
tools](https://platform.openai.com/docs/guides/function-calling#custom-tools),
which allow the model to send a raw string result back, simplifying
json-escape issues. We are migrating gpt-5 to use this by default.

However, gpt-oss models do not support custom tools, only normal
functions. So we keep both tool definitions, and provide whichever one
the model family supports.

## Testing
- [x] Tested locally with various models
- [x] Unit tests pass
- [config] Detect git worktrees for project trust (#2585)

## Summary
When resolving our current directory as a project, we want to be a
little bit more clever:
1. If we're in a sub-directory of a git repo, resolve our project
against the root of the git repo
2. If we're in a git worktree, resolve the project against the root of
the git repo

## Testing
- [x] Added unit tests
- [x] Confirmed locally with a git worktree (the one i was using for
this feature)
- Improve performance of 'cargo test -p codex-tui' (#2593)

before:

```
$ time cargo test -p codex-tui -q
[...]
cargo test -p codex-tui -q  39.89s user 10.77s system 98% cpu 51.328 total
```

after:

```
$ time cargo test -p codex-tui -q
[...]
cargo test -p codex-tui -q  1.37s user 0.64s system 29% cpu 6.699 total
```

the major offenders were the textarea fuzz test and the custom_terminal
doctests. (i think the doctests were being recompiled every time which
made them extra slow?)
- Simplify tests in config.rs (#2586)
- Move models.rs to protocol (#2595)

Moving models.rs to protocol so we can use them in `Codex` operations
- *(deps)* Bump serde_json from 1.0.142 to 1.0.143 in /codex-rs (#2498)
- Fix flakiness in shell command approval test (#2547)

## Summary
- read the shell exec approval request's actual id instead of assuming
it is always 0
- use that id when validating and responding in the test

## Testing
- `cargo test -p codex-mcp-server
test_shell_command_approval_triggers_elicitation`

------
https://chatgpt.com/codex/tasks/task_i_68a6ab9c732c832c81522cbf11812be0
- *(deps)* Bump reqwest from 0.12.22 to 0.12.23 in /codex-rs (#2492)
- Fix typo in AGENTS.md (#2518)

- Change `examole` to `example`
- Open transcript mode at the bottom (#2592)
- Coalesce command output; show unabridged commands in transcript (#2590)
- Fix resize on wezterm (#2600)
- Fork conversation from a previous message (#2575)

This can be the underlying logic in order to start a conversation from a
previous message. will need some love in the UI.

Base for building this: #2588
- Add the ability to interrupt and provide feedback to the model (#2381)
- Transcript hint (#2605)

Adds a hint to use ctrl-t to view transcript for more details

<img width="475" height="49" alt="image"
src="https://github.com/user-attachments/assets/6ff650eb-ed54-4699-be04-3c50f0f8f631"
/>
- Send-aggregated output (#2364)

We want to send an aggregated output of stderr and stdout so we don't
have to aggregate it stderr+stdout as we lose order sometimes.

---------

Co-authored-by: Gabriel Peal <gpeal@users.noreply.github.com>
- Add web search tool (#2371)

Adds web_search tool, enabling the model to use Responses API web_search
tool.
- Disabled by default, enabled by --search flag
- When --search is passed, exposes web_search_request function tool to
the model, which triggers user approval. When approved, the model can
use the web_search tool for the remainder of the turn
<img width="1033" height="294" alt="image"
src="https://github.com/user-attachments/assets/62ac6563-b946-465c-ba5d-9325af28b28f"
/>

---------

Co-authored-by: easong-openai <easong@openai.com>
- Resume conversation from an earlier point in history (#2607)

Fixing merge conflict of this: #2588


https://github.com/user-attachments/assets/392c7c37-cf8f-4ed6-952e-8215e8c57bc4
- Faster test execution in codex-core (#2633)
- [apply_patch] disable default freeform tool (#2643)

## Summary
We're seeing some issues in the freeform tool - let's disable by default
until it stabilizes.

## Testing
- [x] Ran locally, confirmed codex-cli could make edits
- *(deps)* Bump whoami from 1.6.0 to 1.6.1 in /codex-rs (#2497)
- Fix cache hit rate by making MCP tools order deterministic (#2611)

Fixes https://github.com/openai/codex/issues/2610

This PR sorts the tools in `get_openai_tools` by name to ensure a
consistent MCP tool order.

Currently, MCP servers are stored in a HashMap, which does not guarantee
ordering. As a result, the tool order changes across turns, effectively
breaking prompt caching in multi-turn sessions.

An alternative solution would be to replace the HashMap with an ordered
structure, but that would require a much larger code change. Given that
it is unrealistic to have so many MCP tools that sorting would cause
performance issues, this lightweight fix is chosen instead.

By ensuring deterministic tool order, this change should significantly
improve cache hit rates and prevent users from hitting usage limits too
quickly. (For reference, my own sessions last week reached the limit
unusually fast, with cache hit rates falling below 1%.)

## Result

After this fix, sessions with MCP servers now show caching behavior
almost identical to sessions without MCP servers.
Without MCP             |  With MCP
:-------------------------:|:-------------------------:
<img width="1368" height="1634" alt="image"
src="https://github.com/user-attachments/assets/26edab45-7be8-4d6a-b471-558016615fc8"
/> | <img width="1356" height="1632" alt="image"
src="https://github.com/user-attachments/assets/5f3634e0-3888-420b-9aaf-deefd9397b40"
/>
- *(deps)* Bump toml_edit from 0.23.3 to 0.23.4 in /codex-rs (#2665)
- Index file (#2678)
- Avoid error when /compact response has no token_usage (#2417) (#2640)

**Context**  
When running `/compact`, `drain_to_completed` would throw an error if
`token_usage` was `None` in `ResponseEvent::Completed`. This made the
command fail even though everything else had succeeded.

**What changed**  
- Instead of erroring, we now just check `if let Some(token_usage)`
before sending the event.
- If it’s missing, we skip it and move on.  

**Why**  
This makes `AgentTask::compact()` behave in the same way as
`AgentTask::spawn()`, which also doesn’t error out when `token_usage`
isn’t available. Keeps things consistent and avoids unnecessary
failures.

**Fixes**  
Closes #2417

---------

Co-authored-by: Ahmed Ibrahim <aibrahim@openai.com>
- Queue messages (#2637)
- [exec] Clean up apply-patch tests (#2648)

## Summary
These tests were getting a bit unwieldy, and they're starting to become
load-bearing. Let's clean them up, and get them working solidly so we
can easily expand this harness with new tests.

## Test Plan
- [x] Tests continue to pass
- Fix esc (#2661)

Esc should have other functionalities when it's not used in a
backtracking situation. i.e. to cancel pop up menu when selecting
model/approvals or to interrupt an active turn.
- Add auth to send_user_turn (#2688)

It is there for send_user_message but was omitted from send_user_turn.
Presumably this was a mistake
- Copying / Dragging image files (MacOS Terminal + iTerm) (#2567)

In this PR:

- [x] Add support for dragging / copying image files into chat.
- [x] Don't remove image placeholders when submitting.
- [x] Add tests.

Works for:

- Image Files
- Dragging MacOS Screenshots (Terminal, iTerm)

Todos:

- [ ] In some terminals (VSCode, WIndows Powershell, and remote
SSH-ing), copy-pasting a file streams the escaped filepath as individual
key events rather than a single Paste event. We'll need to have a
function (in a separate PR) for detecting these paste events.
- Do not schedule frames for Tui::Draw events in backtrack (#2692)

this was causing continuous rerendering when a transcript overlay was
present
- Queued messages rendered italic (#2693)

<img width="416" height="215" alt="Screenshot 2025-08-25 at 5 29 53 PM"
src="https://github.com/user-attachments/assets/0f4178c9-6997-4e7a-bb30-0817b98d9748"
/>
- Do not show timeouts as "sandbox error"s (#2587)

🙅🫸
```
✗ Failed (exit -1)
  └ 🧪 cargo test --all-features -q
    sandbox error: command timed out
```

😌👉
```
✗ Failed (exit -1)
  └ 🧪 cargo test --all-features -q
    error: command timed out
```
- Fixed a bug that causes token refresh to not work in a seamless manner (#2699)

This PR fixes a bug in the token refresh logic. Token refresh is
performed in a retry loop so if we receive a 401 error, we refresh the
token, then we go around the loop again and reissue the fetch with a
fresh token. The bug is that we're not using the updated token on the
second and subsequent times through the loop. The result is that we'll
try to refresh the token a few more times until we hit the retry limit
(default of 4). The 401 error is then passed back up to the caller.
Subsequent calls will use the refreshed token, so the problem clears
itself up.

The fix is straightforward — make sure we use the updated auth
information each time through the retry loop.
- Single control flow for both Esc and Ctrl+C (#2691)

Esc and Ctrl+C while a task is running should do the same thing. There
were some cases where pressing Esc would leave a "stuck" widget in the
history; this fixes that and cleans up the logic so there's just one
path for interrupting the task. Also clean up some subtly mishandled key
events (e.g. Ctrl+D would quit the app while an approval modal was
showing if the textarea was empty).

---------

Co-authored-by: Ahmed Ibrahim <aibrahim@openai.com>
- Improved user message for rate-limit errors (#2695)

This PR improves the error message presented to the user when logged in
with ChatGPT and a rate-limit error occurs. In particular, it provides
the user with information about when the rate limit will be reset. It
removes older code that attempted to do the same but relied on parsing
of error messages that are not generated by the ChatGPT endpoint. The
new code uses newly-added error fields.
- [feat] reduce bottom padding to 1 line (#2704)
- [fix] emoji padding (#2702)

- We use emojis as bullet icons of sorts, and in some common terminals
like Terminal or iTerm, these can render with insufficient padding
between the emoji and following text.
- This PR makes emoji look better in Terminal and iTerm, at the expense
of Ghostty. (All default fonts.)

# Terminal

<img width="420" height="123" alt="image"
src="https://github.com/user-attachments/assets/93590703-e35a-4781-a697-881d7ec95598"
/>

# iTerm

<img width="465" height="163" alt="image"
src="https://github.com/user-attachments/assets/f11e6558-d2db-4727-bb7e-2b61eed0a3b1"
/>

# Ghostty

<img width="485" height="142" alt="image"
src="https://github.com/user-attachments/assets/7a7b021f-5238-4672-8066-16cd1da32dc6"
/>
- Added caps on retry config settings (#2701)

The CLI supports config settings `stream_max_retries` and
`request_max_retries` that allow users to override the default retry
counts (4 and 5, respectively). However, there's currently no cap placed
on these values. In theory, a user could configure an effectively
infinite retry count which could hammer the server. This PR adds a
reasonable cap (currently 100) to both of these values.
- [chore] Tweak AGENTS.md so agent doesn't always have to test (#2706)
- [feat] Simplfy command approval UI (#2708)

- Removed the plain "No" option, which confused the model,
  since we already have the "No, provide feedback" option,
  which works better.

# Before

<img width="476" height="168" alt="image"
src="https://github.com/user-attachments/assets/6e783d9f-dec9-4610-9cad-8442eb377a90"
/>

# After

<img width="553" height="175" alt="image"
src="https://github.com/user-attachments/assets/3cdae582-3366-47bc-9753-288930df2324"
/>
- Enable alternate scroll in transcript mode (#2686)

this allows the mouse wheel to scroll the transcript / diff views.
- Render keyboard icon with emoji variation selector (⌨️) (#2728)
- Esc while there are queued messages drops the messages back into the composer (#2687)

https://github.com/user-attachments/assets/bbb427c4-cdc7-4997-a4ef-8156e8170742
- Fix crash when backspacing placeholders adjacent to multibyte text (#2674)

Prevented panics when deleting placeholders near multibyte characters by
clamping the cursor to a valid boundary and using get-based slicing

Added a regression test to ensure backspacing after multibyte text
leaves placeholders intact without crashing

---------

Co-authored-by: Ahmed Ibrahim <aibrahim@openai.com>
- Don't send Exec deltas on apply patch (#2742)

We are now sending exec deltas on apply patch which doesn't make sense.
- Cache transcript wraps (#2739)

Previously long transcripts would become unusable.
- Make git_diff_against_sha more robust (#2749)

1. Ignore custom git diff drivers users may have set
2. Allow diffing against filenames that start with a dash
- Send context window with task started (#2752)

- Send context window with task started
- Accounting for changing the model per turn
- Bug fix: deduplicate assistant messages (#2758)

We are treating assistant messages in a different way than other
messages which resulted in a duplicated history.

See #2698
- [mcp-server] Add GetConfig endpoint (#2725)

## Summary
Adds a GetConfig request to the MCP Protocol, so MCP clients can
evaluate the resolved config.toml settings which the harness is using.

## Testing
- [x] Added an end to end test of the endpoint
- README / docs refactor (#2724)

This PR cleans up the monolithic README by breaking it into a set
navigable pages under docs/ (install, getting started, configuration,
authentication, sandboxing and approvals, platform details, FAQ, ZDR,
contributing, license). The top‑level README is now more concise and
intuitive, (with corrected screenshots).

It also consolidates overlapping content from codex-rs/README.md into
the top‑level docs and updates links accordingly. The codex-rs README
remains in place for now as a pointer and for continuity.

Finally, added an extensive config reference table at the bottom of
docs/config.md.

---------

Co-authored-by: easong-openai <easong@openai.com>
- Added back codex-rs/config.md to link to new location (#2778)

Quick fix: point old config.md to new location
- Point the CHANGELOG to the releases page (#2780)

The typescript changelog is misleading and unhelpful
- Add "View Image" tool (#2723)

Adds a "View Image" tool so Codex can find and see images by itself:

<img width="1772" height="420" alt="Screenshot 2025-08-26 at 10 40
04 AM"
src="https://github.com/user-attachments/assets/7a459c7b-0b86-4125-82d9-05fbb35ade03"
/>
- Disallow some slash commands while a task is running (#2792)

/new, /init, /models, /approvals, etc. don't work correctly during a
turn. disable them.
- Require uninlined_format_args from clippy (#2845)
- Try to make it easier to debug the flakiness of test_shell_command_approval_triggers_elicitation (#2848)
- Print stderr from MCP server to test output using eprintln! (#2849)
- Race condition in compact (#2746)

This fixes the flakiness in
`summarize_context_three_requests_and_instructions` because we should
trim history before sending task complete.
- Burst paste edge cases (#2683)

This PR fixes two edge cases in managing burst paste (mainly on power
shell).
Bugs:
- Needs an event key after paste to render the pasted items

> ChatComposer::flush_paste_burst_if_due() flushes on timeout. Called:
>     - Pre-render in App on TuiEvent::Draw.
>     - Via a delayed frame
>
BottomPane::request_redraw_in(ChatComposer::recommended_paste_flush_delay()).

- Parses two key events separately before starting parsing burst paste

> When threshold is crossed, pull preceding burst chars out of the
textarea and prepend to paste_burst_buffer, then keep buffering.

- Integrates with #2567 to bring image pasting to windows.
- Add a VS Code Extension issue template (#2853)

Template mostly copied from the bug template
- Changed OAuth success screen to use the string "Codex" rather than "Codex CLI" (#2737)
- Make slash commands bold in welcome message (#2762)
- Custom /prompts (#2696)

Adds custom `/prompts` to `~/.codex/prompts/<command>.md`.

<img width="239" height="107" alt="Screenshot 2025-08-25 at 6 22 42 PM"
src="https://github.com/user-attachments/assets/fe6ebbaa-1bf6-49d3-95f9-fdc53b752679"
/>

---

Details:

1. Adds `Op::ListCustomPrompts` to core.
2. Returns `ListCustomPromptsResponse` with list of `CustomPrompt`
(name, content).
3. TUI calls the operation on load, and populates the custom prompts
(excluding prompts that collide with builtins).
4. Selecting the custom prompt automatically sends the prompt to the
agent.
- Following up on #2371 post commit feedback (#2852)

- Introduce websearch end to complement the begin 
- Moves the logic of adding the sebsearch tool to
create_tools_json_for_responses_api
- Making it the client responsibility to toggle the tool on or off 
- Other misc in #2371 post commit feedback
- Show the query:

<img width="1392" height="151" alt="image"
src="https://github.com/user-attachments/assets/8457f1a6-f851-44cf-bcca-0d4fe460ce89"
/>
- Bug fix: clone of  incoming_tx  can lead to deadlock (#2747)

POC code

```rust
use tokio::sync::mpsc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== Test 1: Simulating original MCP server pattern ===");
    test_original_pattern().await;
}

async fn test_original_pattern() {
    println!("Testing the original pattern from MCP server...");
    
    // Create channel - this simulates the original incoming_tx/incoming_rx
    let (tx, mut rx) = mpsc::channel::<String>(10);
    
    // Task 1: Simulates stdin reader that will naturally terminate
    let stdin_task = tokio::spawn({
        let tx_clone = tx.clone();
        async move {
            println!("  stdin_task: Started, will send 3 messages then exit");
            for i in 0..3 {
                let msg = format!("Message {}", i);
                if tx_clone.send(msg.clone()).await.is_err() {
                    println!("  stdin_task: Receiver dropped, exiting");
                    break;
                }
                println!("  stdin_task: Sent {}", msg);
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
            println!("  stdin_task: Finished (simulating EOF)");
            // tx_clone is dropped here
        }
    });
    
    // Task 2: Simulates message processor
    let processor_task = tokio::spawn(async move {
        println!("  processor_task: Started, waiting for messages");
        while let Some(msg) = rx.recv().await {
            println!("  processor_task: Processing {}", msg);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        println!("  processor_task: Finished (channel closed)");
    });
    
    // Task 3: Simulates stdout writer or other background task
    let background_task = tokio::spawn(async move {
        for i in 0..2 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("  background_task: Tick {}", i);
        }
        println!("  background_task: Finished");
    });
    
    println!("  main: Original tx is still alive here");
    println!("  main: About to call tokio::join! - will this deadlock?");
    
    // This is the pattern from the original code
    let _ = tokio::join!(stdin_task, processor_task, background_task);
}

```

---------

Co-authored-by: Michael Bolin <bolinfest@gmail.com>
- Fix CI release build (#2864)
- Prevents `brew install codex` in comment to be executed (#2868)
- Suggest just fix -p in agents.md (#2881)
- *(deps)* Bump regex-lite from 0.1.6 to 0.1.7 in /codex-rs (#3010)
- Remove extra quote from disabled-command message (#3035)

there was an extra ' floating around for some reason.
- Bug fix: ignore Enter on empty input to avoid queuing blank messages (#3047)

## Summary
Pressing Enter with an empty composer was treated as a submission, which
queued a blank message while a task was running. This PR suppresses
submission when there is no text and no attachments.

## Root Cause

- ChatComposer returned Submitted even when the trimmed text was empty.
ChatWidget then queued it during a running task, leading to an empty
item appearing in the queued list and being popped later with no effect.

## Changes
- ChatComposer Enter handling: if trimmed text is empty and there are no
attached images, return None instead of Submitted.
- No changes to ChatWidget; behavior naturally stops queuing blanks at
the source.

## Code Paths

- Modified: `tui/src/bottom_pane/chat_composer.rs`
- Tests added:
    - `tui/src/bottom_pane/chat_composer.rs`: `empty_enter_returns_none`
- `tui/src/chatwidget/tests.rs`:
`empty_enter_during_task_does_not_queue`

## Result

### Before


https://github.com/user-attachments/assets/a40e2f6d-42ba-4a82-928b-8f5458f5884d

### After



https://github.com/user-attachments/assets/958900b7-a566-44fc-b16c-b80380739c92
- Adapt pr template with correct link following doc refacto (#2982)
- Rework message styling (#2877)

https://github.com/user-attachments/assets/cf07f62b-1895-44bb-b9c3-7a12032eb371
- Fix laggy typing (#2922)
- Add logs to know when we users are changing the model (#3060)
- Hide '/init' suggestion when AGENTS.md exists (#3038)
- Fix extra blank lines in streamed agent messages (#3065)
- Catch get_cursor_position errors (#2870)
- Unify history loading (#2736)
- Fix occasional UI flicker (#2918)
- Prefer ratatui Stylized for constructing lines/spans (#3068)

no functional change, just simplifying ratatui styling and adding
guidance in AGENTS.md for future.
- Show loading state when @ search results are pending (#3061)

## Summary
- allow selection popups to specify their empty state message
- show a "loading..." placeholder in the file search popup while matches
are pending
- update other popup call sites to continue using a "no matches" message

## Testing
- just fmt
- just fix -p codex-tui
- cargo test -p codex-tui

------
https://chatgpt.com/codex/tasks/task_i_68b73e956e90832caf4d04a75fcc9c46
- [apply-patch] Fix lark grammar (#2651)

## Summary
Fixes an issue with the lark grammar definition for the apply_patch
freeform tool. This does NOT change the defaults, merely patches the
root cause of the issue we were seeing with empty lines, and an issue
with config flowing through correctly.

Specifically, the following requires that a line is non-empty:
```
add_line: "+" /(.+)/ LF -> line
```
but many changes _should_ involve creating/updating empty lines. The new
definition is:
```
add_line: "+" /(.*)/ LF -> line
```

## Testing
- [x] Tested locally, reproduced the issue without the update and
confirmed that the model will produce empty lines wiht the new lark
grammar
- Added back the logic to handle rate-limit errors when using API key (#3070)

A previous PR removed this when adding rate-limit errors for the ChatGPT
auth path.
- Move CodexAuth and AuthManager to the core crate (#3074)

Fix a long standing layering issue.
- [feat] use experimental reasoning summary (#3071)

<img width="1512" height="442" alt="Screenshot 2025-09-02 at 3 49 46 PM"
src="https://github.com/user-attachments/assets/26c3c1cf-b7ed-4520-a12a-8d38a8e0c318"
/>
- Improve gpt-oss compatibility (#2461)

The gpt-oss models require reasoning with subsequent Chat Completions
requests because otherwise the model forgets why the tools were called.
This change fixes that and also adds some additional missing
documentation around how to handle context windows in Ollama and how to
show the CoT if you desire to.
- Parse cd foo && ... for exec and apply_patch (#3083)

sometimes the model likes to run "cd foo && ..." instead of using the
workdir parameter of exec. handle them roughly the same.
- Update link to point to https://agents.md/ (#3089)
- Fix typo of config.md (#3082)
- Fix MCP docs hyperlink in empty_mcp_output (#2907)
- *(deps)* Bump thiserror from 2.0.12 to 2.0.16 in /codex-rs (#2667)
- *(rollout)* Extract rollout module, add listing API, and return file heads (#1634)
- Use the new search tool (#3086)

We were using the preview search tool in the past. We should use the new
one.
- Update guidance on API key permissions (#3112)

Fixes https://github.com/openai/codex/issues/3108
- Correct sandboxed shell tool description (reads allowed anywhere) (#3069)
- Improve @ file search: include specific hidden dirs such as .github, .gitlab (#2981)

# Improve @ file search: include specific hidden dirs

This should close #2980

## What
- Extend `@` fuzzy file search to include select top-level hidden
directories:
`.github`, `.gitlab`, `.circleci`, `.devcontainer`, `.azuredevops`,
`.vscode`, `.cursor`.
- Keep all other hidden directories excluded to avoid noise and heavy
traversals.

## Why
- Common project config lives under these dot-dirs (CI, editor,
devcontainer); users expect `@.github/...` and similar paths to resolve.
- Prior behavior hid all dot-dirs, making these files undiscoverable.

## How
- In `codex-file-search` walker:
  - Enable hidden entries via `WalkBuilder.hidden(false)`.
- Add `filter_entry` to only allow those specific root dot-directories;
other hidden paths remain filtered out.
  - Preserve `.gitignore` semantics and existing exclude handling.

## Local checks
- Ran formatting: `just fmt`
- Ran lint (scoped): `just fix -p codex-file-search`
- Ran tests:
  - `cargo test -p codex-file-search`
  - `cargo test -p codex-tui`

## Readiness
- Branch is up-to-date locally; tests pass; lint/format applied.
- No merge conflicts expected.
- Marking Ready for review.

---------

Signed-off-by: lionelchg <lionel.cheng@hotmail.fr>
- Add a common way to create HTTP client (#3110)

Ensure User-Agent and originator are always sent.
- Introduce Rollout Policy (#3116)

Have a helper function for deciding if we are rolling out a function or
not
- Auto-approve DangerFullAccess patches on non-sandboxed platforms (#2988)

**What?**
Auto-approve patches when `SandboxPolicy::DangerFullAccess` is enabled
on platforms without sandbox support.
Changes in `codex-rs/core/src/safety.rs`: return
`SafetyCheck::AutoApprove { sandbox_type: SandboxType::None }` when no
sandbox is available and DangerFullAccess is set.

**Why?**
On platforms lacking sandbox support, requiring explicit user approval
despite `DangerFullAccess` being explicitly enabled adds friction
without additional safety. This aligns behavior with the stated policy
intent.

**How?**
Extend `assess_patch_safety` match:

* If `get_platform_sandbox()` returns `Some`, keep `AutoApprove {
sandbox_type }`.
* If `None` **and** `SandboxPolicy::DangerFullAccess`, return
`AutoApprove { SandboxType::None }`.
* Otherwise, fall back to `AskUser`.

**Tests**

* Local checks:
  ```bash
cargo test && cargo clippy --tests && cargo fmt -- --config
imports_granularity=Item
  ```
(Additionally: `just fmt`, `just fix -p codex-core`, `cargo check -p
codex-core`.)

**Docs**
No user-facing CLI changes. No README/help updates needed.

**Risk/Impact**
Reduces prompts on non-sandboxed platforms when DangerFullAccess is
explicitly chosen; consistent with policy semantics.

---------

Co-authored-by: Michael Bolin <bolinfest@gmail.com>
- [codex] document `use_experimental_reasoning_summary` toml key config (#3118)

Follow up on https://github.com/openai/codex/issues/3101
- Clean up verbosity config (#3056)
- Fix failing CI (#3130)

In this test, the ChatGPT token path is used, and the auth layer tries
to refresh the token if it thinks the token is “old.” Your helper writes
a fixed last_refresh timestamp that has now aged past the 28‑day
threshold, so the code attempts a real refresh against auth.openai.com,
never reaches the mock, and you end up with
received_requests().await.unwrap() being empty.
- Remove bold the keyword from prompt (#3121)

the model was often including the literal text "Bold the keyword" in
lists.
this guidance doesn't seem particularly useful to me, so just drop it.
- [tui] Update /mcp output (#3134)

# Summary
Quick update to clean up MCP output

## Testing
- [x] Ran locally, confirmed output looked good
- Include originator in authentication URL parameters (#3117)

Associates the client with an authentication session.
- MCP sandbox call (#3128)

I have read the CLA Document and I hereby sign the CLA
- Replay EventMsgs from Response Items when resuming a session with history. (#3123)

### Overview

This PR introduces the following changes:
	1.	Adds a unified mechanism to convert ResponseItem into EventMsg.
2. Ensures that when a session is initialized with initial history, a
vector of EventMsg is sent along with the session configuration. This
allows clients to re-render the UI accordingly.
	3. 	Added integration testing

### Caveats

This implementation does not send every EventMsg that was previously
dispatched to clients. The excluded events fall into two categories:
	•	“Arguably” rolled-out events
Examples include tool calls and apply-patch calls. While these events
are conceptually rolled out, we currently only roll out ResponseItems.
These events are already being handled elsewhere and transformed into
EventMsg before being sent.
	•	Non-rolled-out events
Certain events such as TurnDiff, Error, and TokenCount are not rolled
out at all.

### Future Directions

At present, resuming a session involves maintaining two states:
	•	UI State
Clients can replay most of the important UI from the provided EventMsg
history.
	•	Model State
The model receives the complete session history to reconstruct its
internal state.

This design provides a solid foundation. If, in the future, more precise
UI reconstruction is needed, we have two potential paths:
1. Introduce a third data structure that allows us to derive both
ResponseItems and EventMsgs.
2. Clearly divide responsibilities: the core system ensures the
integrity of the model state, while clients are responsible for
reconstructing the UI.
- Dividing UserMsgs into categories to send it back to the tui (#3127)

This PR does the following:

- divides user msgs into 3 categories: plain, user instructions, and
environment context
- Centralizes adding user instructions and environment context to a
degree
- Improve the integration testing

Building on top of #3123

Specifically this
[comment](https://github.com/openai/codex/pull/3123#discussion_r2319885089).
We need to send the user message while ignoring the User Instructions
and Environment Context we attach.
- *(deps)* Bump wiremock from 0.6.4 to 0.6.5 in /codex-rs (#2666)
- Add session resume picker (--resume) and quick resume (--continue) (#3135)
- Avoid panic when active exec cell area is zero height (#3133)
- [codex] improve handling of reasoning summary (#3138)

<img width="1474" height="289" alt="Screenshot 2025-09-03 at 5 27 19 PM"
src="https://github.com/user-attachments/assets/d6febcdd-fd9c-488c-9e82-348600b1f757"
/>

Fallback to standard behavior when there is no summary in cot, and also
added tests to codify this behavior.
- Add rust-lang.rust-analyzer and vadimcn.vscode-lldb to the list of recommended extensions (#3172)
- Use ⌥⇧⌃ glyphs for key hints on mac (#3143)

#### Summary
- render the edit queued message shortcut with the ⌥ modifier on macOS
builds
- add a helper for status indicator snapshot suffixes
- record macOS-specific snapshots for the status indicator widget
- [codex] move configuration for reasoning summary format to model family config type (#3171)
- Pager pins scroll to bottom (#3167)
- Pause status timer while modals are open (#3131)

Summary:
- pause the status timer while waiting on approval modals
- expose deterministic pause/resume helpers to avoid sleep-based tests
- simplify bottom pane timer handling now that the widget owns the clock
- Prompt to read AGENTS.md files (#3122)
- Clarify test approvals for codex-rs (#3132)
- [mcp-server] Update read config interface (#3093)

## Summary
Follow-up to #3056

This PR updates the mcp-server interface for reading the config settings
saved by the user. At risk of introducing _another_ Config struct, I
think it makes sense to avoid tying our protocol to ConfigToml, as its
become a bit unwieldy. GetConfigTomlResponse was a de-facto struct for
this already - better to make it explicit, in my opinion.

This is technically a breaking change of the mcp-server protocol, but
given the previous interface was introduced so recently in #2725, and we
have not yet even started to call it, I propose proceeding with the
breaking change - but am open to preserving the old endpoint.

## Testing
- [x] Added additional integration test coverage
- *(deps)* Bump uuid from 1.17.0 to 1.18.0 in /codex-rs (#2493)
- Correctly calculate remaining context size (#3190)

We had multiple issues with context size calculation:
1. `initial_prompt_tokens` calculation based on cache size is not
reliable, cache misses might set it to much higher value. For now
hardcoded to a safer constant.
2. Input context size for GPT-5 is 272k (that's where 33% came from).

Fixes.
- Add session resume + history listing;  (#3185)
- Fix approval dialog for large commands (#3087)
- Improve serialization of ServerNotification (#3193)
- Remove AttachImage tui event (#3191)
- Syntax-highlight bash lines (#3142)

i'm not yet convinced i have the best heuristics for what to highlight,
but this feels like a useful step towards something a bit easier to
read, esp. when the model is producing large commands.

<img width="669" height="589" alt="Screenshot 2025-09-03 at 8 21 56 PM"
src="https://github.com/user-attachments/assets/b9cbcc43-80e8-4d41-93c8-daa74b84b331"
/>

also a fairly significant refactor of our line wrapping logic.
- [BREAKING] Stop loading project .env files (#3184)

Loading project local .env often loads settings that break codex cli.

Fixes: https://github.com/openai/codex/issues/3174
- ZSH on UNIX system and better detection (#3187)
- Never store requests (#3212)

When item ids are sent to Responses API it will load them from the
database ignoring the provided values. This adds extra latency.

Not having the mode to store requests also allows us to simplify the
code.

## Breaking change

The `disable_response_storage` configuration option is removed.
- Hide resume until it's complete (#3218)

Hide resume functionality until it's fully done.
- Added logic to cancel pending oauth login to free up localhost port (#3217)

This PR addresses an issue that several users have reported. If the
local oauth login server in one codex instance is left running (e.g. the
user abandons the oauth flow), a subsequent codex instance will receive
an error when attempting to log in because the localhost port is already
in use by the dangling web server from the first instance.

This PR adds a cancelation mechanism that the second instance can use to
abort the first login attempt and free up the port.
- Added CLI version to `/status` output (#3223)

This PR adds the CLI version to the `/status` output.

This addresses feature request #2767
- [codex] respect overrides for model family configuration from toml file (#3176)
- Rewrite codex-rs/scripts/create_github_release.sh in Python (#3226)
- Change create_github_release to create a fresh clone in a temp directory (#3228)
- Use gh instead of git to do work to avoid overhead of a local clone (#3230)
- Move token usage/context information to session level (#3221)

Move context information into the main loop so it can be used to
interrupt the loop or start auto-compaction.
- Clear non-empty prompts with ctrl + c (#3285)

This updates the ctrl + c behavior to clear the current prompt if there
is text and you press ctrl + c.

I also updated the ctrl + c hint text to show `^c to interrupt` instead
of `^c to quit` if there is an active conversation.

Two things I don't love:
1. You can currently interrupt a conversation with escape or ctrl + c
(not related to this PR and maybe fine)
2. The bottom row hint text always says `^c to quit` but this PR doesn't
really make that worse.




https://github.com/user-attachments/assets/6eddadec-0d84-4fa7-abcb-d6f5a04e5748


Fixes https://github.com/openai/codex/issues/3126
- Use ConversationId instead of raw Uuids (#3282)

We're trying to migrate from `session_id: Uuid` to `conversation_id:
ConversationId`. Not only does this give us more type safety but it
unifies our terminology across Codex and with the implementation of
session resuming, a conversation (which can span multiple sessions) is
more appropriate.

I started this impl on https://github.com/openai/codex/pull/3219 as part
of getting resume working in the extension but it's big enough that it
should be broken out.
- Fix broken link to the "Memory with AGENTS.md" section in codex/README.md (#3300)
- *(deps)* Bump insta from 1.43.1 to 1.43.2 in /codex-rs (#3294)
- *(deps)* Bump tree-sitter from 0.25.8 to 0.25.9 in /codex-rs (#3295)
- *(deps)* Bump clap from 4.5.45 to 4.5.47 in /codex-rs (#3296)
- *(deps)* Bump image from 0.25.6 to 0.25.8 in /codex-rs (#3297)
- Paste with ctrl+v checks file_list (#3211)
- Upgrade to actions/setup-node@v5 (#3316)
- Add a getUserAgent MCP method (#3320)

This will allow the extension to pass this user agent + a suffix for its
requests
- Highlight Proposed Command preview (#3319)

#### Summary
- highlight proposed command previews with the shared bash syntax
highlighter
- keep the Proposed Command section consistent with other execution
renderings
- Format large numbers in a more readable way. (#2046)

- In the bottom line of the TUI, print the number of tokens to 3 sigfigs
  with an SI suffix, e.g. "1.23K".
- Elsewhere where we print a number, I figure it's worthwhile to print
  the exact number, because e.g. it's a summary of your session. Here we print
  the numbers comma-separated.
- Generate more typescript types and return conversation id with ConversationSummary (#3219)

This PR does multiple things that are necessary for conversation resume
to work from the extension. I wanted to make sure everything worked so
these changes wound up in one PR:
1. Generate more ts types
2. Resume rollout history files rather than create a new one every time
it is resumed so you don't see a duplicate conversation in history for
every resume. Chatted with @aibrahim-oai to verify this
3. Return conversation_id in conversation summaries
4. [Cleanup] Use serde and strong types for a lot of the rollout file
parsing
- Improve explanation of how the shell handles quotes in config.md (#3169)

* Clarify how the shell's handling of quotes affects the interpretation
of TOML values in `--config`/`-c`
* Provide examples of the right way to pass complex TOML values
* The previous explanation incorrectly demonstrated how to pass TOML
values to `--config`/`-c` (misunderstanding how the shell’s handling of
quotes affects things) and would result in invalid invocations of
`codex`.
- Allow mach-lookup for com.apple.system.opendirectoryd.libinfo (#3334)

in the base sandbox policy. this is [allowed in Chrome
renderers](https://source.chromium.org/chromium/chromium/src/+/main:sandbox/policy/mac/common.sb;l=266;drc=7afa0043cfcddb3ef9dafe5acbfc01c2f7e7df01),
so I feel it's fairly safe.
- Try switching to `cargo nextest` to speed up CI builds (#3323)
- Tweak "failed to find expected lines" message in apply_patch (#3374)

It was hard for me to read the expected lines as a `["one", "two",
"three"]` array, maybe not so hard for the model but probably not having
to un-escape in its head would help it out :)

Co-authored-by: Codex <199175422+chatgpt-codex-connector[bot]@users.noreply.github.com>
- No fail fast (#3387)

Add --no-fail-fast to the new `nextest`
- Replace config.responses_originator_header_internal_override with CODEX_INTERNAL_ORIGINATOR_OVERRIDE_ENV_VAR (#3388)

The previous config approach had a few issues:
1. It is part of the config but not designed to be used externally
2. It had to be wired through many places (look at the +/- on this PR
3. It wasn't guaranteed to be set consistently everywhere because we
don't have a super well defined way that configs stack. For example, the
extension would configure during newConversation but anything that
happened outside of that (like login) wouldn't get it.

This env var approach is cleaner and also creates one less thing we have
to deal with when coming up with a better holistic story around configs.

One downside is that I removed the unit test testing for the override
because I don't want to deal with setting the global env or spawning
child processes and figuring out how to introspect their originator
header. The new code is sufficiently simple and I tested it e2e that I
feel as if this is still worth it.
- Do not send reasoning item IDs (#3390)

Response API doesn't require IDs on reasoning items anymore. 

Fixes: https://github.com/openai/codex/issues/3292
- Include apply_patch tool for oss models from gpt-oss providers with different naming convention (e.g. `openai/gpt-oss-*`) (#2811)

Model providers like Groq, Openrouter, AWS Bedrock, VertexAI and others
typically prefix the name of gpt-oss models with `openai`, e.g.
`openai/gpt-oss-120b`.

This PR is to match the model name slug using `contains` instead of
`starts_with` to ensure that the `apply_patch` tool is included in the
tools for models names like `openai/gpt-oss-120b`

Without this, the gpt-oss models will often try to call the
`apply_patch` tool directly instead of via the `shell` command, leading
to validation errors.

I have run all the local checks.

Note: The gpt-oss models from non-Ollama providers are typically run via
a profile with a different base_url (instead of with the `--oss` flag)

---------

Co-authored-by: Andrew Tan <andrewtan@Andrews-Mac.local>
- Alt+delete deletes the word to the right of the cursor (delete_forward_word) (#3394)

This mirrors alt+backspace, which deletes to the left of the cursor.
- Introduce rollout items  (#3380)

This PR introduces Rollout items. This enable us to rollout eventmsgs
and session meta.

This is mostly #3214 with rebase on main
- Set a user agent suffix when used as a mcp server (#3395)

This automatically adds a user agent suffix whenever the CLI is used as
a MCP server
- Move initial history to protocol (#3422)

To fix an edge case of forking then resuming

#3419
- Added images to `UserMessageEvent` (#3400)

This PR adds an `images` field to the existing `UserMessageEvent` so we
can encode zero or more images associated with a user message. This
allows images to be restored when conversations are restored.
- Fix codex exec heading typo (#2703)
- Remove a broken link to prompting_guide.md in docs/getting-started.md (#2858)

The file no longer exists. We've been receiving this feedback several
times.
- https://github.com/openai/codex/issues/2374
- https://github.com/openai/codex/issues/2810
- https://github.com/openai/codex/issues/2826

My previous PR https://github.com/openai/codex/pull/2413 for this issue
restored the file but now it's compatible with the current file
structure. Thus, let's simply delete the link.
- Improved resiliency of two auth-related tests (#3427)

This PR improves two existing auth-related tests. They were failing when
run in an environment where an `OPENAI_API_KEY` env variable was
defined. The change makes them more resilient.
- Replace tui_markdown with a custom markdown renderer (#3396)

Also, simplify the streaming behavior.

This fixes a number of display issues with streaming markdown, and paves
the way for better markdown features (e.g. customizable styles, syntax
highlighting, markdown-aware wrapping).

Not currently supported:
- footnotes
- tables
- reference-style links
- Back out "feat: POSIX unification and snapshot sessions (#3179)" (#3430)

This reverts https://github.com/openai/codex/pull/3179.

#3179 appears to introduce a regression where sourcing dotfiles causes a
bunch of activity in the title bar (and potentially slows things down?)


https://github.com/user-attachments/assets/a68f7fb3-0749-4e0e-a321-2aa6993e01da

Verified this no longer happens after backing out #3179.

Original commit changeset: 62bd0e3d9d5a
- Persist model & reasoning changes (#2799)

Persists `/model` changes across both general and profile-specific
sessions.
- Make user_agent optional (#3436)

# External (non-OpenAI) Pull Request Requirements

Currently, mcp server fail to start with:
```
🖐  MCP client for `<CLIENT>` failed to start: missing field `user_agent`
````

It isn't clear to me yet why this is happening. My understanding is that
this struct is simply added as a new field to the response but this
should fix it until I figure out the full story here.

<img width="714" height="262" alt="CleanShot 2025-09-10 at 13 58 59"
src="https://github.com/user-attachments/assets/946b1313-5c1c-43d3-8ae8-ecc3de3406fc"
/>
- Put workspace roots in the environment context (#3375)

to keep the tool description constant when the writable roots change.
- Unified execution (#3288)

## Unified PTY-Based Exec Tool

Note: this requires to have this flag in the config:
`use_experimental_unified_exec_tool=true`

- Adds a PTY-backed interactive exec feature (“unified_exec”) with
session reuse via
  session_id, bounded output (128 KiB), and timeout clamping (≤ 60 s).
- Protocol: introduces ResponseItem::UnifiedExec { session_id,
arguments, timeout_ms }.
- Tools: exposes unified_exec as a function tool (Responses API);
excluded from Chat
  Completions payload while still supported in tool lists.
- Path handling: resolves commands via PATH (or explicit paths), with
UTF‑8/newline‑aware
  truncation (truncate_middle).
- Tests: cover command parsing, path resolution, session
persistence/cleanup, multi‑session
  isolation, timeouts, and truncation behavior.
- Change forking to read the rollout from file (#3440)

This PR changes get history op to get path. Then, forking will use a
path. This will help us have one unified codepath for resuming/forking
conversations. Will also help in having rollout history in order. It
also fixes a bug where you won't see the UI when resuming after forking.
- Rust-release.yml should update the latest-alpha-cli branch (#3458)
- Sort replacements and add regression tests (#3425)
- Simplify auth flow and reconcile differences between ChatGPT and API Key auth (#3189)

This PR does the following:
* Adds the ability to paste or type an API key.
* Removes the `preferred_auth_method` config option. The last login
method is always persisted in auth.json, so this isn't needed.
* If OPENAI_API_KEY env variable is defined, the value is used to
prepopulate the new UI. The env variable is otherwise ignored by the
CLI.
* Adds a new MCP server entry point "login_api_key" so we can implement
this same API key behavior for the VS Code extension.
<img width="473" height="140" alt="Screenshot 2025-09-04 at 3 51 04 PM"
src="https://github.com/user-attachments/assets/c11bbd5b-8a4d-4d71-90fd-34130460f9d9"
/>
<img width="726" height="254" alt="Screenshot 2025-09-04 at 3 51 32 PM"
src="https://github.com/user-attachments/assets/6cc76b34-309a-4387-acbc-15ee5c756db9"
/>
- NIT unified exec (#3479)

Fix the default value of the experimental flag of unified_exec
- Add Compact and Turn Context to the rollout items (#3444)

Adding compact and turn context to the rollout items

based on #3440
- Clear composer on fork (#3445)

Fixes this

<img width="344" height="51" alt="image"
src="https://github.com/user-attachments/assets/f227d338-b044-4f8d-bf07-87499b4230d8"
/>
- Assign the entire gpt-5 model family same characteristics (#3490)

So the context size indicator is displayed.
- Enable clippy::redundant_clone (#3489)
- Add more detailed documentation on MCP server usage (#3345)

Adds further information on how to get started with `codex mcp`:
- Tool details and parameter references
- Quickstart with example using MCP inspector.
- Use PlanType enum when formatting usage-limit CTA (#3495)

- Started using Play type struct
- Added CTA for team/business 
- Refactored a bit to unify the logic
- Default to image (#3501)
- Log cf-ray header in client traces (#3488)

## Summary
- log the `cf-ray` header when tracing HTTP responses in the Codex
client
- keep existing response status logging unchanged

## Testing
- just fmt
- just fix -p codex-core
- cargo test -p codex-core *(fails:
suite::client::azure_overrides_assign_properties_used_for_responses_url,
suite::client::env_var_overrides_loaded_auth)*

------
https://chatgpt.com/codex/tasks/task_i_68c31640dacc83209be131baf91611cd
- *(readme)* IDE (#3494)
- Add `just test`, which runs `cargo nextest` (#3508)
- MCP Documentation Changes Requests in Code Review (#3507)

Add in review changes from @bolinfest that were dropped due to
auto-merge (#3345).
- Add spacing to timer duration formats (#3471)

<img width="426" height="28" alt="image"
src="https://github.com/user-attachments/assets/b281aca3-3c8d-4b88-a017-5d2f8ea9f3d5"
/>
- Fix model save (#3525)
- Standardize shell description (#3514)

## Summary
Standardizes the shell description across sandbox_types, since we cover
this in the prompt, and have moved necessary details (like
network_access and writeable workspace roots) to EnvironmentContext
messages.

## Test Plan
- [x] updated unit tests
- Add spacing between dropdown headers and items (#3472)

<img width="927" height="194" alt="image"
src="https://github.com/user-attachments/assets/f4cb999b-16c3-448a-aed4-060bed8b96dd"
/>

<img width="1246" height="205" alt="image"
src="https://github.com/user-attachments/assets/5d9ba5bd-0c02-46da-a809-b583a176528a"
/>
- Add spacing before composer footer hints (#3469)

<img width="647" height="82" alt="image"
src="https://github.com/user-attachments/assets/867eb5d9-3076-4018-846e-260a50408185"
/>
- Update interruption error message styling (#3470)

<img width="497" height="76" alt="image"
src="https://github.com/user-attachments/assets/a1ad279d-1d01-41cd-ac14-b3343a392563"
/>

<img width="493" height="74" alt="image"
src="https://github.com/user-attachments/assets/baf487ba-430e-40fe-8944-2071ec052962"
/>
- If a command parses as a patch, do not attempt to run it (#3382)

sometimes the model forgets to actually invoke `apply_patch` and puts a
patch as the script body. trying to execute this as bash sometimes
creates files named `,` or `{` or does other unknown things, so catch
this situation and return an error to the model.
- Add Azure Responses API workaround (#3528)

Azure Responses API doesn't work well with store:false and response
items.

If store = false and id is sent an error is thrown that ID is not found
If store = false and id is not sent an error is thrown that ID is
required

Add detection for Azure urls and add a workaround to preserve reasoning
item IDs and send store:true
- Expand default sandbox (#3483)
- Fix NUX UI (#3534)

Fix NUX UI
- Review Mode (Core) (#3401)

## 📝 Review Mode -- Core

This PR introduces the Core implementation for Review mode:

- New op `Op::Review { prompt: String }:` spawns a child review task
with isolated context, a review‑specific system prompt, and a
`Config.review_model`.
- `EnteredReviewMode`: emitted when the child review session starts.
Every event from this point onwards reflects the review session.
- `ExitedReviewMode(Option<ReviewOutputEvent>)`: emitted when the review
finishes or is interrupted, with optional structured findings:

```json
{
  "findings": [
    {
      "title": "<≤ 80 chars, imperative>",
      "body": "<valid Markdown explaining *why* this is a problem; cite files/lines/functions>",
      "confidence_score": <float 0.0-1.0>,
      "priority": <int 0-3>,
      "code_location": {
        "absolute_file_path": "<file path>",
        "line_range": {"start": <int>, "end": <int>}
      }
    }
  ],
  "overall_correctness": "patch is correct" | "patch is incorrect",
  "overall_explanation": "<1-3 sentence explanation justifying the overall_correctness verdict>",
  "overall_confidence_score": <float 0.0-1.0>
}
```

## Questions

### Why separate out its own message history?

We want the review thread to match the training of our review models as
much as possible -- that means using a custom prompt, removing user
instructions, and starting a clean chat history.

We also want to make sure the review thread doesn't leak into the parent
thread.

### Why do this as a mode, vs. sub-agents?

1. We want review to be a synchronous task, so it's fine for now to do a
bespoke implementation.
2. We're still unclear about the final structure for sub-agents. We'd
prefer to land this quickly and then refactor into sub-agents without
rushing that implementation.
- Always request encrypted cot (#3539)

Otherwise future requests will fail with 500
- Preserve IDs for more item types in azure (#3542)

https://github.com/openai/codex/issues/3509
- W
- Remove descriptions
- Rename
- Default swiftfox models to experimental reasoning summaries (#3560)
- Refactor transcript view to handle HistoryCells (#3538)

No (intended) functional change.

This refactors the transcript view to hold a list of HistoryCells
instead of a list of Lines. This simplifies and makes much of the logic
more robust, as well as laying the groundwork for future changes, e.g.
live-updating history cells in the transcript.

Similar to #2879 in goal. Fixes #2755.
- Handle resuming/forking after compact (#3533)

We need to construct the history different when compact happens. For
this, we need to just consider the history after compact and convert
compact to a response item.

This needs to change and use `build_compact_history` when this #3446 is
merged.
- Add spacing before queued status indicator messages (#3474)

<img width="687" height="174" alt="image"
src="https://github.com/user-attachments/assets/e68f5a29-cb2d-4aa6-9cbd-f492878d8d0a"
/>
- Update output_lines() to take a struct instead of a sequence of bools (#3591)
- Gate model upgrade prompt behind ChatGPT auth (#3586)

- refresh the login_state after onboarding.
- should be on chatgpt for upgrade
- Align user history message prefix width (#3467)

<img width="798" height="340" alt="image"
src="https://github.com/user-attachments/assets/fdd63f40-9c94-4e3a-bce5-2d2f333a384f"
/>
- Include command output when sending timeout to model (#3576)

Being able to see the output helps the model decide how to handle the
timeout.
- Add per-model-family prompts (#3597)

Allows more flexibility in defining prompts.
- Fix swiftfox model selector (#3598)

The model shouldn't be saved with a suffix. The effort is a separate
field.
- Fix flaky windows test (#3564)

There are exactly 4 types of flaky tests in Windows x86 right now:

1. `review_input_isolated_from_parent_history` => Times out waiting for
closing events
2. `review_does_not_emit_agent_message_on_structured_output` => Times
out waiting for closing events
3. `auto_compact_runs_after_token_limit_hit` => Times out waiting for
closing events
4. `auto_compact_runs_after_token_limit_hit` => Also has a problem where
auto compact should add a third request, but receives 4 requests.

1, 2, and 3 seem to be solved with increasing threads on windows runner
from 2 -> 4.

Don't know yet why # 4 is happening, but probably also because of
WireMock issues on windows causing races.
- Enable-resume (#3537)

Adding the ability to resume conversations.
we have one verb `resume`. 

Behavior:

`tui`:
`codex resume`: opens session picker
`codex resume --last`: continue last message
`codex resume <session id>`: continue conversation with `session id`

`exec`:
`codex resume --last`: continue last conversation
`codex resume <session id>`: continue conversation with `session id`

Implementation:
- I added a function to find the path in `~/.codex/sessions/` with a
`UUID`. This is helpful in resuming with session id.
- Added the above mentioned flags
- Added lots of testing
- Only one branch for swiftfox (#3601)

Make each model family have a single branch.
- Fix EventMsg Optional (#3604)
- Append full raw reasoning event text (#3605)

We don't emit correct delta events and only get full reasoning back.
Append it to history.
- Add session header to chat widget (#3592)

<img width="570" height="332" alt="image"
src="https://github.com/user-attachments/assets/ca6dfcb0-f3a1-4b3e-978d-4f844ba77527"
/>
- Skip frames files in codespell (#3606)

Fixes CI
- Fix get_auth_status response when using custom provider (#3581)

This PR addresses an edge-case bug that appears in the VS Code extension
in the following situation:
1. Log in using ChatGPT (using either the CLI or extension). This will
create an `auth.json` file.
2. Manually modify `config.toml` to specify a custom provider.
3. Start a fresh copy of the VS Code extension.

The profile menu in the VS Code extension will indicate that you are
logged in using ChatGPT even though you're not.

This is caused by the `get_auth_status` method returning an
`auth_method: 'chatgpt'` when a custom provider is configured and it
doesn't use OpenAI auth (i.e. `requires_openai_auth` is false). The
method should always return `auth_method: None` if
`requires_openai_auth` is false.

The same bug also causes the NUX (new user experience) screen to be
displayed in the VSCE in this situation.
- Don't show the model for apikey (#3607)
- When logging in using ChatGPT, make sure to overwrite API key (#3611)

When logging in using ChatGPT using the `codex login` command, a
successful login should write a new `auth.json` file with the ChatGPT
token information. The old code attempted to retain the API key and
merge the token information into the existing `auth.json` file. With the
new simplified login mechanism, `auth.json` should have auth information
for only ChatGPT or API Key, not both.

The `codex login --api-key <key>` code path was already doing the right
thing here, but the `codex login` command was incorrect. This PR fixes
the problem and adds test cases for both commands.
- Show the header when resuming a conversation (#3615)
- Revert "refactor transcript view to handle HistoryCells" (#3614)

Reverts openai/codex#3538
It panics on forking first message. It also calculates the index in a
wrong way.
- Update swiftfox_prompt.md (#3624)
- Initial mcp add interface (#3543)

Adds `codex mcp add`, `codex mcp list`, `codex mcp remove`. Currently writes to global config.
- Change animation frames (#3627)

## Description
- Changes animation frames to be smaller
- Cleans up file names and popup logic

## tests
- Passes local CI
- Show abort in the resume (#3629)

Show abort error when resuming a session
- Fix codex resume so flags (cd, model, search, etc.) still work (#3625)

Bug: now we can add flags/config values only before resume. 

`codex -m gpt-5 resume` works

However, `codex resume -m gpt-5` should also work.

This PR is following this
[approach](https://stackoverflow.com/questions/76408952/rust-clap-re-use-same-arguments-in-different-subcommand)
in doing so.

I didn't convert those flags to global because we have `codex login`
that shouldn't expect them.
- Login flow polish (#3632)

# Description
- Update sign in flow

# Tests
- Passes CI

---------

Co-authored-by: Michael Bolin <mbolin@openai.com>
- *(deps)* Bump slab from 0.4.10 to 0.4.11 in /codex-rs (#3635)
- *(deps)* Bump tracing-subscriber from 0.3.19 to 0.3.20 in /codex-rs (#3620)
- Set `prerelease:true` for now (#3645)
- Rename (#3648)
- Add file reference guidelines to gpt-5 prompt (#3651)
- Notifications on approvals and turn end (#3329)

uses OSC 9 to notify when a turn ends or approval is required. won't
work in vs code or terminal.app but iterm2/kitty/wezterm supports it :)
- Restore prerelease logic in rust-release.yml (#3659)
- *(deps)* Bump serde_json from 1.0.143 to 1.0.145 in /codex-rs (#3617)
- Simplify dep so serde=1 in Cargo.toml (#3664)
- *(deps)* Bump serde from 1.0.219 to 1.0.223 in /codex-rs (#3618)
- *(deps)* Bump wildmatch from 2.4.0 to 2.5.0 in /codex-rs (#3619)
- Update azure model provider example (#3680)

Make the section linkable.
- Persist search items (#3745)

Let's record the search items because they are part of the history.
- Review mode core updates (#3701)

1. Adds the environment prompt (including cwd) to review thread
2. Prepends the review prompt as a user message (temporary fix so the
instructions are not replaced on backend)
3. Sets reasoning to low
4. Sets default review model to `gpt-5-codex`
- Restyle thinking outputs (#3755)

<img width="1205" height="930" alt="Screenshot 2025-09-16 at 2 23 18 PM"
src="https://github.com/user-attachments/assets/bb2494f1-dd59-4bc9-9c4e-740605c999fd"
/>
- Add dev message upon review out (#3758)

Proposal: We want to record a dev message like so:

```
{
      "type": "message",
      "role": "user",
      "content": [
        {
          "type": "input_text",
          "text": "<user_action>
  <context>User initiated a review task. Here's the full review output from reviewer model. User may select one or more comments to resolve.</context>
  <action>review</action>
  <results>
  {findings_str}
  </results>
</user_action>"
        }
      ]
    },
```

Without showing in the chat transcript.

Rough idea, but it fixes issue where the user finishes a review thread,
and asks the parent "fix the rest of the review issues" thinking that
the parent knows about it.

### Question: Why not a tool call?

Because the agent didn't make the call, it was a human. + we haven't
implemented sub-agents yet, and we'll need to think about the way we
represent these human-led tool calls for the agent.
- Add instruction to install missing commands (#3807)
- Use agent reply text in turn notifications (#3756)

Instead of "Agent turn complete", turn-complete notifications now
include the first handful of chars from the agent's final message.
- Update "Codex CLI harness, sandboxing, and approvals" section (#3822)
- Ignore tests for now (#3777)
- Switch to uuid_v7 and tighten ConversationId usage (#3819)

Make sure conversations have a timestamp.
- Unify animations (#3729)

Unify the animation in a single code and add the CTRL + . in the
onboarding
- Make ESC button work when auto-compaction (#3857)

Only emit a task finished when the compaction comes from a `/compact`
- Hint for codex resume on tui exit (#3757)

<img width="931" height="438" alt="Screenshot 2025-09-16 at 4 25 19 PM"
src="https://github.com/user-attachments/assets/ccfb8df1-feaf-45b4-8f7f-56100de916d5"
/>
- Use tokio mutex and async function to prevent blocking a worker (#3850)
- Reland "refactor transcript view to handle HistoryCells" (#3753)

Reland of #3538
- Use a unified shell tell to not break cache (#3814)

Currently, we change the tool description according to the sandbox
policy and approval policy. This breaks the cache when the user hits
`/approvals`. This PR does the following:
- Always use the shell with escalation parameter:
- removes `create_shell_tool_for_sandbox` and always uses unified tool
via `create_shell_tool`
- Reject the func call when the model uses escalation parameter when it
cannot.
- Move responses mocking helpers to a shared lib (#3878)

These are generally useful
- Use helpers instead of fixtures (#3888)

Move to using test helper method everywhere.
- Don't clear screen on startup (#3925)
- Cache keyboard enhancement detection before event streams (#3950)

Hopefully fixes incorrectly showing ^J instead of Shift+Enter in the key
hints occasionally.
- Forward Rate limits to the UI (#3965)

We currently get information about rate limits in the response headers.
We want to forward them to the clients to have better transparency.
UI/UX plans have been discussed and this information is needed.
- Rate limits (#3977)
- Add non_sandbox_test helper (#3880)

Makes tests shorter
- Unify cargo versions (#4044)
- More clippy rules 2 (#4057)
- Timeouts for mcp tool calls (#3959)

defaults to 60sec, overridable with MCP_TOOL_TIMEOUT or on a per-server
basis in the config.
- Add limits to /status (#4053)

Add limits to status

<img width="579" height="430" alt="image"
src="https://github.com/user-attachments/assets/d3794d92-ffca-47be-8011-b4452223cc89"
/>
- Simplify StreamController (#3928)

no intended functional change, just simplifying the code.
- Fix backtracking (#4020)
- Remove /limits after moving to /status (#4055)

Moved to /status #4053
- Change limits warning copy (#4059)
- Clippy on redundant closure (#4058)
- Fix branch mode prompt for /review (#4061)

Updates `/review` branch mode to review against a branch's upstream.
- Change headers and struct of rate limits (#4060)
- Fix pager overlay clear between pages (#3952)

should fix characters sometimes hanging around while scrolling the
transcript.
- Truncate potentially long user messages in compact message. (#4068)

If a prior user message is massive, any future `/compact` task would
fail because we're verbatim copying the user message into the new chat.
- Add notifier tests (#4064)

Proposal:
1. Use anyhow for tests and avoid unwrap
2. Extract a helper for starting a test instance of codex
- Enable auto-compaction for `gpt-5-codex` (#4093)
- Use TestCodex builder in stream retry tests (#4096)

## Summary
- refactor the stream retry integration tests to construct conversations
through `TestCodex`
- remove bespoke config and tempdir setup now handled by the shared
builder

## Testing
- cargo test -p codex-core --test all
stream_error_allows_next_turn::continue_after_stream_error
- cargo test -p codex-core --test all
stream_no_completed::retries_on_early_close

------
https://chatgpt.com/codex/tasks/task_i_68d2b94d83888320bc75a0bc3bd77b49
- Rate limits warning (#4075)

Only show the highest warning rate.
Change the warning threshold
- Compact do not modify instructions  (#4088)
- Refactor codex card layout (#4069)

Refactor it to be used in status
- Use anyhow::Result in tests for error propagation (#4105)
- Add exec output-schema parameter (#4079)

Adds structured output to `exec` via the `--structured-output`
parameter.
- Send limits when getting rate limited (#4102)

Users need visibility on rate limits when they are rate limited.
- [exec] add include-plan-tool flag and print it nicely (#3461)

### Summary
Sometimes in exec runs, we want to allow the model to use the
`update_plan` tool, but that's not easily configurable. This change adds
a feature flag for this, and formats the output so it's human-readable

## Test Plan
<img width="1280" height="354" alt="Screenshot 2025-09-11 at 12 39
44 AM"
src="https://github.com/user-attachments/assets/72e11070-fb98-47f5-a784-5123ca7333d9"
/>
- *(deps)* Bump serde from 1.0.224 to 1.0.226 in /codex-rs (#4031)
- *(deps)* Bump log from 0.4.27 to 0.4.28 in /codex-rs (#4027)
- *(deps)* Bump tempfile from 3.20.0 to 3.22.0 in /codex-rs (#4030)
- Update auto compact to 250k (#4135)
- Drop instruction override for auto-compact (#4137)
- Extract readiness in a dedicated utils crate (#4140)
- Add Reset in for rate limits (#4111)

- Parse the headers
- Reorganize the struct because it's getting too long
- show the resets at in the tui

<img width="324" height="79" alt="image"
src="https://github.com/user-attachments/assets/ca15cd48-f112-4556-91ab-1e3a9bc4683d"
/>
- 350k tokens (#4156)
- Upgrade to Rust 1.90 (#4124)
- Remove `once_cell` dependency from multiple crates (#4154)
- *(deps)* Bump unicode-width from 0.1.14 to 0.2.1 in /codex-rs (#2156)
- *(deps)* Bump chrono from 0.4.41 to 0.4.42 in /codex-rs (#4028)
- Simplify tool implemetations (#4160)

Use Result<String, FunctionCallError> for all tool handling code and
rely on error propagation instead of creating failed items everywhere.
- Hide the status indicator when the answer stream starts (#4101)

This eliminates a "bounce" at the end of streaming where we hide the
status indicator at the end of the turn and the composer moves up two
lines.

Also, simplify streaming further by removing the HistorySink and
inverting control, and collapsing a few single-element structures.
- Improve MCP tool call styling (#3871)

<img width="760" height="213" alt="Screenshot 2025-09-18 at 12 29 15 PM"
src="https://github.com/user-attachments/assets/48a205b7-b95a-4988-8c76-efceb998dee7"
/>
- Pageless session list (#3194)
- Adds a windows-specific method to check if a command is safe (#4119)

refactors command_safety files into its own package, so we can add
platform-specific ones
Also creates a windows-specific of `is_known_safe_command` that just
returns false always, since that is what happens today.
- Drop unused values from env_flags (#4188)
- Send text parameter for non-gpt-5 models (#4195)

We had a hardcoded check for gpt-5 before.

Fixes: https://github.com/openai/codex/issues/4181
- Update codespell action to v2.1 in workflow (#4205)
- Full state refactor (#4174)
- Revamp /status (#4196)

<img width="543" height="520" alt="image"
src="https://github.com/user-attachments/assets/bbc0eec0-e40b-45e7-bcd0-a997f8eeffa2"
/>
- Refactor attempt_stream_responses() out of stream_responses() (#4194)
- Add explicit codex exec events (#4177)

This pull request add a new experimental format of JSON output.

You can try it using `codex exec --experimental-json`.

Design takes a lot of inspiration from Responses API items and stream
format.

# Session and items
Each invocation of `codex exec` starts or resumes a session. 

Session contains multiple high-level item types:
1. Assistant message 
2. Assistant thinking 
3. Command execution 
4. File changes
5. To-do lists
6. etc.

# Events 
Session and items are going through their life cycles which is
represented by events.

Session is `session.created` or `session.resumed`
Items are `item.added`, `item.updated`, `item.completed`,
`item.require_approval` (or other item types like `item.output_delta`
when we need streaming).

So a typical session can look like:

<details>

```
{
  "type": "session.created",
  "session_id": "01997dac-9581-7de3-b6a0-1df8256f2752"
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_0",
    "item_type": "assistant_message",
    "text": "I’ll locate the top-level README and remove its first line. Then I’ll show a quick summary of what changed."
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_1",
    "item_type": "command_execution",
    "command": "bash -lc ls -la | sed -n '1,200p'",
    "aggregated_output": "pyenv: cannot rehash: /Users/pakrym/.pyenv/shims isn't writable\ntotal 192\ndrwxr-xr-x@  33 pakrym  staff   1056 Sep 24 14:36 .\ndrwxr-xr-x   41 pakrym  staff   1312 Sep 24 09:17 ..\n-rw-r--r--@   1 pakrym  staff      6 Jul  9 16:16 .codespellignore\n-rw-r--r--@   1 pakrym  staff    258 Aug 13 09:40 .codespellrc\ndrwxr-xr-x@   5 pakrym  staff    160 Jul 23 08:26 .devcontainer\n-rw-r--r--@   1 pakrym  staff   6148 Jul 22 10:03 .DS_Store\ndrwxr-xr-x@  15 pakrym  staff    480 Sep 24 14:38 .git\ndrwxr-xr-x@  12 pakrym  staff    384 Sep  2 16:00 .github\n-rw-r--r--@   1 pakrym  staff    778 Jul  9 16:16 .gitignore\ndrwxr-xr-x@   3 pakrym  staff     96 Aug 11 09:37 .husky\n-rw-r--r--@   1 pakrym  staff    104 Jul  9 16:16 .npmrc\n-rw-r--r--@   1 pakrym  staff     96 Sep  2 08:52 .prettierignore\n-rw-r--r--@   1 pakrym  staff    170 Jul  9 16:16 .prettierrc.toml\ndrwxr-xr-x@   5 pakrym  staff    160 Sep 14 17:43 .vscode\ndrwxr-xr-x@   2 pakrym  staff     64 Sep 11 11:37 2025-09-11\n-rw-r--r--@   1 pakrym  staff   5505 Sep 18 09:28 AGENTS.md\n-rw-r--r--@   1 pakrym  staff     92 Sep  2 08:52 CHANGELOG.md\n-rw-r--r--@   1 pakrym  staff   1145 Jul  9 16:16 cliff.toml\ndrwxr-xr-x@  11 pakrym  staff    352 Sep 24 13:03 codex-cli\ndrwxr-xr-x@  38 pakrym  staff   1216 Sep 24 14:38 codex-rs\ndrwxr-xr-x@  18 pakrym  staff    576 Sep 23 11:01 docs\n-rw-r--r--@   1 pakrym  staff   2038 Jul  9 16:16 flake.lock\n-rw-r--r--@   1 pakrym  staff   1434 Jul  9 16:16 flake.nix\n-rw-r--r--@   1 pakrym  staff  10926 Jul  9 16:16 LICENSE\ndrwxr-xr-x@ 465 pakrym  staff  14880 Jul 15 07:36 node_modules\n-rw-r--r--@   1 pakrym  staff    242 Aug  5 08:25 NOTICE\n-rw-r--r--@   1 pakrym  staff    578 Aug 14 12:31 package.json\n-rw-r--r--@   1 pakrym  staff    498 Aug 11 09:37 pnpm-lock.yaml\n-rw-r--r--@   1 pakrym  staff     58 Aug 11 09:37 pnpm-workspace.yaml\n-rw-r--r--@   1 pakrym  staff   2402 Jul  9 16:16 PNPM.md\n-rw-r--r--@   1 pakrym  staff   4393 Sep 12 14:36 README.md\ndrwxr-xr-x@   4 pakrym  staff    128 Sep 18 09:28 scripts\ndrwxr-xr-x@   2 pakrym  staff     64 Sep 11 11:34 tmp\n",
    "exit_code": 0,
    "status": "completed"
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_2",
    "item_type": "reasoning",
    "text": "**Reviewing README.md file**\n\nI've located the README.md file at the root, and it’s 4393 bytes. Now, I need to remove the first line, but first, I should check its content to make sure I’m patching it correctly. I’ll use sed to display the first 20 lines. By reviewing those lines, I can determine exactly what needs to be removed before I proceed with the editing. Let's do this carefully!"
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_3",
    "item_type": "command_execution",
    "command": "bash -lc sed -n '1,40p' README.md",
    "aggregated_output": "<h1 align=\"center\">OpenAI Codex CLI</h1>\n\n<p align=\"center\"><code>npm i -g @openai/codex</code><br />or <code>brew install codex</code></p>\n\n<p align=\"center\"><strong>Codex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.\n</br>\n</br>If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href=\"https://developers.openai.com/codex/ide\">install in your IDE</a>\n</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codex Web</strong>, go to <a href=\"https://chatgpt.com/codex\">chatgpt.com/codex</a></p>\n\n<p align=\"center\">\n  <img src=\"./.github/codex-cli-splash.png\" alt=\"Codex CLI splash\" width=\"80%\" />\n  </p>\n\n---\n\n## Quickstart\n\n### Installing and running Codex CLI\n\nInstall globally with your preferred package manager. If you use npm:\n\n```shell\nnpm install -g @openai/codex\n```\n\nAlternatively, if you use Homebrew:\n\n```shell\nbrew install codex\n```\n\nThen simply run `codex` to get started:\n\n```shell\ncodex\n```\n\n<details>\n<summary>You can also go to the <a href=\"https://github.com/openai/codex/releases/latest\">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>\n\n",
    "exit_code": 0,
    "status": "completed"
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_4",
    "item_type": "reasoning",
    "text": "**Updating README file**\n\nThe user wants to update the README by removing the first line, which is an HTML h1 tag. I'll proceed to delete that line while keeping the rest intact. To do this, I need to use the apply_patch tool and create a patch in the diff format. I’ll take care to include context lines around the change. After removing the header, I’m considering whether to delete the following blank line as well to keep it tidy."
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_5",
    "item_type": "reasoning",
    "text": "**Crafting the patch**\n\nThe user has asked to remove just the first line, without mentioning the blank lines. Technically, leaving a blank line at the start is acceptable, but it's not ideal. Ideally, I'd prefer to remove the first line while maintaining the subsequent blank line. However, if I can also remove that blank line, it could tidy things up even more. Still, since the user's request is strictly to remove only the first line, I'll stick to that and craft the patch accordingly."
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_6",
    "item_type": "assistant_message",
    "text": "Applying the change by removing the first line from `README.md`."
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_7",
    "item_type": "file_change",
    "changes": [
      {
        "path": "/Users/pakrym/code/codex/README.md",
        "kind": "update"
      }
    ],
    "status": "completed"
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_8",
    "item_type": "reasoning",
    "text": "**Validating README Update**\n\nI updated the README.md and now I need to validate that the first line has been successfully removed. To do this, I'll check the first 10 lines of the document to ensure everything looks correct. It’s always good to double-check these kinds of changes, so I’m going to take that extra step to make sure the update was executed properly. Let's see what those initial lines say!"
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_9",
    "item_type": "command_execution",
    "command": "bash -lc sed -n '1,20p' README.md",
    "aggregated_output": "\n<p align=\"center\"><code>npm i -g @openai/codex</code><br />or <code>brew install codex</code></p>\n\n<p align=\"center\"><strong>Codex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.\n</br>\n</br>If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href=\"https://developers.openai.com/codex/ide\">install in your IDE</a>\n</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codex Web</strong>, go to <a href=\"https://chatgpt.com/codex\">chatgpt.com/codex</a></p>\n\n<p align=\"center\">\n  <img src=\"./.github/codex-cli-splash.png\" alt=\"Codex CLI splash\" width=\"80%\" />\n  </p>\n\n---\n\n## Quickstart\n\n### Installing and running Codex CLI\n\nInstall globally with your preferred package manager. If you use npm:\n\n",
    "exit_code": 0,
    "status": "completed"
  }
}
{
  "type": "item.completed",
  "item": {
    "id": "itm_10",
    "item_type": "assistant_message",
    "text": "- Updated `README.md:1` by removing the opening H1 line.\n- The README now starts with the install instructions paragraph.\n\nWant me to also remove the leading blank line at the top?"
  }
}
```

</details>

The idea is to give users fully formatted items they can use directly in
their rendering/application logic and avoid having them building up
items manually based on events (unless they want to for streaming).

This PR implements only the `item.completed` payload for some event
types, more event types and item types to come.

---------

Co-authored-by: Michael Bolin <mbolin@openai.com>
- Fix error message (#4204)

Co-authored-by: Ahmed Ibrahim <aibrahim@openai.com>
- Make tests pass cleanly in sandbox (#4067)

This changes the reqwest client used in tests to be sandbox-friendly,
and skips a bunch of other tests that don't work inside the
sandbox/without network.
- [codex exec] Add item.started and support it for command execution (#4250)

Adds a new `item.started` event to `codex exec` and implements it for
command_execution item type.

```jsonl
{"type":"session.created","session_id":"019982d1-75f0-7920-b051-e0d3731a5ed8"}
{"type":"item.completed","item":{"id":"item_0","item_type":"reasoning","text":"**Executing commands securely**\n\nI'm thinking about how the default harness typically uses \"bash -lc,\" while historically \"bash\" is what we've been using. The command should be executed as a string in our CLI, so using \"bash -lc 'echo hello'\" is optimal but calling \"echo hello\" directly feels safer. The sandbox makes sure environment variables like CODEX_SANDBOX_NETWORK_DISABLED=1 are set, so I won't ask for approval. I just need to run \"echo hello\" and correctly present the output."}}
{"type":"item.completed","item":{"id":"item_1","item_type":"reasoning","text":"**Preparing for tool calls**\n\nI realize that I need to include a preamble before making any tool calls. So, I'll first state the preamble in the commentary channel, then proceed with the tool call. After that, I need to present the final message along with the output. It's possible that the CLI will show the output inline, but I must ensure that I present the result clearly regardless. Let's move forward and get this organized!"}}
{"type":"item.completed","item":{"id":"item_2","item_type":"assistant_message","text":"Running `echo` to confirm shell access and print output."}}
{"type":"item.started","item":{"id":"item_3","item_type":"command_execution","command":"bash -lc echo hello","aggregated_output":"","exit_code":null,"status":"in_progress"}}
{"type":"item.completed","item":{"id":"item_3","item_type":"command_execution","command":"bash -lc echo hello","aggregated_output":"hello\n","exit_code":0,"status":"completed"}}
{"type":"item.completed","item":{"id":"item_4","item_type":"assistant_message","text":"hello"}}
```
- Fixed login failure with API key in IDE extension when a `.codex` directory doesn't exist (#4258)

This addresses bug #4092

Testing:
* Confirmed error occurs prior to fix if logging in using API key and no
`~/.codex` directory exists
* Confirmed after fix that `~/.codex` directory is properly created and
error doesn't occur
- Add codex exec testing helpers (#4254)

Add a shortcut to create working directories and run codex exec with
fake server.
- Actually mount sse once (#4264)

Mock server was responding with the same result many times.
- Add potentially dangerous command check (#4211)
- State - 2 (#4229)
- Refactor the footer logic to a new file (#4259)

This will help us have more control over the footer

---------

Co-authored-by: pakrym-oai <pakrym@openai.com>
- Move approvals to use ListSelectionView (#4275)

Unify selection menus:
- Move approvals to the vertical menu `ListSelectionView`
- Add header section to `ListSelectionView`

<img width="502" height="214" alt="image"
src="https://github.com/user-attachments/assets/f4b43ddf-3549-403c-ad9e-a523688714e4"
/>

<img width="748" height="214" alt="image"
src="https://github.com/user-attachments/assets/f94ac7b5-dc94-4dc0-a1df-7a8e3ba2453b"
/>

---------

Co-authored-by: pakrym-oai <pakrym@openai.com>
- Show exec output on success with trimmed display (#4113)

- Refactor Exec Cell into its own module
- update exec command rendering to inline the first command line
- limit continuation lines
- always show trimmed output
- Add todo-list tool support (#4255)

Adds a 1-per-turn todo-list item and item.updated event

```jsonl
{"type":"item.started","item":{"id":"item_6","item_type":"todo_list","items":[{"text":"Record initial two-step plan  now","completed":false},{"text":"Update progress to next step","completed":false}]}}
{"type":"item.updated","item":{"id":"item_6","item_type":"todo_list","items":[{"text":"Record initial two-step plan  now","completed":true},{"text":"Update progress to next step","completed":false}]}}
{"type":"item.completed","item":{"id":"item_6","item_type":"todo_list","items":[{"text":"Record initial two-step plan  now","completed":true},{"text":"Update progress to next step","completed":false}]}}
```
- [MCP] Introduce an experimental official rust sdk based mcp client (#4252)

The [official Rust
SDK](https://github.com/modelcontextprotocol/rust-sdk/tree/57fc428c578a1a3fe851ee0838bf068bda120eb3)
has come a long way since we first started our mcp client implementation
5 months ago and, today, it is much more complete than our own
stdio-only implementation.

This PR introduces a new config flag `experimental_use_rmcp_client`
which will use a new mcp client powered by the sdk instead of our own.

To keep this PR simple, I've only implemented the same stdio MCP
functionality that we had but will expand on it with future PRs.

---------

Co-authored-by: pakrym-oai <pakrym@openai.com>
- Dead code removal; remove frame count and stateful render helpers (#4310)
- /status followup (#4304)

- Render `send a message to load usage data` in the beginning of the
session
- Render `data not available yet` if received no rate limits 
- nit case
- Deleted stall snapshots that were moved to
`codex-rs/tui/src/status/snapshots`
- Initial plan
- Implement core .codexignore and .aiignore functionality

Co-authored-by: sealad886 <155285242+sealad886@users.noreply.github.com>
- Add comprehensive CLI flags and integration tests for ignore functionality

Co-authored-by: sealad886 <155285242+sealad886@users.noreply.github.com>
- Add comprehensive documentation and examples for ignore functionality

Co-authored-by: sealad886 <155285242+sealad886@users.noreply.github.com>
- Add .gitignore support with hierarchical precedence: .gitignore > .aiignore > .codexignore

Co-authored-by: sealad886 <155285242+sealad886@users.noreply.github.com>
## [codex-rs-2925136536b06a324551627468d17e959afa18d4-1-rust-v0.2.0-alpha.2] - 2025-06-29
### 🪲 Bug Fixes

- Support pre-release identifiers in tags (#1422)
- Need to check out the branch, not the tag (#1430)

### 💼 Other

- Fix Rust release process so generated .tar.gz source works with Homebrew (#1423)
## [codex-rs-b289c9207090b2e27494545d7b5404e063bd86f3-1-rust-v0.1.0-alpha.4] - 2025-06-28
### 🚀 Features

- Make file search cancellable (#1414)
- Add support for @ to do file search (#1401)
- Introduce --compute-indices flag to codex-file-search (#1419)
- Highlight matching characters in fuzzy file search (#1420)

### 🪲 Bug Fixes

- Add tiebreaker logic for paths when scores are equal (#1400)
- Build with `codegen-units = 1` for profile.release (#1421)

### 💼 Other

- Handle Ctrl+C quit when idle (#1402)

## Summary
- show `Ctrl+C to quit` hint when pressing Ctrl+C with no active task
- exiting with Ctrl+C if the hint is already visible
- clear the hint when tasks begin or other keys are pressed


https://github.com/user-attachments/assets/931e2d7c-1c80-4b45-9908-d119f74df23c



------
https://chatgpt.com/s/cd_685ec8875a308191beaa95886dc1379e

Fixes #1245
- Change `built_in_model_providers` so "openai" is the only "bundled" provider (#1407)
- Change arg from PathBuf to &Path (#1409)
## [codex-rs-6a8a936f75ea44faf05ff4fab0c6a36fc970428d-1-rust-v0.0.2506261603] - 2025-06-26
### 🚀 Features

- *(ts)* Provider‑specific API‑key discovery and clearer Azure guidance (#1324)
- Redesign sandbox config (#1373)
- Add --dangerously-bypass-approvals-and-sandbox (#1384)
- Standalone file search CLI (#1386)
- Show number of tokens remaining in UI (#1388)
- Add support for /diff command (#1389)

### 🪲 Bug Fixes

- Pretty-print the sandbox config in the TUI/exec modes (#1376)

### 💼 Other

- Rename `/clear` to `/new`, make it start an entirely new chat (#1264)
- Update codex-rs/README.md to list new features in the Rust CLI (#1267)
- Responses api support for azure (#1321)
- Install clippy and rustfmt in the devcontainer for Linux development (#1374)
- Install `just` in the devcontainer for Linux development (#1375)
- Rename unless-allow-listed to untrusted (#1378)
- Improve docstring for --full-auto (#1379)
- Rename AskForApproval::UnlessAllowListed to AskForApproval::UnlessTrusted (#1385)
- [Rust] Allow resuming a session that was killed with ctrl + c (#1387)

Previously, if you ctrl+c'd a conversation, all subsequent turns would
400 because the Responses API never got a response for one of its call
ids. This ensures that if we aren't sending a call id by hand, we
generate a synthetic aborted call.

Fixes #1244 


https://github.com/user-attachments/assets/5126354f-b970-45f5-8c65-f811bca8294a
## [codex-rs-5fc3c3023d9f179fb416b2722d1434bac278e916-1-rust-v0.0.2506060849] - 2025-06-06
### 🚀 Features

- Port maybeRedeemCredits() from get-api-key.tsx to login_with_chatgpt.py (#1221)

### 💼 Other

- Ensure next Node.js release includes musl binaries for arm64 Linux (#1232)
## [codex-rs-ac6e1b2661320a631d80aa51bdfa8f1635e0c8fa-1-rust-v0.0.2506052246] - 2025-06-06
### 🪲 Bug Fixes

- Use aarch64-unknown-linux-musl instead of aarch64-unknown-linux-gnu (#1228)
## [codex-rs-121686615fd634e35f3e415896f36908cf8632f9-1-rust-v0.0.2506052203] - 2025-06-06
### 🪲 Bug Fixes

- Include codex-linux-sandbox-aarch64-unknown-linux-musl in the set of release artifacts (#1230)
- Truncate auth.json file before rewriting it (#1231)
## [codex-rs-84eae7b1bc4e3b5420f2d6127b7c17e7a979a5b0-1-rust-v0.0.2506052135] - 2025-06-06
### 🚀 Features

- Add support for login with ChatGPT (#1212)

### 🪲 Bug Fixes

- Support arm64 build for Linux (#1225)

### 💼 Other

- Make tool calls prettier (#1211)
## [codex-rs-45519e12f39777b65c05ed498503ddcb60beb289-1-rust-v0.0.2506030956] - 2025-06-03
### 🚀 Features

- Make reasoning effort/summaries configurable (#1199)

### 🪲 Bug Fixes

- Disable agent reasoning output by default in the GitHub Action (#1183)
- Set `--config hide_agent_reasoning=true` in the GitHub Action (#1185)
- Chat completions API now also passes tools along (#1167)
- Provide tolerance for apply_patch tool (#993)
- Always send full instructions when using the Responses API (#1207)

### 💼 Other

- Update the WORKFLOW_URL in install_native_deps.sh to the latest release (#1190)
- Logging cleanup (#1196)
- Replace regex with regex-lite, where appropriate (#1200)
## [codex-rs-ca8e97fcbcb991e542b8689f2d4eab9d30c399d6-1-rust-v0.0.2505302325] - 2025-05-31
### 🚀 Features

- Add hide_agent_reasoning config option (#1181)
- Show the version when starting Codex (#1182)
## [codex-rs-378d773f3af95384eef51addf560df30aa9fd15f-1-rust-v0.0.2505301630] - 2025-05-30
### 🚀 Features

- Initial import of experimental GitHub Action (#1170)
- For `codex exec`, if PROMPT is not specified, read from stdin if not a TTY (#1178)
- Grab-bag of improvements to `exec` output (#1179)
- Dim the timestamp in the exec output (#1180)

### 🪲 Bug Fixes

- Enable `set positional-arguments` in justfile (#1169)
- Update outdated repo setup in codex.yml (#1171)
- Missed a step in #1171 for codex.yml (#1172)
- Add extra debugging to GitHub Action (#1173)
- Introduce `create_tools_json()` and share it with chat_completions.rs (#1177)
## [codex-rs-dfac02b343605ce61154ab2e075ac6c38f533916-1-rust-v0.0.2505291659] - 2025-05-29
### 🪲 Bug Fixes

- Update justfile to facilitate running CLIs from source and formatting source code (#1163)
- *(codex-rs)* Use codex-mini-latest as default (#1164)

### 💼 Other

- Update GitHub workflow for native artifacts for npm release (#1162)
- Split the config-related portion of codex-rs/README.md into its own config.md file (#1165)
## [codex-rs-b152435fb95e7f1ab197ae2cdde68ae29a7d219b-1-rust-v0.0.2505291458] - 2025-05-29
### 🚀 Features

- Add support for -c/--config to override individual config items (#1137)
- Introduce CellWidget trait (#1148)

### 🪲 Bug Fixes

- Update install_native_deps.sh to pick up the latest release (#1136)
- Honor RUST_LOG in mcp-client CLI and default to DEBUG (#1149)
- Ensure inputSchema for MCP tool always has "properties" field when talking to OpenAI (#1150)
- Introduce ResponseInputItem::McpToolCallOutput variant (#1151)
- Update UI treatment of slash command menu to match that of the TS CLI (#1161)
## [codex-rs-d519bd8bbd1e1fd9efdc5d68cf7bebdec0dd0f28-1-rust-v0.0.2505270918] - 2025-05-27
### 🪲 Bug Fixes

- TUI was not honoring --skip-git-repo-check correctly (#1105)
- Use o4-mini as the default model (#1135)
## [codex-rs-aa156ceac953c3e6f3602e6eb2f61b14ac8adaf3-1-rust-v0.0.2505231205] - 2025-05-23
### 🪲 Bug Fixes

- Forgot to pass codex_linux_sandbox_exe through in cli/src/debug_sandbox.rs (#1095)
## [codex-rs-d2eee362c1c6cdc00bcb5bf1d479823ef33c143a-1-rust-v0.0.2505231137] - 2025-05-23
### 🚀 Features

- Add `codex_linux_sandbox_exe: Option<PathBuf>` field to Config (#1089)

### 🪲 Bug Fixes

- For the @native release of the Node module, use the Rust version by default (#1084)
- Overhaul how we spawn commands under seccomp/landlock on Linux (#1086)
## [codex-rs-6a77484c94956d5cd319da3f8500b178ec93fc90-1-rust-v0.0.2505220956] - 2025-05-22
### 🚀 Features

- Introduce support for shell_environment_policy in config.toml (#1061)
## [codex-rs-79cb07bf70a9036200aa2b61b211fe47ea13184a-1-rust-v0.0.2505212314] - 2025-05-22
### 🚀 Features

- Show Config overview at start of exec (#1073)

### 💼 Other

- Move types out of config.rs into config_types.rs (#1054)
## [codex-rs-68e94c8c08943e1d4a53bd7987e319ba7dbffb74-1-rust-v0.0.2505191609] - 2025-05-19
### 🚀 Features

- Experimental --output-last-message flag to exec subcommand (#1037)
## [codex-rs-c74d7e13e7d8daf3a2493f6216918d5e59a38bed-1-rust-v0.0.2505191518] - 2025-05-19
### 🪲 Bug Fixes

- Persist token after refresh (#1006)
- Add node version check (#1007)

### 💼 Other

- Update install_native_deps.sh to use rust-v0.0.2505171051 (#995)
- `codex --login` + `codex --free` (#998)
- Produce .tar.gz versions of artifacts in addition to .zst (#1036)

### 🛳️ Release

- *(version)* 0.1.2505171619 (#1001)
- *(version)* 0.1.2505172129 (#1008)
## [codex-rs-5ee08335ac690a69035720a798df9865bc5a4278-1-rust-v0.0.2505171051] - 2025-05-17
### 🪲 Bug Fixes

- Do not let Tab keypress flow through to composer when used to toggle focus (#977)
- Ensure the first user message always displays after the session info (#988)
- Artifacts from previous frames were bleeding through in TUI (#989)

### 💼 Other

- Sign in with chatgpt credits (#974)
- Remove unnecessary console log from test (#970)

When running `npm test` on `codex-cli`, the test
`agent-cancel-prev-response.test.ts` logs a significant body of text to
console for no obvious reason.

This is not helpful, as it makes test logs messy and far longer.

This change deletes the `console.log(...)` that produces the behavior.

### 🛳️ Release

- *(version)* 0.1.2505161800 (#978)
## [codex-rs-b5257992b06373acef8b20a4ca25ffc1b96688e2-1-rust-v0.0.2505161708] - 2025-05-17
### 🚀 Features

- Add support for file_opener option in Rust, similiar to #911 (#957)
- Add support for OpenAI tool type, local_shell (#961)
- Make it possible to toggle mouse mode in the Rust TUI (#971)

### 🪲 Bug Fixes

- Diff command for filenames with special characters (#954)
- Apply patch issue when using different cwd (#942)
- Introduce ExtractHeredocError that implements PartialEq (#958)
- Remove file named ">" in the codex-cli folder (#968)
- Use text other than 'TODO' as test example (#969)
- Make codex-mini-latest the default model in the Rust TUI (#972)

### 💼 Other

- Codex-mini-latest (#951)
- Update exec crate to use std::time instead of chrono (#952)
- Session history viewer (#912)
- Sign in with chatgpt (#963)
- Refactor handle_function_call() into smaller functions (#965)
- Fix CLA link in workflow (#964)

## Summary
- fix the CLA link posted by the bot
- docs suggest using an absolute URL:
https://github.com/marketplace/actions/cla-assistant-lite

### 🛳️ Release

- *(version)* 0.1.2505160811 `codex-mini-latest` (#953)
- *(version)* 0.1.2505161243 (#967)
## [codex-rs-8d6a8b308e7457d432564083bb2f577cd39e132b-1-rust-v0.0.2505151627] - 2025-05-15
### 🚀 Features

- Record messages from user in ~/.codex/history.jsonl (#939)

### 💼 Other

- Expose codex_home via Config (#941)
- Pin Rust version to 1.86 and use io::Error::other to prepare for 1.87 (#947)
- Introduce AppEventSender to help fix clippy warnings and update to Rust 1.87 (#948)
## [codex-rs-cb19037ca3822e9b19b51417392f8afc046be607-1-rust-v0.0.2505141652] - 2025-05-14
### 🪲 Bug Fixes

- Properly wrap lines in the Rust TUI (#937)
## [codex-rs-3a70a0bc280734d09448cb08ec05b5c44f7c798e-1-rust-v0.0.2505141337] - 2025-05-14
### 🚀 Features

- Auto-approve nl and support piping to sed (#920)
- Introduce --profile for Rust CLI (#921)
- Ctrl+J for newline in Rust TUI, default to one line of height (#926)
- Add support for commands in the Rust TUI (#935)
- Add mcp subcommand to CLI to run Codex as an MCP server (#934)

### 🪲 Bug Fixes

- Always load version from package.json at runtime (#909)
- Add support for fileOpener in config.json (#911)
- Remember to set lastIndex = 0 on shared RegExp (#918)
- Patch in #366 and #367 for marked-terminal (#916)
- Tweak the label for citations for better rendering (#919)
- Tighten up some logic around session timestamps and ids (#922)
- Change EventMsg enum so every variant takes a single struct (#925)
- Test_dev_null_write() was not using echo as intended (#923)
- Gpt-4.1 apply_patch handling (#930)
- Reasoning default to medium, show workdir when supplied (#931)
- Increase timeout for test_dev_null_write (#933)

### 💼 Other

- Restructure flake for codex-rs (#888)

Right now since the repo is having two different implementations of
codex, flake was updated to work with both typescript implementation and
rust implementation
- Dynamic instructions (#927)
- Add codespell support (config, workflow to detect/not fix) and make it fix some typos (#903)

More about codespell: https://github.com/codespell-project/codespell .

I personally introduced it to dozens if not hundreds of projects already
and so far only positive feedback.

CI workflow has 'permissions' set only to 'read' so also should be safe.

Let me know if just want to take typo fixes in and get rid of the CI

---------

Signed-off-by: Yaroslav O. Halchenko <debian@onerussian.com>
- Move each view used in BottomPane into its own file (#928)
- Handle all cases for EventMsg (#936)

### 🛳️ Release

- *(version)* 0.1.2505140839 (#932)
## [codex-rs-94c47d69a3f92257e7f9717a2044bd55786eb999-1-rust-v0.0.2505121726] - 2025-05-13
### 🪲 Bug Fixes

- Agent instructions were not being included when ~/.codex/instructions.md was empty (#908)
## [codex-rs-9949f6404378db6f54a01bcadb1956e0535d4921-1-rust-v0.0.2505121520] - 2025-05-12
### 🚀 Features

- Include "reasoning" messages in Rust TUI (#892)

### 🪲 Bug Fixes

- Fix border style for BottomPane (#893)
- Normalize paths in resolvePathAgainstWorkdir to prevent path traversal vulnerability (#895)
- Navigate initialization phase before tools/list request in MCP client (#904)
- Use "thinking" instead of "codex reasoning" as the label for reasoning events in the TUI (#905)

### 💼 Other

- Disallow expect via lints (#865)

Adds `expect()` as a denied lint. Same deal applies with `unwrap()`
where we now need to put `#[expect(...` on ones that we legit want. Took
care to enable `expect()` in test contexts.

# Tests

```
cargo fmt
cargo clippy --all-features --all-targets --no-deps -- -D warnings
cargo test
```
- Introduce new --native flag to Node module release process (#844)
## [codex-rs-7f24ec8cae83ae22e7cc306fea4844958370827d-1-rust-v0.0.2505101753] - 2025-05-11
### 🚀 Features

- Introduce the use of tui-markdown (#851)
- Save session transcripts when using Rust CLI (#845)
- Read `model_provider` and `model_providers` from config.toml (#853)
- Support the chat completions API in the Rust CLI (#862)
- Allow pasting newlines (#866)
- Experimental env var: CODEX_SANDBOX_NETWORK_DISABLED (#879)
- Added arceeai as a provider (#818)
- Add support for AGENTS.md in Rust CLI (#885)

### 🪲 Bug Fixes

- Add optional timeout to McpClient::send_request() (#852)
- Remove CodexBuilder and Recorder (#858)
- Creating an instance of Codex requires a Config (#859)
- Remove clap dependency from core crate (#860)
- Remove wrapping in Rust TUI that was incompatible with scrolling math (#868)
- Enable clippy on tests (#870)
- Use `continue-on-error: true` to tidy up GitHub Action (#871)
- Get responses API working again in Rust (#872)
- Make McpConnectionManager tolerant of MCPs that fail to start (#854)
- Retry on OpenAI server_error even without status code (#814)
- Migrate to AGENTS.md (#764)
- Guard against missing choices (#817)
- Flex-mode via config/flag (#813)

### 💼 Other

- Update cargo to 2024 edition (#842)

Some effects of this change:
- New formatting changes across many files. No functionality changes
should occur from that.
- Calls to `set_env` are considered unsafe, since this only happens in
tests we wrap them in `unsafe` blocks
- Update submodules version to come from the workspace (#850)

Tie the version of submodules to the workspace version.
- Workspace lints and disallow unwrap (#855)

Sets submodules to use workspace lints. Added denying unwrap as a
workspace level lint, which found a couple of cases where we could have
propagated errors. Also manually labeled ones that were fine by my eye.
- Refactor exec() into spawn_child() and consume_truncated_output() (#878)
- Adds Azure OpenAI support (#769)

## Summary

This PR introduces support for Azure OpenAI as a provider within the
Codex CLI. Users can now configure the tool to leverage their Azure
OpenAI deployments by specifying `"azure"` as the provider in
`config.json` and setting the corresponding `AZURE_OPENAI_API_KEY` and
`AZURE_OPENAI_API_VERSION` environment variables. This functionality is
added alongside the existing provider options (OpenAI, OpenRouter,
etc.).

Related to #92

**Note:** This PR is currently in **Draft** status because tests on the
`main` branch are failing. It will be marked as ready for review once
the `main` branch is stable and tests are passing.

---

## What’s Changed

-   **Configuration (`config.ts`, `providers.ts`, `README.md`):**
- Added `"azure"` to the supported `providers` list in `providers.ts`,
specifying its name, default base URL structure, and environment
variable key (`AZURE_OPENAI_API_KEY`).
- Defined the `AZURE_OPENAI_API_VERSION` environment variable in
`config.ts` with a default value (`2025-03-01-preview`).
    -   Updated `README.md` to:
        -   Include "azure" in the list of providers.
- Add a configuration section for Azure OpenAI, detailing the required
environment variables (`AZURE_OPENAI_API_KEY`,
`AZURE_OPENAI_API_VERSION`) with examples.
- **Client Instantiation (`terminal-chat.tsx`, `singlepass-cli-app.tsx`,
`agent-loop.ts`, `compact-summary.ts`, `model-utils.ts`):**
- Modified various components and utility functions where the OpenAI
client is initialized.
- Added conditional logic to check if the configured `provider` is
`"azure"`.
- If the provider is Azure, the `AzureOpenAI` client from the `openai`
package is instantiated, using the configured `baseURL`, `apiKey` (from
`AZURE_OPENAI_API_KEY`), and `apiVersion` (from
`AZURE_OPENAI_API_VERSION`).
- Otherwise, the standard `OpenAI` client is instantiated as before.
-   **Dependencies:**
- Relies on the `openai` package's built-in support for `AzureOpenAI`.
No *new* external dependencies were added specifically for this Azure
implementation beyond the `openai` package itself.

---

## How to Test

*This has been tested locally and confirmed working with Azure OpenAI.*

1.  **Configure `config.json`:**
Ensure your `~/.codex/config.json` (or project-specific config) includes
Azure and sets it as the active provider:
    ```json
    {
      "providers": {
        // ... other providers
        "azure": {
          "name": "AzureOpenAI",
"baseURL": "https://YOUR_RESOURCE_NAME.openai.azure.com", // Replace
with your Azure endpoint
          "envKey": "AZURE_OPENAI_API_KEY"
        }
      },
      "provider": "azure", // Set Azure as the active provider
      "model": "o4-mini" // Use your Azure deployment name here
      // ... other config settings
    }
    ```
2.  **Set up Environment Variables:**
    ```bash
    # Set the API Key for your Azure OpenAI resource
    export AZURE_OPENAI_API_KEY="your-azure-api-key-here"

# Set the API Version (Optional - defaults to `2025-03-01-preview` if
not set)
# Ensure this version is supported by your Azure deployment and endpoint
    export AZURE_OPENAI_API_VERSION="2025-03-01-preview"
    ```
3.  **Get the Codex CLI by building from this PR branch:**
Clone your fork, checkout this branch (`feat/azure-openai`), navigate to
`codex-cli`, and build:
    ```bash
    # cd /path/to/your/fork/codex
    git checkout feat/azure-openai # Or your branch name
    cd codex-cli
    corepack enable
    pnpm install
    pnpm build
    ```
4.  **Invoke Codex:**
Run the locally built CLI using `node` from the `codex-cli` directory:
    ```bash
    node ./dist/cli.js "Explain the purpose of this PR"
    ```
*(Alternatively, if you ran `pnpm link` after building, you can use
`codex "Explain the purpose of this PR"` from anywhere)*.
5. **Verify:** Confirm that the command executes successfully and
interacts with your configured Azure OpenAI deployment.

---

## Tests

- [x] Tested locally against an Azure OpenAI deployment using API Key
authentication. Basic commands and interactions confirmed working.

---

## Checklist

- [x] Added Azure provider details to configuration files
(`providers.ts`, `config.ts`).
- [x] Implemented conditional `AzureOpenAI` client initialization based
on provider setting.
-   [x] Ensured `apiVersion` is passed correctly to the Azure client.
-   [x] Updated `README.md` with Azure OpenAI setup instructions.
- [x] Manually tested core functionality against a live Azure OpenAI
endpoint.
- [x] Add/update automated tests for the Azure code path (pending `main`
stability).

cc @theabhinavdas @nikodem-wrona @fouad-openai @tibo-openai (adjust as
needed)

---

I have read the CLA Document and I hereby sign the CLA
- Add reasoning effort option to CLI help text (#815)

Reasoning effort was already available, but not expressed into the help
text, so it was non-discoverable.

Other issues discovered, but will fix in separate PR since they are
larger:
* #816 reasoningEffort isn't displayed in the terminal-header, making it
rather hard to see the state of configuration
* I don't think the config file setting works, as the CLI option always
"wins" and overwrites it
## [codex-rs-132146b6d4e133d014f763a0d8dabd853f3fc0c0-1-rust-v0.0.2505061740] - 2025-05-07
### 🚀 Features

- Make cwd a required field of Config so we stop assuming std::env::current_dir() in a session (#800)
- Make Codex available as a tool when running it as an MCP server (#811)
- Initial McpClient for Rust (#822)
- Update McpClient::new_stdio_client() to accept an env (#831)
- Support mcp_servers in config.toml (#829)
- Show MCP tool calls in TUI (#836)
- Drop support for `q` in the Rust TUI since we already support ctrl+d (#799)
- Show MCP tool calls in `codex exec` subcommand (#841)

### 🪲 Bug Fixes

- TUI should use cwd from Config (#808)
- Is_inside_git_repo should take the directory as a param (#809)
- Ensure apply_patch resolves relative paths against workdir or project cwd (#810)
- Increase output limits for truncating collector (#575)
- Build all crates individually as part of CI (#833)
- Make all fields of Session struct private again (#840)

### 💼 Other

- Update the config.toml documentation for the Rust CLI in codex-rs/README.md (#795)
- Use "Title case" in README.md (#798)
- Use "Title case" for ToC (#812)
- Introduce codex-common crate (#843)
## [codex-rs-5915a59c8290765d6097caf4074aae93a85380fa-1-rust-v0.0.2505021951] - 2025-05-03
### 🚀 Features

- `@mention` files in codex (#701)
- Use Landlock for sandboxing on Linux in TypeScript CLI (#763)
- Introduce mcp-types crate (#787)
- Introduce mcp-server crate (#792)
- Configurable notifications in the Rust CLI  (#793)

### 🪲 Bug Fixes

- Remove unused _writableRoots arg to exec() function (#762)
- Insufficient quota message (#758)
- Input keyboard shortcut opt+delete (#685)
- Mcp-types serialization wasn't quite working (#791)

### 💼 Other

- Make build process a single script to run (#757)
- Vite version (#766)
- Configure HTTPS agent for proxies (#775)

- Some workflows require you to route openAI API traffic through a proxy
- See
https://github.com/openai/openai-node/tree/v4?tab=readme-ov-file#configuring-an-https-agent-eg-for-proxies
for more details

---------

Co-authored-by: Thibault Sottiaux <tibo@openai.com>
Co-authored-by: Fouad Matin <fouad@openai.com>

### 🛳️ Release

- *(version)* 0.1.2504301751 (#768)
## [codex-rs-e40bc9911433bd3f942ef4604626fab5638a7a72-1-rust-v0.0.2504301327] - 2025-04-30
### 💼 Other

- Mark Rust releases as "prerelease" (#761)
## [codex-rs-72a4c38e41bc64f5a7c8c73d52f45784cb6b7137-1-rust-v0.0.2504301219] - 2025-04-30
### 🪲 Bug Fixes

- Remove codex-repl from GitHub workflows (#760)

### 💼 Other

- Script to create a Rust release (#759)
## [rust-v0.0.2504301132] - 2025-04-30
### 🪲 Bug Fixes

- Include x86_64-unknown-linux-gnu in the list of arch to build codex-linux-sandbox (#748)
- Remove expected dot after v in rust-v tag name (#742)
- Read version from package.json instead of modifying session.ts (#753)
- Remove errant eslint-disable so `pnpm run lint` passes again (#756)

### 💼 Other

- Remove the REPL crate/subcommand (#754)
- Rust release, set prerelease:false and version=0.0.2504301132 (#755)
## [rust-v.0.0.2504292236] - 2025-04-30
### 🪲 Bug Fixes

- Add another place where $dest was missing in rust-release.yml (#747)
## [rust-v.0.0.2504292006] - 2025-04-30
### 💼 Other

- Fix errors in .github/workflows/rust-release.yml and prep 0.0.2504292006 release (#745)
## [rust-v.0.0.2504291954] - 2025-04-30
### 🪲 Bug Fixes

- Primary output of the codex-cli crate is named codex, not codex-cli (#743)

### 💼 Other

- Set Cargo workspace to version 0.0.2504291954 to create a scratch release (#744)
## [rust-v.0.0.2504291926] - 2025-04-30
### 💼 Other

- Set Cargo workspace to version 0.0.2504291926 to create a scratch release (#741)
## [rust-v0.0.2504291921] - 2025-04-30
### 🚀 Features

- Add shell completion subcommand (#138)
- --config/-c flag to open global instructions in nvim (#158)
- Add command history persistence (#152)
- Shell command explanation option (#173)
- Enhance image path detection in input processing (#189)
- Add notifications for MacOS using Applescript (#160)
- Update position of cursor when navigating input history with arrow keys to the end of the text (#255)
- *(bin)* Support bun fallback runtime for codex CLI (#282)
- Add /compact (#289)
- Add Nix flake for reproducible development environments (#225)
- Add /bug report command (#312)
- Notify when a newer version is available (#333)
- Add flex mode option for cost savings (#372)
- Add user-defined safe commands configuration and approval logic #380 (#386)
- Allow switching approval modes when prompted to approve an edit/command (#400)
- Read approvalMode from config file (#298)
- Add /command autocomplete (#317)
- `/diff` command to view git diff (#426)
- Add support for `/diff` command autocomplete in TerminalChatInput (#431)
- Allow multi-line input  (#438)
- Auto-open model selector if user selects deprecated model (#427)
- Support multiple providers via Responses-Completion transformation (#247)
- Tab completions for file paths (#279)
- Add support for ZDR orgs (#481)
- Add CLI –version flag (#492)
- Show actionable errors when api keys are missing (#523)
- Add openai model info configuration (#551)
- Create parent directories when creating new files. (#552)
- Added provider to run quiet mode function (#571)
- Add support for custom provider configuration in the user config (#537)
- Add specific instructions for creating API keys in error msg (#581)
- More loosely match context for apply_patch (#610)
- Enhance toCodePoints to prevent potential unicode 14 errors (#615)
- Update README and config to support custom providers with API k… (#577)
- Initial import of Rust implementation of Codex CLI in codex-rs/ (#629)
- Display error on selection of invalid model (#594)
- *(bug-report)* Print bug report URL in terminal instead of opening browser (#510) (#528)
- Introduce codex_execpolicy crate for defining "safe" commands (#634)
- Add support for OpenAI-Organization and OpenAI-Project headers (#626)
- More native keyboard navigation in multiline editor (#655)
- *(tui-rs)* Add support for mousewheel scrolling (#641)
- Add ZDR support to Rust implementation (#642)
- User config api key (#569)
- Load defaults into Config and introduce ConfigOverrides (#677)
- Make it possible to set `disable_response_storage = true` in config.toml (#714)
- Add `debug landlock` subcommand comparable to `debug seatbelt` (#715)
- Lower default retry wait time and increase number of tries (#720)
- Add `--reasoning` CLI flag (#314)
- Improve output of exec subcommand (#719)
- Add common package registries domains to allowed-domains list (#414)
- Bring back -s option to specify sandbox permissions (#739)
- Codex-linux-sandbox standalone executable (#740)

### 🪲 Bug Fixes

- Update package-lock.json name to codex (#4)
- Prompt typo (#81)
- Miss catching `auto-edit` (#99)
- Silence deprecation warnings without NODE_OPTIONS (#80)
- *(text-buffer)* Correct word deletion logic for trailing spaces (Ctrl+Backspace)    (#131)
- Correct typos in thinking texts (transcendent & parroting) (#108)
- Add missing "as" in prompt prefix in agent loop (#186)
- Allow continuing after interrupting assistant (#178)
- Typos in prompts and comments  (#195)
- Check workdir before spawn (#221)
- *(security)* Shell commands auto-executing in 'suggest' mode without permission (#197)
- Improve Windows compatibility for CLI commands and sandbox (#261)
- Update regex to better match the retry error messages (#266)
- Npm run format:fix in root (#268)
- Duplicated message on model change (#276)
- Add empty vite config file to prevent resolving to parent (#273)
- Small update to bug report template (#288)
- Standardize filename to kebab-case 🐍➡️🥙 (#302)
- *(docs)* Fix the <details> misplace in README.md (#294)
- Handle invalid commands (#304)
- Raw-exec-process-group.test improve reliability and error handling (#280)
- Canonicalize the writeable paths used in seatbelt policy (#275)
- Update context left display logic in TerminalChatInput component (#307)
- Improper spawn of sh on Windows Powershell (#318)
- Include pnpm lock file (#377)
- /bug report command, thinking indicator (#381)
- Enable shell option for child process execution (#391)
- Configure husky and lint-staged for pnpm monorepo (#384)
- Name of the file not matching the name of the component (#354)
- `full-auto` support in quiet mode (#374)
- *(cli)* Ensure /clear resets context and exclude system messages from approximateTokenUsed count (#443)
- Remove unnecessary isLoggingEnabled() checks (#420)
- *(raw-exec-process-group)* Improve test reliability (#434)
- Command pipe execution by improving shell detection (#437)
- Allow proper exit from new Switch approval mode dialog (#453)
- Auto-open model-selector when model is not found (#448)
- Remove extraneous type casts (#462)
- `/clear` now clears terminal screen and resets context left indicator (#425)
- Correct fish completion function name in CLI script (#485)
- Unintended tear down of agent loop (#483)
- *(agent-loop)* Update required properties to include workdir and ti… (#530)
- Inconsistent usage of base URL and API key (#507)
- Remove requirement for api key for ollama (#546)
- Support [provider]_BASE_URL (#542)
- Agent loop for disable response storage (#543)
- Fix typo in prompt (#558)
- Remove unreachable "disableResponseStorage" logic flow introduced in #543 (#573)
- Don't clear turn input before retries (#611)
- Lint-staged error (#617)
- Update bug report template - there is no --revision flag (#614)
- *(agent-loop)* Notify type (#608)
- `apply_patch` unicode characters (#625)
- Do not grant "node" user sudo access when using run_in_container.sh (#627)
- Update scripts/build_container.sh to use pnpm instead of npm (#631)
- *(utils)* Save config (#578)
- Only run rust-ci.yml on PRs that modify files in codex-rs (#637)
- Add RUST_BACKTRACE=full when running `cargo test` in CI (#638)
- Close stdin when running an exec tool call (#636)
- Nits in apply patch (#640)
- Model selection (#643)
- Only allow going up in history when not already in history if input is empty (#654)
- Flipped the sense of Prompt.store in #642 (#663)
- Remove dependency on expanduser crate (#667)
- Small fixes so Codex compiles on Windows (#673)
- Handling weird unicode characters in `apply_patch` (#674)
- Remove outdated copy of text input and external editor feature (#670)
- Input keyboard shortcuts (#676)
- Write logs to ~/.codex/log instead of /tmp (#669)
- Duplicate messages in quiet mode (#680)
- `/diff` should include untracked files (#686)
- Check if sandbox-exec is available (#696)
- Only allow running without sandbox if explicitly marked in safe container (#699)
- Drop d as keyboard shortcut for scrolling in the TUI (#704)
- Increase timeout of test_writable_root (#713)
- Tighten up check for /usr/bin/sandbox-exec (#710)
- Make the TUI the default/"interactive" CLI in Rust (#711)
- Eliminate runtime dependency on patch(1) for apply_patch (#718)
- Overhaul SandboxPolicy and config loading in Rust (#732)

### 💼 Other

- Initial commit

Signed-off-by: Ilan Bigio <ilan@openai.com>
- Initial commit

Signed-off-by: Ilan Bigio <ilan@openai.com>
- Add link to cookbook (#2)
- Move all tests under tests/ (#3)
- W (#8)
- Remove triaging labels section + avoid capturing exitOnCtrlC for full-context mode

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- Update summary to auto (#1)
- Update README.md to include correct default (#12)
- *(readme)* Fix typos (#27)
- (cleanup) remove unused express dep (#20)

* remove unused express dep
* update package-lock.json
- Update model in code to o4-mini (#39)

Old docs had o3 as the default
- (fix) o3 instead of o3-mini  (#37)

* o3 instead of o3-mini
- [readme] Add recipe for code review (#40)

One of my favorite use cases is a read-only one; have `codex` suggest areas of the codebase that need attention. From here, it's also easy for the user to select one of the proposed tasks and have `codex` make the PR.
- (feat) add request error details (#31)

Signed-off-by: Adam Montgomery <montgomery.adam@gmail.com>
- Add explanation on how to add OPENAI_API_KEY to docs (#28)

Signed-off-by: Hugo Biais <hugobiais75@gmail.com>
- Remove rg requirement (#50)

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- Update README.md (#46)

Fix a typo where a p should have been a q
- (fix) update Docker container scripts (#47)

* Fix Docker container scripts

Signed-off-by:: Eric Burke <eburke@openai.com>

* Build codex TGZ

* fix run_in_container

---------

Co-authored-by: Kyle Kosic <kylekosic@openai.com>
- [readme] security review recipe (#72)

* [readme] security review recipe
- (feat) gracefully handle invalid commands (#79)

* handle invalid commands
* better test
* format
- Changing some readme text to make it more exact (#77)

Making README more factually and grammatically exact.
- Release script (#96)
- Add FAQ clarifying relation to OpenAI Codex (2021) (#91)
- (feat) basic retries when hitting rate limit errors (#105)

* w

Signed-off-by: Thibault Sottiaux <tibo@openai.com>

* w

Signed-off-by: Thibault Sottiaux <tibo@openai.com>

* w

Signed-off-by: Thibault Sottiaux <tibo@openai.com>

* w

Signed-off-by: Thibault Sottiaux <tibo@openai.com>

* w

Signed-off-by: Thibault Sottiaux <tibo@openai.com>

---------

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- Release (#109)
- Back out @lib indirection in tsconfig.json (#111)
- Set up CLA process and remove DCO (#129)

Signed-off-by: Ilan Bigio <ilan@openai.com>
- Dotenv support (#122)

Signed-off-by: Aron Jones <aron.jones@gmail.com>
- Change bash to shell in README (#132)
- Removes computeAutoApproval() and tightens up canAutoApprove() as the source of truth (#126)

Previously, `parseToolCall()` was using `computeAutoApproval()`, which
was a somewhat parallel implementation of `canAutoApprove()` in order to
get `SafeCommandReason` metadata for presenting information to the user.
The only function that was using `SafeCommandReason` was
`useMessageGrouping()`, but it turns out that function was unused, so
this PR removes `computeAutoApproval()` and all code related to it.

More importantly, I believe this fixes
https://github.com/openai/codex/issues/87 because
`computeAutoApproval()` was calling `parse()` from `shell-quote` without
wrapping it in a try-catch. This PR updates `canAutoApprove()` to use a
tighter try-catch block that is specific to `parse()` and returns an
appropriate `SafetyAssessment` in the event of an error, based on the
`ApprovalPolicy`.

Signed-off-by: Michael Bolin <mbolin@openai.com>
- Fixed gramatical errors in prompt examples (#136)
- Improve performance of renderFilesToXml using Array.join (#127)
- Adds link to prompting guide in README (#141)
- Fix markdown table in README (#144)

Before:

<img width="909" alt="image"
src="https://github.com/user-attachments/assets/8de0d798-5587-407a-9196-bda3e26c1331"
/>

After:

<img width="945" alt="image"
src="https://github.com/user-attachments/assets/90454694-2f26-49b1-8c4b-e017237797ca"
/>
- Minor change in description of Build from source in README.md (#149)

Separated the `node ./dist/cli.js --help ` and `node ./dist/cli.js `.
The comment suggested `node ./dist/cli.js --help ` was to run the
locally-built CLI but in fact it shows the usage an options. It is a
minor change and clarifies the flow for new developers.
- (feat) expontential back-off when encountering rate limit errors (#153)

...and try to parse the suggested time from the error message while we
don't yet have this in a structured way

---------

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- W

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- Fmt (#161)

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- Document Codex open source fund (#162)

I added a section about the Codex Open Source Fund to the README to
reach more developers.
- (fix) do not transitively rely on deprecated lodash deps (#175)

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- (fix) move funding section before contrib section (#184)

Signed-off-by: Thibault Sottiaux <tibo@openai.com>
- Update task.yaml (#237)
- Add ZDR org limitation to README (#234)
- Reduce docker image size (#194)
- Feat/add husky (#223)

# Add Husky and lint-staged for automated code quality checks

## Description
This PR adds Husky Git hooks and lint-staged to automate code quality
checks during the development workflow.

## Features Added
- Pre-commit hook that runs lint-staged to check files before committing
- Pre-push hook that runs tests and type checking before pushing
- Configuration for lint-staged to format and lint different file types
- Documentation explaining the Husky setup and usage
- Updated README.md with information about Git hooks

## Benefits
- Ensures consistent code style across the project
- Prevents pushing code with failing tests or type errors
- Reduces the need for style-related code review comments
- Improves overall code quality

## Implementation Details
- Added Husky and lint-staged as dev dependencies
- Created pre-commit and pre-push hooks
- Added configuration for lint-staged
- Added documentation in HUSKY.md
- Updated README.md with a new section on Git hooks

## Testing
The hooks have been tested locally and work as expected:
- Pre-commit hook runs ESLint and Prettier on staged files
- Pre-push hook runs tests and type checking

I have read the CLA Document and I hereby sign the CLA

---------

Signed-off-by: Alpha Diop <alphakhoss@gmail.com>
- Additional error handling logic for model errors that occur in stream (#203)
- Remove redundant thinking updates and put a thinking timer above the prompt instead (#216)
- Clarify sandboxing situation on Linux (#103)
- Add tracing instructions to README (#257)
- Git ignore unwanted package managers (#214)
- Mention dotenv support in Quickstart (#262)
- Add support for -w,--writable-root to add more writable roots for sandbox (#263)

This adds support for a new flag, `-w,--writable-root`, that can be
specified multiple times to _amend_ the list of folders that should be
configured as "writable roots" by the sandbox used in `full-auto` mode.
Values that are passed as relative paths will be resolved to absolute
paths.

Incidentally, this required updating a number of the `agent*.test.ts`
files: it feels like some of the setup logic across those tests could be
consolidated.

In my testing, it seems that this might be slightly out of distribution
for the model, as I had to explicitly tell it to run `apply_patch` and
that it had the permissions to write those files (initially, it just
showed me a diff and told me to apply it myself). Nevertheless, I think
this is a good starting point.
- Consolidate patch prefix constants in apply‑patch.ts (#274)
- *(.github)* Issue templates (#283)
- Suggest mode file read behavior openai/codex#197 (#285)
- Changelog (#308)
- Bug report form (#313)
- Fix handling of Shift+Enter in e.g. Ghostty (#338)

Fix: Shift + Enter no longer prints “[27;2;13~” in the single‑line
input. Validated as working and necessary in Ghostty on Linux.

## Key points
- src/components/vendor/ink-text-input.tsx
- Added early handler that recognises the two modifyOtherKeys
escape‑sequences
    - [13;<mod>u  (mode 2 / CSI‑u)
    - [27;<mod>;13~ (mode 1 / legacy CSI‑~)
- If Ctrl is held (hasCtrl flag) → call onSubmit() (same as plain
Enter).
- Otherwise → insert a real newline at the caret (same as Option+Enter).
  - Prevents the raw sequence from being inserted into the buffer.

- src/components/chat/multiline-editor.tsx
- Replaced non‑breaking spaces with normal spaces to satisfy eslint
no‑irregular‑whitespace rule (no behaviour change).

All unit tests (114) and ESLint now pass:
npm test ✔️
npm run lint ✔️
- Revert "fix: canonicalize the writeable paths used in seatbelt policy… (#370)

This reverts commit 3356ac0aefac43d45973b994dcabfb8125779cd7.

related #330
- Migrate to pnpm for improved monorepo management (#287)
- Fix #371 Allow multiple containers on same machine (#373)

- Docker container name based on work  directory
- Centralize container removal logic
- Improve quoting for command arguments
- Ensure workdir is always set and normalized

Resolves: #371 

Signed-off-by: BadPirate <badpirate@gmail.com>

Signed-off-by: BadPirate <badpirate@gmail.com>
- Change file name to start with small letter instead of captial l… (#356)
- Update lockfile (#379)
- Update README to use pnpm commands (#390)
- Add fallback text for missing images (#397)

# What?
* When a prompt references an image path that doesn’t exist, replace it
with
  ```[missing image: <path>]``` instead of throwing an ENOENT.
* Adds a few unit tests for input-utils as there weren't any beforehand.

# Why?
Right now if you enter an invalid image path (e.g. it doesn't exist),
codex immediately crashes with a ENOENT error like so:
```
Error: ENOENT: no such file or directory, open 'test.png'
   ...
 {
  errno: -2,
  code: 'ENOENT',
  syscall: 'open',
  path: 'test.png'
}
```
This aborts the entire session. A soft fallback lets the rest of the
input continue.

# How?
Wraps the image encoding + inputItem content pushing in a try-catch. 

This is a minimal patch to avoid completely crashing — future work could
surface a warning to the user when this happens, or something to that
effect.

---------

Co-authored-by: Thibault Sottiaux <tibo@openai.com>
- Gracefully handle SSE parse errors and suppress raw parser code (#367)

Closes #187
Closes #358

---------

Co-authored-by: Thibault Sottiaux <tibo@openai.com>
- Re-enable Prettier check for codex-cli in CI (#417)

This check was lost in https://github.com/openai/codex/pull/287. Both
the root folder and `codex-cli/` have their own `pnpm format` commands
that check the formatting of different things.

Also ran `pnpm format:fix` to fix the formatting violations that got in
while this was disabled in CI.

---
[//]: # (BEGIN SAPLING FOOTER)
Stack created with [Sapling](https://sapling-scm.com). Best reviewed
with [ReviewStack](https://reviewstack.dev/openai/codex/pull/417).
* #420
* #419
* #416
* __->__ #417
- Use spawn instead of exec to avoid injection vulnerability (#416)

https://github.com/openai/codex/pull/160 introduced a call to `exec()`
that takes a format string as an argument, but it is not clear that the
expansions within the format string are escaped safely. As written, it
is possible a carefully crafted command (e.g., if `cwd` were `"; && rm
-rf` or something...) could run arbitrary code.

Moving to `spawn()` makes this a bit better, as now at least `spawn()`
itself won't run an arbitrary process, though I suppose `osascript`
itself still could if the value passed to `-e` were abused. I'm not
clear on the escaping rules for AppleScript to ensure that `safePreview`
and `cwd` are injected safely.

---
[//]: # (BEGIN SAPLING FOOTER)
Stack created with [Sapling](https://sapling-scm.com). Best reviewed
with [ReviewStack](https://reviewstack.dev/openai/codex/pull/416).
* #423
* #420
* #419
* __->__ #416
- Make it so CONFIG_DIR is not in the list of writable roots by default (#419)

To play it safe, let's keep `CONFIG_DIR` out of the default list of
writable roots.

This also fixes an issue where `execWithSeatbelt()` was modifying
`writableRoots` instead of creating a new array.

---
[//]: # (BEGIN SAPLING FOOTER)
Stack created with [Sapling](https://sapling-scm.com). Best reviewed
with [ReviewStack](https://reviewstack.dev/openai/codex/pull/419).
* #423
* #420
* __->__ #419
- *(component)* Rename component to match its filename (#432)
- *(history-overlay)* Split into modular functions & add tests (fixes #402) (#403)
- Remove `README.md` and `bin` from `package.json#files` field (#461)

This PR removes always included files and folders from the
[`package.json#files`
field](https://docs.npmjs.com/cli/v11/configuring-npm/package-json#files):

> Certain files are always included, regardless of settings:
> - package.json
> - README
> - LICENSE / LICENCE
> - The file in the "main" field
> - The file(s) in the "bin" field

Validated by running `pnpm i && cd codex-cli && pnpm build && pnpm
release:readme && pnpm pack` and confirming both the `README.md` file
and `bin` directory are still included in the tarball:

<img width="227" alt="image"
src="https://github.com/user-attachments/assets/ecd90a07-73c7-4940-8c83-cb1d51dfcf96"
/>
- *(updates)* Fetch version from registry instead of npm CLI to support multiple managers (#446)
- Improve storage/ implementation; use log(...) consistently (#473)
- Consolidate model utils and drive-by cleanups (#476)
- *(build)* Cleanup dist before build (#477)
- Revert #386 due to unsafe shell command parsing (#478)

Reverts https://github.com/openai/codex/pull/386 because:

* The parsing logic for shell commands was unsafe (`split(/\s+/)`
instead of something like `shell-quote`)
* We have a different plan for supporting auto-approved commands.
- Drop src from publish (#474)
- Do not auto-approve the find command if it contains options that write files or spawn commands (#482)

Updates `isSafeCommand()` so that an invocation of `find` is not
auto-approved if it contains any of: `-exec`, `-execdir`, `-ok`,
`-okdir`, `-delete`, `-fls`, `-fprint`, `-fprint0`, `-fprintf`.
- Add note about non-openai providers; add --provider cli flag to the help (#484)
- Readme (#491)
- Include fractional portion of chunk that exceeds stdout/stderr limit (#497)

I saw cases where the first chunk of output from `ls -R` could be large
enough to exceed `MAX_OUTPUT_BYTES` or `MAX_OUTPUT_LINES`, in which case
the loop would exit early in `createTruncatingCollector()` such that
nothing was appended to the `chunks` array. As a result, the reported
`stdout` of `ls -R` would be empty.

I asked Codex to add logic to handle this edge case and write a unit
test. I used this as my test:

```
./codex-cli/dist/cli.js -q 'what is the output of `ls -R`'
```

now it appears to include a ton of stuff whereas before this change, I
saw:

```
{"type":"function_call_output","call_id":"call_a2QhVt7HRJYKjb3dIc8w1aBB","output":"{\"output\":\"\\n\\n[Output truncated: too many lines or bytes]\",\"metadata\":{\"exit_code\":0,\"duration_seconds\":0.5}}"}
```
- Enforce ASCII in README.md (#513)

This all started because I was going to write a script to autogenerate
the Table of Contents in the root `README.md`, but I noticed that the
`href` for the "Why Codex?" heading was `#whycodex` instead of
`#why-codex`. This piqued my curiosity and it turned out that the space
in "Why Codex?" was not an ASCII space but **U+00A0**, a non-breaking
space, and so GitHub ignored it when generating the `href` for the
heading.

This also meant that when I did a text search for `why codex` in the
`README.md` in VS Code, the "Why Codex" heading did not match because of
the presence of **U+00A0**.

In short, these types of Unicode characters seem like a hazard, so I
decided to introduce this script to flag them, and if desired, to
replace them with "good enough" ASCII equivalents. For now, this only
applies to the root `README.md` file, but I think we should ultimately
apply this across our source code, as well, as we seem to have quite a
lot of non-ASCII Unicode and it's probably going to cause `rg` to miss
things.

Contributions of this PR:

* `./scripts/asciicheck.py`, which takes a list of filepaths and returns
non-zero if any of them contain non-ASCII characters. (Currently, there
is one exception for ✨ aka **U+2728**, though I would like to default to
an empty allowlist and then require all exceptions to be specified as
flags.)
* A `--fix` option that will attempt to rewrite files with violations
using a equivalents from a hardcoded substitution list.
* An update to `ci.yml` to verify `./scripts/asciicheck.py README.md`
succeeds.
* A cleanup of `README.md` using the `--fix` option as well as some
editorial decisions on my part.
* I tried to update the `href`s in the Table of Contents to reflect the
changes in the heading titles. (TIL that if a heading has a character
like `&` surrounded by spaces, it becomes `--` in the generated `href`.)
- Minimal mid-stream #429 retry loop using existing back-off (#506)
- Add check to ensure ToC in README.md matches headings in the file (#541)

This introduces a Python script (written by Codex!) to verify that the
table of contents in the root `README.md` matches the headings. Like
`scripts/asciicheck.py` in https://github.com/openai/codex/pull/513, it
reports differences by default (and exits non-zero if there are any) and
also has a `--fix` option to synchronize the ToC with the headings.

This will be enforced by CI and the changes to `README.md` in this PR
were generated by the script, so you can see that our ToC was missing
some entries prior to this PR.
- Add instructions for connecting to a visual debugger under Contributing (#496)

While here, I also moved the Nix stuff to the end of the
**Contributing** section and replaced some examples with `npm` to use
`pnpm`.
- When a shell tool call invokes apply_patch, resolve relative paths against workdir, if specified (#556)

Previously, we were ignoring the `workdir` field in an `ExecInput` when
running it through `canAutoApprove()`. For ordinary `exec()` calls, that
was sufficient, but for `apply_patch`, we need the `workdir` to resolve
relative paths in the `apply_patch` argument so that we can check them
in `isPathConstrainedTowritablePaths()`.

Likewise, we also need the workdir when running `execApplyPatch()`
because the paths need to be resolved again.

Ideally, the `ApplyPatchCommand` returned by `canAutoApprove()` would
not be a simple `patch: string`, but the parsed patch with all of the
paths resolved, in which case `execApplyPatch()` could expect absolute
paths and would not need `workdir`.
- Non-openai mode - fix for gemini content: null, fix 429 to throw before stream (#563)
- Non-openai mode - don't default temp and top_p (#572)
- Fix error catching when checking for updates (#597)
- Update lint-staged config to use pnpm --filter (#582)
- Readme (#630)
- [codex-rs] More fine-grained sandbox flag support on Linux (#632)

##### What/Why
This PR makes it so that in Linux we actually respect the different
types of `--sandbox` flag, such that users can apply network and
filesystem restrictions in combination (currently the only supported
behavior), or just pick one or the other.

We should add similar support for OSX in a future PR.

##### Testing
From Linux devbox, updated tests to use more specific flags:
```
test linux::tests_linux::sandbox_blocks_ping ... ok
test linux::tests_linux::sandbox_blocks_getent ... ok
test linux::tests_linux::test_root_read ... ok
test linux::tests_linux::test_dev_null_write ... ok
test linux::tests_linux::sandbox_blocks_dev_tcp_redirection ... ok
test linux::tests_linux::sandbox_blocks_ssh ... ok
test linux::tests_linux::test_writable_root ... ok
test linux::tests_linux::sandbox_blocks_curl ... ok
test linux::tests_linux::sandbox_blocks_wget ... ok
test linux::tests_linux::sandbox_blocks_nc ... ok
test linux::tests_linux::test_root_write - should panic ... ok
```

##### Todo
- [ ] Add negative tests (e.g. confirm you can hit the network if you
configure filesystem only restrictions)
- Upgrade prettier to v3 (#644)
- Provider config (#653)
- Optimize token streaming with balanced approach (#635)
- [codex-rs] Reliability pass on networking (#658)

We currently see a behavior that looks like this:
```
2025-04-25T16:52:24.552789Z  WARN codex_core::codex: stream disconnected - retrying turn (1/10 in 232ms)...
codex> event: BackgroundEvent { message: "stream error: stream disconnected before completion: Transport error: error decoding response body; retrying 1/10 in 232ms…" }
2025-04-25T16:52:54.789885Z  WARN codex_core::codex: stream disconnected - retrying turn (2/10 in 418ms)...
codex> event: BackgroundEvent { message: "stream error: stream disconnected before completion: Transport error: error decoding response body; retrying 2/10 in 418ms…" }
```

This PR contains a few different fixes that attempt to resolve/improve
this:
1. **Remove overall client timeout.** I think
[this](https://github.com/openai/codex/pull/658/files#diff-c39945d3c42f29b506ff54b7fa2be0795b06d7ad97f1bf33956f60e3c6f19c19L173)
is perhaps the big fix -- it looks to me like this was actually timing
out even if events were still coming through, and that was causing a
disconnect right in the middle of a healthy stream.
2. **Cap response sizes.** We were frequently sending MUCH larger
responses than the upstream typescript `codex`, and that was definitely
not helping. [Fix
here](https://github.com/openai/codex/pull/658/files#diff-d792bef59aa3ee8cb0cbad8b176dbfefe451c227ac89919da7c3e536a9d6cdc0R21-R26)
for that one.
3. **Much higher idle timeout.** Our idle timeout value was much lower
than typescript.
4. **Sub-linear backoff.** We were much too aggressively backing off,
[this](https://github.com/openai/codex/pull/658/files#diff-5d5959b95c6239e6188516da5c6b7eb78154cd9cfedfb9f753d30a7b6d6b8b06R30-R33)
makes it sub-exponential but maintains the jitter and such.

I was seeing that `stream error: stream disconnected` behavior
constantly, and anecdotally I can no longer reproduce. It feels much
snappier.
- [codex-rs] CI performance for rust (#639)

* Refactors the rust-ci into a matrix build
* Adds directory caching for the build artifacts
* Adds workflow dispatch for manual testing
- [codex-rs] Improve linux sandbox timeouts (#662)

* Fixes flaking rust unit test
* Adds explicit sandbox exec timeout handling
- Build Rust on Windows as part of CI (#665)
- [codex-rs] fix: exit code 1 if no api key (#697)
- Fixes issue #726 by adding config to configToSave object (#728)

The saveConfig() function only includes a hardcoded subset of properties
when writing the config file. Any property not explicitly listed (like
disableResponseStorage) will be dropped.
I have added `disableResponseStorage` to the `configToSave` object as
the immediate fix.

[Linking Issue this fixes.](https://github.com/openai/codex/issues/726)
- [codex-rs] Add rust-release action (#671)

Taking a pass at building artifacts per platform so we can consider
different distribution strategies that don't require users to install
the full `cargo` toolchain.

Right now this grabs just the `codex-repl` and `codex-tui` bins for 5
different targets and bundles them into a draft release. I think a
clearly marked pre-release set of artifacts will unblock the next step
of testing.

### 🛳️ Release

- *(version)* 0.1.04161352 (#125)
- *(version)* 0.1.2504161510 (#135)
- *(version)* 0.1.2504161551 (#254)
- *(version)* 0.1.2504172351 (#310)
- *(version)* 0.1.2504181820 (#385)
- *(version)* 0.1.2504211509 (#493)
- *(version)* 0.1.2504220136 (#518)
- *(version)* 0.1.2504221401 (#559)
- *(version)* 0.1.2504251709 (#660)
<!-- generated - do not edit -->
