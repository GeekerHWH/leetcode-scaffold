## leetcode-scaffold PR FAQs
A local unit test template render for Rust and Go. It is fully written in Rust,
and rendering is idempotent. It's under development right now for personal Rust-learning
journey.

### FAQs
1. why do I need leetcode-scaffold?

While leetcode offers a great test cases integration on web, it charges you fees
for premium-only debugging online. Even if you can debugging online, it is inconvenient
for people who work on strong type languages like Rust to debug without intellisense.

2. how to use it?

for now, you have to compile it yourself:
```bash
git clone https://github.com/GeekerHWH/leetcode-scaffold.git

cd leetcode-scaffold
cargo build --release

# execute it to render rust leetcode template
chmod u+x ./leetcode-scaffold && ./leetcode-scaffold rust
```

3. to do list OR roadmap?

- [ ] support go.
- [ ] using looping to support endless test cases.
- [ ] Github Actions for release.