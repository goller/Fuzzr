[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg?style=flat-square)](http://unlicense.org/)
[![Build status](https://img.shields.io/github/workflow/status/FuzzrNet/fuzzr/Rust/main?style=flat-square)](https://github.com/FuzzrNet/fuzzr/actions?query=branch:main)
[![Discord](https://img.shields.io/discord/788559109011406889?style=flat-square&logo=discord)](https://discord.gg/cvgbcSwYzy)

# Fuzzr

Fuzzr is intended to be a censorship-resistant platform for publishing, curation, and browsing of all content (with the explicit exception of HTML). At some point, an optional cryptocurrency miner will be added, so as to assist in monetization for content creators, scalable and low-latency content distribution that cannot be interfered with, and incentivizing privacy measures.

A major focus of our project is ensuring content distribution, censorship resistance, and keeping users as anonymous as they care to be. No user registration is required. Encryption of content at rest isn't necessarily a focus for this project, but ensuring secure, unfettered encrypted connections to all anonymized peers is.

This project is built with Rust for native desktop (and laptop) OS platforms, using [iced](https://github.com/hecrj/iced) for UI and [embedded IPFS](https://github.com/ipfs-rust/ipfs-embed/) for data.

## FuzzrWeb vs FuzzrNet

We want to reinvent the web to be much, much simpler, and to move past it. The Web is __mostly__ centralized (including much of "DWeb" infrastructure), but the broader Internet __mostly__ isn't. That's why the FuzzrWeb should eventually move to the FuzzrNet, and the Fuzzr client can both serve FuzzrWeb sites, access other Fuzzr sites on the FuzzrNet, and also mine to compensate contributors, and stream video and audio.

This is an architecture for an actual locally-run p2p app, not a simple web-based "decentralized app". Any FuzzrWeb sites will not include a crypto miner of any kind. That's called cryptojacking, it's incredibly inefficient, and is wrong to do without someone's explicit knowledge.

To learn more about our plans for FuzzrWeb, Fuzzr's bridge to the web, take a look at this document: [Fuzzr Web Publishing](notes/web_publishing.md)

## Status

This project is a massive work in progress. Not all of these goals are currently met, but we do intend to. More thoughts in the notes directory. Feedback always appreciated, feel free to  Drop us an issue.

Only desktop Linux platforms are officially supported for now. The code should compile to other platforms, but no guarantees it will work out of the box. As the project grows, we'll have more contributors who develop on other systems, but as of now, the overwhelming majority of our contributors run Linux.

Support is eventually planned for all platforms except for web. As with anything, if you see something you would like to see, or if something seems broken, let us know.

To run the project, all you need to do is [install Rust](https://rustup.rs), check out the code using git, and run `cargo run` in the project directory. It's pretty simple compared to other projects, and should work well out of the box easily enough.

This project is intended to make the web and all web technologies less necessary than they once were by doing what people use the web for in a standardized and hyper-minimal way.

For more, see our milestones document: [Periodic Project Milestones and Goals](notes/milestones.md)

## Contribute

We love to hear feedback and ideas. Feel free to leave some issues, or go through our current issues and PRs, and give us your thoughts.

If you'd like to reach out and learn more, we use a Discord community server to coordinate. There's a link to join our server at the top of this README.

Feel free to send some XMR to the address below. And if you do so, feel free to reach out, or promote what we're doing.

`8ADbBKaunVWjdg5aWQ5ZBNDACdPVMTUBnKETaZbUZ8gMfDfpwhcBeo31kfUgCJKATMPaqmsUoxBwicTpRLg4p4F57kPJ5ab`
