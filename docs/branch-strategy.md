# Gitflow Branch Strategy

## Permanent branches

- **main** contains release-ready code. Releases are tagged from this branch.
- **develop** integrates changes for the next release.

Configure GitHub rulesets so both branches require pull requests, dismiss stale approvals, require conversations to be resolved, and block force pushes and deletion. Documentation alone does not enforce these settings.

## Temporary branches

Create work branches from the base shown below. Delete them after merging.

| Prefix | Create from | PR target | Purpose |
|---|---|---|---|
| feature/<topic> | develop | develop | New functionality |
| fix/<topic> | develop | develop | Normal bug fixes |
| docs/<topic> | develop | develop | Documentation |
| chore/<topic> | develop | develop | Maintenance and automation |
| release/<version> | develop | main | Stabilize a release |
| hotfix/<topic> | main | main | Urgent production fix |

Examples: feature/html-tables, release/0.2.0, hotfix/pdf-crash.

Feature, fix, docs, and chore branches are temporary; do not create empty branches just to reserve a prefix.

## Start a feature

1. Fetch and switch to develop.
2. Create a branch such as feature/html-tables from develop.
3. Commit the focused change and push the feature branch.
4. Open a pull request from the feature branch to develop.
5. Address review feedback; merge by squash after approval and resolved conversations.
6. Delete the feature branch after merge.

## Release flow

1. Create release/X.Y.Z from develop.
2. Update version, changelog, documentation, and release artifacts on the release branch.
3. Open a pull request from release/X.Y.Z to main; merge after review.
4. Tag the resulting main commit as vX.Y.Z and publish the release.
5. Open a pull request to bring release-only changes back into develop.

## Hotfix flow

1. Create hotfix/<topic> from main.
2. Fix the production issue and open a pull request to main.
3. Tag and publish the corrective release.
4. Open a follow-up pull request to bring the fix into develop.

## Review and merge

- The author cannot approve their own pull request. Request an independent reviewer when available.
- New commits should invalidate previous approvals; resolve all review conversations before merging.
- Use squash merges to keep permanent branch history concise.
- GitHub rulesets enforce protected destination branches. GitHub does not natively enforce every source-prefix-to-target pairing; review that convention in each pull request.

See [contributing](../CONTRIBUTING.md) and [release process](releases.md).