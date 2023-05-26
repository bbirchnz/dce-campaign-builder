![Build](https://github.com/bbirchnz/dce-campaign-builder/actions/workflows/ci.yml/badge.svg)

# DCE Campaign Builder

Build new campaigns for the [Dynamic Campaign Engine](https://forum.dcs.world/topic/162712-dce-campaigns/)

Designed to make the campaign design and build process user friendly and faster, but all credit must go to MBot, Miguel21 and PB0_CEF for their amazing work on DCE.

Warning: this is an early stage project built for personal use, is sparsely documented and likely full of bugs!

Latest build can be grabbed from Actions

## Build and Setup

This is built in Rust with [Dioxus](https://dioxuslabs.com/) as the UI library. It also relies on [proj](https://proj.org/about.html) for coordinate transforms.

1. Build Proj - Follow these [instructions](https://github.com/georust/proj/pull/79#issuecomment-1308751602)
2. Copy the built dlls from vcpkg to repo/target/debug
3. `cargo build`
4. `cargo run`

## Features and todos

- [x] import a template mission from dcs and create a DCE compatible folder structure
- [x] export to DCE Manager compatible zip
- [x] CAP target zones
- [x] Strike target groups
- [x] Campaign specific loadouts and liveries
- [x] Edit home bases
- [x] Set task priorities
- [x] Support DCE triggers
- [ ] Intercepts for base protection
- [ ] Add extra resources (briefing text and images)

and many more.
  
