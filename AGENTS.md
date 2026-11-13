# AGENTS

LANG   this file uses the notation below. keep writing it in this notation.
FORM   RULE <topic> = <value>        hard rule
       RULE <topic> : <predicate>    hard rule, predicate form
       WHY  <reason>                 why the rule exists
       THEN <action>                 what to do when the rule triggers
       EX+  <good example>
       EX-  <bad example>
       REF  <path>                   where to look
       EXC  <exception>              where a rule is broken, and on whose word
READ   RULE is binding. WHY is context, not negotiable. EX is literal.

## Project

RULE product = plate-menu, an egui widget: a menu of wide plates, walked by
     the keys and the pointer and searched by typing
REF  README.md
RULE users = zyterm and obusina, each through a path dependency
WHY  one menu, one egui; two copies drift apart
THEN a change for one user keeps the other working; a behaviour one of them
     does not want is an option of the menu, its default the old behaviour
RULE lang = rust, edition 2024
RULE toolkit = egui; `default_fonts` is left to the application
RULE license = MIT

## Code

RULE crate.knowledge : the crate knows egui and nothing of an application
EX-  a text of its own shown to the user; it takes every text from the caller
RULE file.max_lines = 1000
THEN split into submodules
RULE module.subject : one module is one subject
RULE docs.deny = `#![deny(missing_docs)]`; every public item is documented
RULE comment.body : no inline comments inside a function body
RULE format = `cargo fmt`, rustfmt with its defaults

## Documentation

RULE doc.readme : README.md says what the menu does now: the calls, the keys,
     the pointer, with pictures
RULE text.lang = simplified english (/workspace/AGENTS.md, `RULE text.lang`)
RULE doc.picture : a picture of the README is one its picture test rendered
THEN `img/<name>.png`, named in the README by `[picture-<name>]`; `build.rs`
     writes the labels as `data:` URLs for rustdoc

## Tests

RULE test.picture.tool = `egui_kittest` with `wgpu` and `snapshot`;
     `tests/snapshots.rs`, one test a picture, output path `img`, a fixed size
     and `Theme::Dark`
RULE test.picture.renderer = lavapipe, the software Vulkan of mesa
RULE test.picture.update : a picture changes only with the widget, and is
     looked at before it is committed
THEN `UPDATE_SNAPSHOTS=1 cargo test --test snapshots`, then open every PNG
     that changed
RULE test.input : what a key or the pointer does has a test that hands the
     menu egui events
REF  tests/pointer.rs, tests/tree.rs

## Checks before finishing

RULE checks : all are clean before the work is reported done
THEN cargo fmt
     cargo clippy --all-targets
     cargo test
     cargo build --examples

## Commits

RULE commit : finished work is committed, after the checks are clean
THEN one commit per subject; the subject says what the work does, the body
     what it does it for
RULE commit.clean : a message carries nothing of the machine it was written on

## Exceptions

RULE exception : a rule is broken only when the user asks for it
