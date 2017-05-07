# A Certain Pull Request-merging Railgun

**Railgun** is a service for monitoring and merging GitHub Pull Requests.

## Usage

When you set up Railgun, you connect it to your GitHub repository via a webhook.

Railgun waits for your PR to have a fully green commit status by listening to
for status change events. When all of the PR's status checks are successful,
Railgun performs a speculative merge of the PR's head into the base, and then
starts CI builds on that merge.

If the builds are successful, Railgun will automatically merge your PR!

This process ensures that your master branch never breaks by testing every
commit just before it merges. This guarantees that no other changes have merged
into the base branch that might interact badly with the PR.

## Requirements

* Postgres

## Configuration

Railgun is configured through environment variables.

| Var            | Description                      | Required |
|----------------|----------------------------------|----------|
| `DATABASE_URL` | Connection URL for your database | Yes      |

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
