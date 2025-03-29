## leetcode-scaffold PR FAQs
A local unit test template render for Rust and Go. It is fully written in Rust,
and rendering is idempotent. It's under development right now for personal Rust-learning
journey.

### FAQs
1. why do I need leetcode-scaffold?

While leetcode offers a great test cases integration on web, it charges you fees
for premium-only debugging online. Even if you can debugging online, it is inconvenient
for people who work on strong type languages like Rust to debug without intellisense.

Secondly, if you want to leetcode at work, (e.g. you are going to be fired, or you want to change job), you'd better not to be found out by your boss.

2. how to use it?

using python3 to execute templates/leetcode-problem-list.py, which will upgrade the templates/leetcode-problems.yaml

for now, you have to compile it yourself:
```bash
git clone https://github.com/GeekerHWH/leetcode-scaffold.git

# compile it
cd leetcode-scaffold && cargo build --release && cp target/release/leetcode-scaffold .

# execute it to render rust leetcode template
chmod u+x ./leetcode-scaffold

# fetch leetcode problem description and save it into ./two-sum.md
./leetcode-scaffold leetcode --url https://leetcode.cn/problems/two-sum > two-sum.md

# render rust leetcode template
./leetcode-scaffold rust
```
exploring the unit test template, and all done!

3. to do list OR roadmap?

- [ ] Github Actions to release binary for different platforms
- [ ] Grab leetcode description and inject it into template
- [ ] Containerize it for users who don't want to install rust and compile it
- [ ] support go template