# Multi Git Graph CLI

[![stability-wip](https://img.shields.io/badge/stability-wip-lightgrey.svg)]()

Allows you to view your contribution graph for multiple Git servers combined, in your CLI.

<img src="demo/demo.png" alt="demo" width="700"/>

### Configuration

Provide your tokens, server address, and color preferences in your local `.env` file.
A [color scheme](src/colors.rs) can be set using the `COLOR_SCHEME` variable.

### Current support

|           | Supported | Environment Variables                                    |
| --------- | --------- | -------------------------------------------------------- |
| Github    | ✅        | `GITHUB_TOKEN`, `GITHUB_USERNAME`                        |
| GitLab    | ✅        | `GITLAB_TOKEN`, `GITLAB_SERVER`, `GITLAB_USERNAME`       |
| Gitea     | ✅        | `GITEA_TOKEN`, `GITEA_SERVER`, `GITEA_USERNAME`          |
| Codeberg  | ✅        | `CODEBERG_TOKEN`, `CODEBERG_SERVER`, `CODEBERG_USERNAME` |
| BitBucket | ❌        |
