# PDD Kit correct usage (Proof-Driven Development, GitHub Next + Microsoft Research)

Scope is the GitHub Next / Microsoft Research **Proof-Driven Development Kit**, as of 2026-11-07. It is not the unrelated npm project `pdd-toolkit` / `itai-gendler/pdd-kit` (Product-Driven Development). That second project is recorded only as a name collision that would make a first trial install the wrong tool.

There is no documented public install, hello-world command, release tag, or license for the Proof-Driven Development Kit itself. The operational path below is the loop the authors describe inside Chopin. Commands quoted from Chopin’s README are **host** commands. Following them does not install PDD Kit.

## Repository URL, default branch, latest release/tag, license, and maintenance status

### Takeaway
As of 2026-11-07 there is no public GitHub repository, default branch, release tag, or stated license for the Proof-Driven Development Kit. The canonical description is the GitHub Next project page (Research Prototype, published November 2026). The kit is described as running inside Chopin, whose public repo is actively maintained on `main` under MIT, but that repo’s README does not ship PDD Kit.

### Cited Findings
- Project page: stage **Research Prototype**, published **Nov 2026**, “Building trust in the code that your agents wrote.” Authors listed: Alex Lavaee, Matthai Philipose, Maggie Appleton, Krzysztof Cieślak. “PDD Kit is a collaboration between GitHub Next and Microsoft Research.” It “runs inside Chopin as a planning partner.” No source-repository URL, license, release, or install link is on that page. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Microsoft Research blog, published 2026-11-05: “PDD Kit is our toolkit” for Proof-Driven Development. It “runs as a partner inside Chopin.” Acknowledgments name Microsoft Research Catalyst Lab, GitHub Next, and the Research in Software Engineering (RiSE) group. No repository URL or license. — [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Announcement post, 2026-10-21: “Now in research preview.” “PDD Kit is integrated with @GitHubNext’s Chopin.” “It works on Rust today.” Live demo promised at GitHub Universe, Oct 28–29, 2026. No clone URL. — [GitHub Next on X, 2026-10-21](https://x.com/GitHubNext/status/2112936525457740139); [GitHub on X, 2026-10-21](https://x.com/github/status/2112938664959562081)
- Follow-up, 2026-11-04, points at the write-up (short link `https://t.co/EwhqTx2OjT`) and still does not link a repository. — [GitHub Next on X, 2026-11-04](https://x.com/GitHubNext/status/2118105391707967967)
- GitHub repository search `pdd-kit` restricted to orgs `githubnext`, `microsoft`, and `microsoftresearch` returned `total_count: 0` on 2026-11-07. — [GitHub search API](https://api.github.com/search/repositories?q=pdd-kit+org:githubnext+OR+org:microsoft+OR+org:microsoftresearch)
- A broader name/description search for `"proof-driven development"` or `pdd-kit` does not return a githubnext or Microsoft repository. The hit named `pdd-kit` is `itai-gendler/pdd-kit`, described as “Product-Driven Development for coding agents,” homepage `https://pdd-kit.dev`, not the Verus kit. Other `proof-driven-development` repos (AlexTavor, pb3975, HomenShum, FinalTheory) are independent and are not the GitHub Next kit. — [GitHub search API](https://api.github.com/search/repositories?q=%22proof-driven+development%22+OR+pdd-kit+in:name,description&per_page=15)
- `githubnext/pdd-demo`, named in a Chopin pull request as a real-GitHub QA target, returned HTTP 404 on 2026-11-07 (private, renamed, or removed). User `lavaman131` has no public repositories matching `pdd`. — [githubnext/pdd-demo API](https://api.github.com/repos/githubnext/pdd-demo); [lavaman131 repo search](https://api.github.com/search/repositories?q=pdd+user:lavaman131&per_page=10)
- Public host repo: `https://github.com/githubnext/chopin`. `default_branch` is `main`. License SPDX `MIT`. Not archived. `pushed_at` 2026-11-06T07:26:57Z, `updated_at` 2026-11-07T06:10:46Z. `open_issues_count` 18. Language TypeScript. Homepage `https://githubnext.com/projects/chopin/`. Created 2026-07-27. — [GitHub API, githubnext/chopin](https://api.github.com/repos/githubnext/chopin)
- Chopin README: “experimental research software, not a supported GitHub product or a production-ready service.” “We do not provide public Chopin instance for general use.” The README never names PDD Kit, Verus, `pdd-plan`, `pdd-prove`, or `pdd-ship`. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Chopin pull requests by Alex Lavaee (PDD Kit co-author) treat `pdd-kit` as an **operator extension** of the Atomic harness, not as a repository a user clones. PR 445 (merged 2026-10-10): “Planner stages now load operator extensions such as pdd-kit and run in the document’s checkout,” after upgrading Atomic to 0.9.32. PR 422 and PR 439 repeat the preference “Keep pdd-kit and Chopin on the same Atomic release.” PR 440 (merged 2026-10-09): “Planner extensions such as pdd-kit can now reach image models through codemode.” PR 441 (merged 2026-10-09) mentions “a picture drawn for a plan (for example by pdd-kit’s `plan_image`).” None of these PRs publish the extension’s source tree or an install command for it. — [chopin#445](https://github.com/githubnext/chopin/pull/445); [chopin#422](https://github.com/githubnext/chopin/pull/422); [chopin#439](https://github.com/githubnext/chopin/pull/439); [chopin#440](https://github.com/githubnext/chopin/pull/440); [chopin#441](https://github.com/githubnext/chopin/pull/441)
- Name collision, do not use for this trial: `itai-gendler/pdd-kit` is MIT, default branch `main`, `pushed_at` 2026-11-06T23:20:48Z, homepage `https://pdd-kit.dev`, description “Product-Driven Development for coding agents.” A commit on that repo is titled “chore: release 0.37.0” (seen on the commits page around 2026-10-14). That release is a Product Model / agent-skills tool, not Verus. — [itai-gendler/pdd-kit](https://github.com/itai-gendler/pdd-kit); [commits](https://github.com/itai-gendler/pdd-kit/commits/main); [introducing pdd-kit, 2026-10-07](https://pdd-kit.dev/blog/introducing-pdd-kit)

### Inferences
- Maintenance of the proof kit is active as a research collaboration through at least 2026-11-05 (MSR blog) and the host repo was pushed 2026-11-06, but “active host repo” is not evidence that a public PDD Kit package exists.
- The extension mentions in October 2026 Chopin PRs are the strongest public evidence that PDD Kit is an operator-loaded Atomic extension used by the authors, not a standalone CLI. The extension is not part of the documented local-run path.

### Gaps
- No license text for the Proof-Driven Development Kit was found. Chopin’s MIT license must not be copied onto the kit.
- No latest tag or release notes for the kit. Chopin’s `releases/latest` is only referenced by the Chopin CLI installer; the tag name was not retrieved.
- Whether a private `githubnext/pdd-kit` repository exists was not confirmable. The public API search returned zero and `githubnext/pdd-demo` is 404.
- No public issue tracker belongs to the kit. Chopin’s 18 open issues were not individually triaged for PDD behavior.

## Prerequisites: runtimes, provers, package managers, API keys, model providers

### Takeaway
The kit’s own pages require Rust code and the Verus verifier, with Z3 as Verus’s solver. They do not publish Verus, Rust, or Z3 version pins, and they do not tell the user to install Lean, Dafny, or Isabelle. Model access is through Chopin. Chopin’s documented local prerequisites are Bun 1.4.4, Docker Compose, a GitHub App, and a Copilot entitlement for the default harness. Those are necessary to run public Chopin and are not sufficient to run PDD Kit.

### Cited Findings
- Verifier today is Verus. “You write ordinary Rust plus `requires` and `ensures` clauses, and Verus checks at compile time that the code meets them for all inputs.” Other tools named as things “you may have heard of,” not as dependencies of this kit: Lean, Dafny (prove code meets a spec), TLA+ (checks a design or protocol, not the code). — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- “The proof is Rust code annotated with Verus constructs, checked mechanically by Verus using the Z3 solver.” “The check is deterministic, with no model involved. Models write the spec, code, and proof, but only a deterministic Verus run decides whether it passed.” Assumed trusted computing base of the toolchain: “The Verus, Rust compiler and Z3 toolchain, and Verus’s standard library vstd.” — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- “We chose Verus because it allows us to verify Rust code directly.” “Other proof languages (e.g., Lean, F*, Rocq) take different approaches.” The techniques “could help put them to work as well.” That is a future possibility, not a current install option. — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Isabelle is not named on the project page or the MSR blog.
- In the Pet Store build-plan figure, the first task says “Verus is installed at a pinned revision with async fn support.” The revision identifier is not printed. The task also says the backend must still pass its existing tests, and “if an async construct can’t be expressed the task stops with that question instead of weakening a contract.” — [GitHub Next, PDD Kit, Build plan figure](https://githubnext.com/projects/pdd-kit/)
- No API key, model name, or provider is specified for PDD Kit itself. Models are described only as the writers of spec, code, and proof; Verus is the pass/fail judge. — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Chopin local development, documented requirements: Bun 1.4.4; Docker Engine with Docker Compose (PostgreSQL); a GitHub App owned by the deployment; a GitHub account with push or administration access to a test repository and, for the default Planner harness, an active Copilot entitlement. Callback URLs are `http://127.0.0.1:8787`, `http://127.0.0.1:8787/auth/github/callback`, and `http://127.0.0.1:8787/auth/github/setup`. Optional conversation-derived cards use `OPENAI_API_KEY` (OpenAI Decisions API) or `JEV_API_KEY` (TypeSafe Jev) and default off. Under `HARNESS=atomic`, model access comes from the operator’s `HARNESS_AUTH` instead of the user’s Copilot entitlement. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Default harness `copilot-sdk`: “it cannot write to GitHub, edit a checkout, or implement code.” `HARNESS=atomic` “deliberately runs it as a full Atomic session with shell and filesystem access as the server process’s user.” — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Machine connector agents the README says were tested: GitHub Copilot CLI (`npm install -g @github/copilot`, `--agent copilot`) and Atomic (`npm install -g @bastani/atomic @bastani/atomic-acp`, `--agent atomic`). — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- macOS is not mentioned in the PDD Kit project page, the MSR blog, or the Chopin “Run locally” section. The Chopin CLI doc link says the CLI page covers Windows; the Unix installer is `curl … install.sh | sh`. No Apple Silicon versus Intel note was found. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- The unrelated Product-Driven Development package requires Node.js 22.12 or newer and is installed with npm / `npx`. That requirement does not apply to the proof kit. — [pdd-kit.dev installation](https://pdd-kit.dev/docs/installation)

### Inferences
- A first trial that only has Node, Python, or .NET, and no Rust toolchain, cannot stay on the verified path the authors describe.
- Z3 is a transitive solver inside Verus, not a prover the user is told to script. Installing Z3, Lean, Dafny, or Isabelle by hand is not a documented PDD Kit step.
- Because the prove task “installs Verus at a pinned revision,” the intended operator of a working preview may not install Verus themselves. The pin is unpublished, so a manual Verus install would be reconstructed, not documented.
- Public Chopin’s default Copilot harness cannot implement code. The author PRs that mention the `pdd-kit` extension are about the Atomic harness. A trial that never sets `HARNESS=atomic` and never loads an operator extension will not be running the kit, even if Chopin starts.

### Gaps
- Exact Rust, Verus, Z3, and vstd versions or commit SHAs used for the Pet Store proof. Only “a pinned revision with async fn support” is stated.
- Which model provider and which API key the research-preview prove loop uses. Chopin documents Copilot, and optionally OpenAI or Jev, for Chopin. The blog does not say PDD Kit uses those same credentials.
- Whether Verus’s pinned revision builds on macOS Apple Silicon or Intel. No platform matrix was published for the kit.
- Isabelle: no statement that it is or is not supported, beyond the positive statement that the verifier today is Verus on Rust.

## Install and hello-world

### Takeaway
There is no official install command and no hello-world for the Proof-Driven Development Kit. The closest documented user action is a two-sentence request inside an already-running Chopin session against a Rust app. Public Chopin can be started from its README, but that start path does not enable PDD Kit. `npx pdd-toolkit install` is the wrong product.

### Cited Findings
- No install, clone, package name, or `cargo`/`verus` invocation is given on the project page or the MSR blog. The user-facing demo request, quoted as the whole request, is: “Verify the login logic in backend/src/handlers/auth.rs. Here are my requirements: 1. You only get a token with the right password. 2. Only admin accounts get admin tokens.” After sign-off, the user action is **Build plan**. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- The Pet Store sources are not linked as a cloneable repository. The blog links the Rust language, Axum, SQLite, Leptos, and WebAssembly project sites, not a Pet Store git URL. — [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- `githubnext/pdd-demo` is 404. — [GitHub API](https://api.github.com/repos/githubnext/pdd-demo)
- Chopin local start, from the README, is not a PDD Kit install. Exact sequence:

```
bun install
cp .env.example .env
openssl rand -hex 32
# Fill in the GitHub App values, generated session key, and OPENAI_API_KEY or
# JEV_API_KEY in .env.
bun run db:up
bun run migrate
bun run dev
```

Then open `http://127.0.0.1:8787`, sign in, install or update the GitHub App, select a repository, and create a channel. Stop with Ctrl-C. Tear down Postgres with `bun run db:down`. The README says the committed Compose override publishes PostgreSQL on host port 5432 and should be used only on a trusted machine. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Chopin machine connector, also not a PDD Kit install:

```
curl -fsSL https://github.com/githubnext/chopin/releases/latest/download/install.sh | sh
CHOPIN_URL=https://your-chopin-instance.example chopin connect --agent copilot
```

`--install-service` is documented for keeping it running after SSH logout. The README says to install and sign in to the agent first. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Chopin verification commands for developing Chopin itself, not for proving an app: `bun test`, `bun run test:postgres`, `bun run e2e`, `bun run types`, `bun run ci`. `bun run e2e:browsers` installs Chromium once. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- One concrete test command appears inside the Pet Store plan’s empirical section, marked **planned**, not as a kit smoke test: `cargo test -p backend check_password`. — [GitHub Next, PDD Kit, empirical verification figure](https://githubnext.com/projects/pdd-kit/)
- Wrong-tool install, labeled so it is not followed for this trial. Product-Driven Development (Itai Gendler), Node.js 22.12+:

```
npx pdd-toolkit install
```

or `npm install --global pdd-toolkit` then `pdd-kit install`. Skills are `/pdd-map`, `/pdd-ideate`, `/pdd-verify` (Codex uses `$` instead of `/`). That tool writes a Markdown `product-model/` and does not invoke Verus. — [pdd-kit.dev installation](https://pdd-kit.dev/docs/installation); [itai-gendler/pdd-kit README](https://github.com/itai-gendler/pdd-kit/blob/main/README.md)
- Secondary report, 2026-10-27, said the GitHub Next site then had no PDD Kit page and no application form. The project page now exists (November 2026) and still has no install or signup. Treat the “no page” claim as stale; the “no public install or application form” claim still matches the current project page. — [techmoon.xyz, 2026-10-27](https://techmoon.xyz/pdd-kit-verus-rust-proof-driven-development/); [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)

### Inferences
- A macOS first trial cannot honestly start from a PDD Kit README, Makefile, `package.json`, or CI workflow, because none were published. Reconstructing `cargo verus` or a Verus install script would be reconstructed, not documented, and is not recommended here.
- The only “hello world” the authors show is incremental verification of an existing Rust login handler, not a greenfield scaffold and not a non-Rust web app.
- Running the Chopin README on macOS is plausible (Bun, Docker, zsh, `curl | sh`) but undocumented for Apple Silicon. Even a successful Chopin boot is a negative test for PDD Kit unless the unpublished operator extension is present. The README gives no flag to turn that extension on.

### Gaps
- Research-preview admission: who can get the extension, whether there is a waitlist, and whether GitHub Universe demo access still exists after 2026-10-29. Not stated on the November pages.
- No CI workflow file for `pdd-ship` was published, so the exact Verus CI command is unknown.
- macOS-specific failures (Docker Desktop versus Docker Engine, port 5432 already taken by a local Postgres, Bun 1.4.4 on Apple Silicon, `install.sh` architecture) are not documented.

## The correct loop, in order

### Takeaway
Humans settle intent before any proved code is written. The agent may draft the Verus spec, the proof, the implementation, and the trusted-boundary tests only after that sign-off. Verification failure sends gaps back to the agent, or contradictions back to the human. The agent must not weaken a contract to get a green check. After merge, `pdd-ship` is what the authors say keeps later edits on the same spec.

### Cited Findings
- Named workflows: `pdd-plan` (questions before code, decision cards, intent artifact, trusted boundary, two reviewer approvals), then a formal Verus spec, then `pdd-prove` (Verus proof, tests for trusted parts, LLM gap review). Contradictions in the spec go back to `pdd-plan` as decisions. Gaps go back to the coding agent. `pdd-ship` adds the Verus check to the build and CI. — [GitHub Next, PDD Kit, pipeline figure](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog, Figure 1 and Step 7](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Order the authors walked for login:
  1. Human states rough intent in Chopin. The example is two sentences naming a file and two requirements. The kit is supposed to capture intent; the request is deliberately rough. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  2. `pdd-plan` reads the code and asks questions **before writing anything**, on decision cards. Cards explain the relevant code first. Decisions are retained as attributed answers, separate from prose. — [Microsoft Research blog, Step 1](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/); [Chopin README, Decisions](https://github.com/githubnext/chopin/blob/main/README.md)
  3. While the conversation continues, a rough-work section stays current: flowchart, spec landscape (what will be proved versus trusted), optional mental-model picture, and an intent map tying each requirement to the human’s words, the spec section, and the tests. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  4. A comprehension quiz can be one of the cards, so the people who own the code see what they are approving. The login quiz’s correct answer, as the page states it, is 401 and no token when email wins over a username and the password is not that customer’s. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  5. Before anything is built, the team signs off on the plan as a whole. Widening scope (proving the async handler glue, not only the decision function) is a separate decision, because it adds trusted wrappers. A later sign-off is required again: “matching your intent isn’t proof, and it doesn’t approve new trust.” — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  6. Each trusted piece is its own sign-off. For login, six pieces. Every assumed clause gets a Given/When/Then test written into the plan **before the test exists**, marked planned until it has run, then recorded with the command result and the commit. — [Microsoft Research blog, Step 4](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
  7. Two reviewer agents check that the derived spec matches the intent and does not promise more than a proof could show. Refinement iterates until those reviewers agree. That is still review. The proof comes after. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  8. The human reviews the intent artifact and the spec (English comment on every clause). “Nobody” is supposed to read the proof instead of the code, because the agent writes the proof and Verus checks every step. “Signing off on the spec is where the developer’s review ends.” — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  9. **Build plan** hands the approved plan to `pdd-prove`, which builds a task graph and runs it on a development machine. Login’s graph has seven tasks: make `verified/src/login.rs` build under Verus inside the backend crate; prove `lookup_target`, `decide_login`, and `login_core`; prove `issue_token`; prove `login_flow` end to end and rewire the login handler as its shell; test the trusted wrappers’ contracts; end-to-end login tests through the handler; signing-failure and signing-count tests. — [GitHub Next, PDD Kit, Build plan](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog, Step 6](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
  10. Each task: a coding agent writes implementation and proof; Verus checks them against the approved spec; trusted-boundary items get the tests the plan named; an LLM verifier looks for requirements with no proof and no passing check. Gaps return to the coding agent until none remain. A contradiction in the spec returns to the developer as a decision. — [Microsoft Research blog, Step 6](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
  11. Mutation check, inside `pdd-prove`: one bug is planted per `ensures` clause in a scratch copy. Verus must reject each mutant on the clause it targets. A mutant that verifies means the clause constrains nothing and comes back as a gap. The page’s examples of planted bugs include refusing everyone, and checking the username before the email. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
  12. On success, `pdd-prove` opens pull requests that add the verified code on the production path, not a side copy, and the PR description is a receipt: each requirement marked formally verified or empirically checked, each trusted piece listed with its test. The developer reviews that receipt against the intent already signed, rather than reading generated code line by line. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  13. `pdd-ship` puts the Verus check in CI beside the empirical tests. Later edits by a person or an agent must keep proving the same spec. A broken proof fails the build like a failing test. Tests that only re-check a proved property (example: example-based tests of the password gate) are pruned. Tests that guard the trusted boundary stay. — [Microsoft Research blog, Step 7](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Hard stop, not a silent rewrite: “If an async construct can’t be expressed in the pinned Verus, the task stops and asks the team instead of switching to a non-async helper or weakening a contract.” — [Microsoft Research blog, Step 6](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/); same rule in the build-plan caption on [GitHub Next](https://githubnext.com/projects/pdd-kit/)
- What the human must not skip: “A proof shows the code meets the spec, not that the spec is what you meant.” A wrong spec “is still possible.” The kit’s answer is explicit requirements, flagged ambiguity, human approval before verification, two reviewer agents, and the mutation check. — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- What an agent must not do alone: pick one reading of an ambiguous sentence without telling the team. The documented example is “only with the right password” (token implies right password), which a login that refuses everyone still satisfies, versus “exactly when” (right password always yields a token, except a real 500). The team chose both directions. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Agent-written tests are called out as a failure mode of the status quo: they “tend to pass for the code it wrote, not the code you meant.” PDD still uses tests, but only where a proof cannot reach, and labels a passing test as evidence rather than proof. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Chopin chat entry points that are documented for the Planner, not as PDD commands: `@chopin` asks the Planner to act; `/research` starts a research child document. There is no documented `/pdd-plan` slash command in the Chopin README. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Integration bug that stopped planning, since fixed in public Chopin: when `invoke_planner` returned `planner-owner-unavailable` because nobody was signed in, `pdd-plan` retried by reading the document by UUID. `read_document` then had no canonical URL, so `pdd-plan` “refused to hand off” and “planning stopped even after the user signed in.” PR 305 (merged 2026-10-06) makes `read_document` return `url`. — [chopin#305](https://github.com/githubnext/chopin/pull/305)

### Inferences
- The correct human/agent split is: human writes the rough requirement and approves decisions, scope, each trust assumption, and the spec; the agent may write the Verus text, proofs, production wiring, and empirical tests only after that approval. Generating the spec from already-written code and then approving it without the decision cards is the failure mode the authors name (“vibe verification” and silent ambiguity).
- Regeneration is “send the gap back,” not “edit the spec until Verus accepts.” Spec changes that contradict the approved intent are supposed to surface as new human decisions.
- Drift control after the first proof is CI (`pdd-ship`), not a second copy of the model. The spec stays the contract; the proof is re-run on later changes.
- A trial that implements first in a normal coding agent and only afterwards asks for a proof has left the documented order. The proved functions in the published spec are shown with `unimplemented!()` bodies until the proof stage. — [GitHub Next, login_flow listing](https://githubnext.com/projects/pdd-kit/)

### Gaps
- The exact chat utterance or button, other than **Build plan** and the two-sentence login request, that starts `pdd-plan` on a self-hosted Chopin. Not in the public Chopin README.
- How many model calls one login-sized run takes, and whether the two reviewer agents and the LLM gap-reviewer are the same model as the coder. Not stated.
- Whether a failed mutation check can be overridden by a human. The pages say it comes back as a gap. They do not describe an override.

## Spec and code conventions

### Takeaway
The spec is ordinary Rust plus Verus `requires` and `ensures`, with a plain-English comment on every clause. Proved logic moves onto the production path under a `verified/` module. External systems stay behind named trusted wrappers. The documented forbidden move is weakening or shrinking a contract so the proof succeeds.

### Cited Findings
- Spec language: Rust with Verus. Each function gets an `ensures` block about its result `r`. Clauses are if-then (`==>`). Verus checks them at build time and points at a clause that could fail for even one input. `requires` on an `async fn` is checked when the future is created. `ensures` describes `r` only once the future finishes. The doc comment on `login_flow` says “No promise the future completes.” A hung query therefore breaks no clause. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog, Figure 14](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Layout in the login case study, not a generic template for every app:
  - Existing handler: `backend/src/handlers/auth.rs`, lines 82–127 at commit `8b8b70f`, before verification did every step.
  - After verification, that handler only unpacks the request and awaits `login_flow`.
  - Spec and proved functions: `verified/src/login.rs` (the project page’s file map says 368 lines), compiled as a module of the real backend crate so contracts name real `Customer`, `Admin`, `Config`, and `AppError` types, not copies.
  - Header of that file lists five functions to be proved (`login_flow`, `login_core`, `issue_token`, `lookup_target`, `decide_login`) and six trusted pieces (two lookups, `check_password` over bcrypt, `expiry_from_now` over the clock, `generate_token` over JWT signing, `database_error` for the sqlx message). “Only the axum extractor unpacking in `login` stays outside.”
  - The PR adds a `verified/` crate with `login_flow`, its pieces, their proofs, and the trusted wrappers with their tests, and routes the real handler through `login_flow`. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog, Step 7](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- File layering the authors use inside `verified/src/login.rs`: header; turn Verus on; `external_type_specification` for existing types (pool opaque, customer row fields visible); uninterpreted spec functions (`customer_on_file`, `admin_on_file`, `bcrypt_verdict`, `token_claims`, lookup-fails and signing-fails flags); `open spec fn` ghost definitions such as `password_matches` and `named_account` that never run and are erased by the compiler; trusted contracts; then the five proved contracts. Proved bodies are “written by the proof stage.” The spec file as approved still shows `unimplemented!()`. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Trust encoding: a lookup keeps its real code and gets `#[verifier::external_fn_specification]`. Wrappers Verus must not read (`check_password`, `expiry_from_now`) are `#[verifier::external_body]`. The pool uses `external_type_specification`, `external_body`, and `reject_recursive_types`. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- The two-sentence request became eight `ensures` clauses on `login_flow`. Four of those pin existing refusals (400 when neither email nor username is sent, 401 for an unknown account, 401 for a wrong password, 500 when bcrypt cannot read the stored hash) “so the storefront and the existing API tests keep working,” even though the human did not ask for them. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Smaller lemmas: `login_flow` is proved from the contracts of `lookup_target`, `login_core`, `decide_login`, and `issue_token`, without re-reading their bodies. Changing `decide_login`’s body re-proves only `decide_login`. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Explicit non-goals written into the plan for this spec: that the login future ever completes; that an out-of-range clock will not panic (`expiry_from_now` “panics if the time is out of range, as today’s login does; that panic is not promised against”); timing or rate of attempts; how other endpoints read the role; anything nobody wrote down; UI judgment such as whether an error message helps. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog, Step 4](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Forbidden shortcuts the authors state in the loop, not as a style-guide file:
  - Do not weaken a contract, and do not switch an async proof to a non-async helper, when the pinned Verus cannot express a construct. Stop and ask. — [Microsoft Research blog, Step 6](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
  - Do not accept a spec that verifies but rules out almost nothing. They name that “vibe verification.” The mutation check is the mechanical guard. — [GitHub Next, PDD Kit, section “Vibe Verification”](https://githubnext.com/projects/pdd-kit/)
  - Do not let the agent settle an ambiguity silently. One-directional implications are the worked example. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
  - Do not treat the proof as proof of the trusted wrappers. “The guarantees are only as strong as the six trusted pieces,” and “two carry most of the weight: `check_password` for every password promise, and the two lookups for every promise about which account a token names. Tests support them, but nothing proves them.” — [GitHub Next, PDD Kit, “What to Expect”](https://githubnext.com/projects/pdd-kit/)
  - Do not review only the generated code and skip the spec. The spec is what the proof is held to. — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Comparison the authors draw with other methods: Spec-Driven Development has nothing that enforces the spec; Test-Driven Development samples inputs and tests are often written to match what was built; an agent checking its own work is the same model that wrote the code; Proof-Driven Development is Verus checking a proof against the spec a human approved. PDD “keeps the planning loop from SDD and still uses tests where a proof can’t reach.” Spec Kit is the analogy they use for packaging, not a component of this kit (`https://github.com/github/spec-kit`). — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)

### Inferences
- There is no published naming standard beyond the login case study. Copying `verified/src/login.rs` and a `verified/` crate is what they did for this one handler. Using that layout for a different module would be reconstructed, not documented.
- “Generating the spec from the code” is how `pdd-plan` starts (it reads the code), but the code is evidence of current behavior, not the source of truth. The human approval, the both-ways decision, and the extra clauses that preserve today’s 400/401/500 are what stop the spec from being a paraphrase of whatever the agent just wrote.
- Keeping specs from drifting after merge is CI re-proof against the same `ensures`, plus Chopin’s separate decision records so a later prose rewrite does not silently change a decision. Both mechanisms are described. Neither is a published linter command.

### Gaps
- No standalone spec-language manual, grammar, or forbidden-pattern checklist beyond the login walkthrough.
- No statement of who is allowed to edit `ensures` after `pdd-ship` without a new intent review. The pages say later changes must still prove. They do not say whether editing the spec in the same PR is blocked.
- Commit `8b8b70f` is cited as the pre-verification `auth.rs` snapshot. The repository that contains that commit was not identified. `githubnext/pdd-demo` is 404, so the snapshot cannot be checked out from a public URL found in this research.

## Supported application shapes

### Takeaway
The only first-class shape the authors demonstrate is a Rust full-stack app. The case study is Pet Store: Axum and SQLite on the server, a Leptos storefront compiled to WebAssembly, about 5,000 lines, with only the login handler taken through the proof. Other languages are outside the verified path unless the code is ported to Rust or calls verified Rust through bindings. A non-Rust web stack does not become verified by being planned in Chopin.

### Cited Findings
- “To show PDD Kit working with an existing codebase, we built Pet Store, a full-stack web app written in Rust, with an Axum and SQLite backend and a Leptos storefront, about 5,000 lines in all.” The backend “handles inventory, promotions, orders and checkout.” The storefront is “compiled to WebAssembly.” — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- They recommend starting at the highest-cost bug, “such as login, payments, or permissions, and verify the rest of a codebase one piece at a time.” The blog says the same process can verify functional correctness of the whole app, but the published walkthrough is login only. “Pet Store’s login is one piece of one app.” — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- “It only verifies Rust today since that’s the language Verus works on.” “Code in other languages can be ported to Rust and checked against the same spec, and non-Rust projects can call verified Rust through bindings.” “If your codebase is already Rust, there’s nothing to migrate.” “No new language, no change in how you produce the underlying code” means the implementation language stays Rust, not that every stack is in scope. — [GitHub Next on X, 2026-10-21](https://x.com/GitHubNext/status/2112936525457740139); [Microsoft Research blog, “Why Rust”](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- What a proof cannot see, so it falls off the formal path even inside this Rust app: third-party crates (bcrypt, sqlx, jsonwebtoken), live database contents, the clock, termination and latency, UI quality, and anything not written down. Empirical substitutes named: unit and end-to-end tests, terminal automation, browser agents, and computer-use agents “for anything a browser can’t drive.” — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Verus can put contracts on async functions, which is why the login handler’s glue could be specified. That is support for async Rust in this pin, not support for a JavaScript or C# async stack. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Stack Overflow 2026 is cited only as motivation (44.4% of respondents in full-stack, back-end, or front-end roles; n = 25,767), not as a compatibility claim. — [GitHub Next on X, 2026-11-04](https://x.com/GitHubNext/status/2118105391707967967)

### Inferences
- Axum + SQLite + Leptos is the demonstrated shape, not a framework allow-list. Nothing says Actix, Poem, Postgres, or MySQL is supported or rejected. Leaving the crates Verus cannot see (sqlx, bcrypt, a JWT library) inside the trusted boundary is the documented pattern, so a different Rust web stack could in principle be treated the same way. That generalization is an inference. The authors did not show a second stack.
- A Node, .NET, or Python web app on the trial machine would leave the verified path. Planning it in Chopin would produce a Chopin document, not a Verus proof. The documented alternatives are a Rust port or bindings to verified Rust, both of which are described at high level with no binding example and no porting guide.
- “Full-stack web app” in the blog title is the Pet Store claim, not a statement that the UI is proved. The Leptos storefront is outside the login proof. The handler shell that unpacks the request is trusted and tested, not proved.

### Gaps
- No list of supported Rust crates, HTTP frameworks, or database drivers.
- No example of calling verified Rust from another language, so “bindings” is a stated option without a documented interface.
- Whether payments or permissions in Pet Store were actually proved. The pages say they are good places to start and that login was the piece they walked through. They do not report a second proved module.
- No statement about browser-only front ends, GraphQL, or serverless. Those would be outside the demonstrated path.

## Known issues, open bugs, and documented limitations

### Takeaway
A first trial fails closed because the kit is not publicly installable. If a preview environment is available, the documented ways to get a meaningless green check are a one-sided spec, a weakened contract, and trusted wrappers (especially bcrypt and the two SQL lookups) that are only tested. Several Chopin integration bugs that blocked `pdd-plan` were fixed in October 2026; they are host bugs, and fixing them did not publish the kit.

### Cited Findings
- Research prototype, Rust only, proofs “still take real effort, mostly in producing the proof itself.” The authors “want to be upfront about the weakest links”: the six trusted pieces, with `check_password` and the two lookups carrying most of the weight. “We expect that trusted boundary to shrink significantly over time” is a hope, not a current capability. — [GitHub Next, PDD Kit, “What to Expect”](https://githubnext.com/projects/pdd-kit/); same wording on [Microsoft Research](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- “Verus hardened the code and uncovered existing bugs, proving the absence of login bugs with respect to the specification.” The bugs are not enumerated. The proof claim is relative to the spec and the trust assumptions, not absolute absence of login bugs. — [Microsoft Research blog, at a glance](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- What is proved, tested, and only assumed, in the authors’ own partition:
  - Proved: bodies of `login_flow`, `login_core`, `lookup_target`, `decide_login`, and `issue_token` meet their contracts for every input. Login picks the table, compares the password to the named account’s hash, issues a token with that account’s id and the table’s role, and otherwise returns today’s 400, 401, or 500 with no token.
  - Trusted and tested: two lookups, `check_password` over bcrypt, the clock, token signing, the database error message, and request unpacking.
  - Assumed, not tested by the app’s suite: Verus, the Rust compiler, Z3, and vstd. — [Microsoft Research blog, “What the proof does and doesn’t say”](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- A spec that is wrong still verifies the wrong property. The FAQ states that directly. — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Async limitation: if the pinned Verus cannot express a construct, the task must stop. Continuing by weakening the contract is the failure mode they forbid, which means a first proof can legitimately stall on a question rather than finish. — [Microsoft Research blog, Step 6](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- Termination, latency, rate limits, clock-range panics, and “how other endpoints read the role” are explicitly not promised. A trial that treats a green Verus run as “login is secure” has over-claimed. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Host limits that will stop a local trial even before PDD Kit: no public Chopin instance; GitHub Enterprise Server endpoints are not configurable; the Planner normally reads the default branch captured when the session starts, not a local dirty tree, except under `HARNESS=atomic`; one Chopin process may write the database at a time; horizontal scaling is not supported; `/research` web search via a self-registered GitHub App “is unconfirmed.” — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- Fixed host bug that previously aborted `pdd-plan`: missing `url` on `read_document` (chopin#305, merged 2026-10-06). The PR says the live handoff was “not run” on that branch and would be checked on `dev/integration` later. — [chopin#305](https://github.com/githubnext/chopin/pull/305)
- No open GitHub issue whose title is a PDD Kit defect was in the top results for `repo:githubnext/chopin pdd OR verus OR "proof-driven"`. The matches were closed pull requests about the Atomic harness and MCP, not an issue labeled as a Verus failure. — [GitHub issues search](https://api.github.com/search/issues?q=repo:githubnext/chopin+pdd+OR+verus+OR+%22proof-driven%22&per_page=10)
- Secondary limits articles (DEV Community, 2026-10-22) restate Rust-only, research-preview, and “vague requirements make unprovable specs.” They are not an official bug list. DEV also points at Microsoft Research’s separate `verus-proof-synthesis` work (AutoVerus and VeruSAGE) as underlying research, not as the kit’s install. — [DEV Community](https://dev.to/piekwerk/githubs-pdd-kit-proves-agent-written-rust-correct-5-steps-and-3-limits-l7l)

### Inferences
- The reliable first-trial failure is installing the wrong `pdd-kit` or booting public Chopin and finding no Verus workflow. That is a packaging gap, not a transient bug.
- The login proof’s empirical layer can fail independently of Verus: `cargo test -p backend check_password` is the kind of command the plan records, and a failing bcrypt or sqlx test means the trust assumption is false even if the proof checks. The authors say the password and account-identity promises then “hold only on paper.”
- Because `githubnext/pdd-demo` is 404 and Pet Store is not linked, a trial cannot reproduce the published login proof from a public tree. Any local Rust app would be a new proof effort, not a replay.

### Gaps
- No open-bug list, changelog, or known-issues page for the kit.
- The “existing bugs” Verus found in Pet Store login are not listed, so they cannot be used as a regression oracle.
- Whether chopin#305’s handoff was re-tested with the actual `pdd-plan` extension after merge is not in the PR text. The PR says that check was still to be done on `dev/integration`.

## Cost and time shape

### Takeaway
Dollar cost and model-call counts are not documented. The only timing numbers are for the deterministic Verus check: usually seconds for a small module, up to minutes for a larger codebase. Producing the proof is described as the expensive part, without a measured duration for the Pet Store login run.

### Cited Findings
- “How long the check takes varies with the size of the code and how hard the proofs are: usually seconds for a small module, up to minutes for a larger codebase.” The check “runs wherever your Rust builds.” It is deterministic. Models are not in the pass/fail path. — [Microsoft Research blog, FAQ](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- “Proofs still take real effort, mostly in producing the proof itself, and a clear spec is what keeps that effort down.” Historical formal proofs “took years”; the authors say agents have changed that, but they do not give a wall-clock time for Pet Store. — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/); [Microsoft Research blog](https://www.microsoft.com/en-us/research/blog/formally-verifying-agent-written-full-stack-web-apps-with-pdd-kit/)
- The review-shift diagram’s widths “are illustrative, not measured.” — [GitHub Next, PDD Kit](https://githubnext.com/projects/pdd-kit/)
- Login’s prove stage is seven tasks on “a development machine,” run in the background after **Build plan**. No token counts, no dollar figures, no retry budget. — [GitHub Next, PDD Kit, Build plan](https://githubnext.com/projects/pdd-kit/)
- Chopin, if used as the host, sends document text, chat, and repository material selected by the Planner to the harness model provider (GitHub Copilot by default; under `HARNESS=atomic`, whatever `HARNESS_AUTH` selects). That is a data-flow fact, not a price. — [Chopin README](https://github.com/githubnext/chopin/blob/main/README.md)
- No pricing, Copilot premium-request estimate, or Z3 time limit appears on the project page or the MSR FAQ.

### Inferences
- A login-sized module, once the spec is stable, is in the “seconds to minutes” band for the solver check itself. The unmeasured part is the agent loop that writes the proof, the two reviewer agents, the mutation campaign (one mutant per `ensures` clause; login has eight clauses on `login_flow` plus the smaller functions), and human decision time.
- Cost of a macOS trial that never obtains the extension is only the cost of cloning Chopin (Bun, Docker, Copilot) and is not a PDD Kit run. Copilot entitlement is a prerequisite of default Chopin, so that cost is a host cost, not a published kit price.

### Gaps
- Model call count, context size, and dollar cost for one Pet Store login run: not published.
- Wall-clock time of `pdd-prove` for the seven login tasks: not published. Only the solver check has a range, and that range is qualitative (“usually seconds,” “up to minutes”).
- Whether mutation testing re-runs Verus once per clause or batches mutants: not stated. The page says one bug is planted per clause and Verus must reject each mutant.
- Z3 resource limits (`rlimit`, memory) used by the kit: not stated.

## Update, checked 2026-11-07 (after the sections above were written)

- Resolves the gap “Whether Verus’s pinned revision builds on macOS Apple Silicon or Intel”: the verus-lang/verus latest release (`release/0.2026.11.02.8bf68ee`, published 2026-11-02) publishes `verus-…-arm64-macos.zip` and `…-x86-macos.zip` as prebuilt assets. A current Apple Silicon Mac (the trial machine: macOS 26.7.2, arm64) can use a prebuilt binary without a source build; Z3 ships inside the release.
- New gap opened: the trial machine has no Rust toolchain (`cargo` absent as of 2026-11-07), so rustup is a prerequisite, and the rustup toolchain version the binary requires is only discoverable by running it.
- A trial scaffold now lives at `../trial/` in the research root (spec-first template, decision cards, trusted-boundary tests, mutation table, `install-verus.sh` / `verify.sh`). It is an Experiment B (“Verus plus the harness”) reconstruction, not a PDD Kit install; the PDD Kit access findings above are unchanged.
