# A Certain Pull Request-merging Railgun

**Railgun** is a service for monitoring and merging GitHub Pull Requests.

## Usage

When you set up Railgun, you connect it to your GitHub repository via a webhook.

Whenever a PR is opened or updated, Railgun is notified and takes the head SHA
of the PR and performs a speculative merge into the target branch. Then it
starts CI runs on the speculative merge with your CI service. If the runs are
successful, Railgun can merge the Pull Request automatically.
