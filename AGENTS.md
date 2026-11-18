# unshittify

This repo holds a method, packaged as an agent skill, for rebuilding web apps as minimal, text-first native clients (Rust + Tauri).

## If you're asked to rebuild, replace or unshittify a web app

Read `skills/unshittify/SKILL.md` and follow it. It points to the reference files and the starter template in the same folder. This works the same in every agent: the skill is plain Markdown plus a template directory.

## If you're asked to change the method itself

- `skills/unshittify/SKILL.md` holds the workflow and ground rules. Keep it under about 200 lines; detail goes in `references/`.
- `skills/unshittify/references/ui-style.md` and `template/ui/style.css` hold the look. This is the main customization point.
- `skills/unshittify/template/` holds the starter app. It must stay buildable: after changing Rust code, run `cd skills/unshittify/template/src-tauri && cargo run -- --render /notes`.
- Stay agent-agnostic: describe browser tools by what they do, not by one vendor's tool names (tables of examples are fine).
- Explain *why* in instructions instead of stacking MUSTs. Agents follow reasons better than rules.
