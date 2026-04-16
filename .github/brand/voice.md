# Voice & Tone

The visual system is terse, dense, and fact-bound. The writing is too.

## Principles

1. **Numbers over adjectives.** `6 recipes · 5 pathways · 54 workflows` beats `a comprehensive collection`.
2. **Verbs over nouns.** `Build a parachain.` beats `Parachain development.`
3. **Monospace-friendly.** Short lines. No em-dashes where a hyphen works. ASCII over Unicode where possible (terminal panels render the literal characters).
4. **No marketing voice.** No "unlock", "empower", "seamless", "robust", "elegant", "blazing fast", "battle-tested".
5. **No first-person-plural marketing "we".** Instructions use imperative ("Run `cargo test`"). Descriptions of the repo use present tense third-person ("The Cookbook ships tested recipes…").
6. **Brand name discipline.** `Polkadot Cookbook` (title case) or `the Cookbook` in running text. Never `cookbook` lowercase, never `PolkaDot`.

## Microcopy examples

| Don't                                           | Do                                              |
| ----------------------------------------------- | ----------------------------------------------- |
| "Our comprehensive suite of recipes"            | "6 recipes, 5 pathways"                         |
| "Unlock the power of Polkadot development"      | "Build on Polkadot."                            |
| "Blazing-fast parachain scaffolding"            | "`dot create` scaffolds a parachain in <5s"     |
| "We maintain robust test coverage"              | "SDK coverage ≥80%, enforced in CI"             |
| "Check out our amazing contributors!"           | "12 contributors, 264 commits in the last 90d"  |
| "Get started today!"                            | "Run: `dot create --title my-parachain`"        |

## Tagline canon

Primary: **Practical, tested recipes for building on Polkadot.**

Long form: **Build runtime logic, smart contracts, dApps, and cross-chain applications with working code examples.**

Short (≤5 words, for social card): **Build on Polkadot.**

Never invent new taglines. If you need a new variant, add it here first.

## Pathway naming

Five pathway labels, always in this order and casing:

1. Pallets
2. Contracts
3. Transactions
4. XCM
5. Networks

(`Cross-chain-transactions/` is the *directory*; `XCM` is the *pathway label*. Don't mix.)

## Commit/PR prose

- Conventional commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `ci:`).
- PR titles under 70 characters.
- PR bodies: Summary (1–3 bullets), Test Plan (checklist). Always ship with the checklist completed.
- Co-authorship lines: only real humans. No AI attributions.

## Error & status strings

The CLI (`dot`) already uses pink for success via the `colored` crate. Extend this convention:

- Success: `color.primary.pink`, prefix `✓`
- Info: `color.surface.cream` opacity 0.7, prefix `▸`
- Warn: `color.semantic.warn`, prefix `!`
- Error: `color.primary.pink` on `color.surface.terminal`, prefix `✗`

(Pink does double duty as brand + success. If this ever causes contrast confusion, split success into its own teal token — not today's problem.)

## What to avoid in text

- Emojis in headlines, titles, commit messages. (OK in callout leading icons — already established in README.)
- Hype dates ("coming soon!", "stay tuned"). Link the issue instead.
- Celebratory language in release notes. Facts speak louder.
- References to "the team" in external-facing prose (docs, README, release notes). Use "the project" or passive voice.
