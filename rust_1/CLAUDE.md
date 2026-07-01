# CLAUDE.md

## Git Conventions

- When making git commits, NEVER include Co-Authored-By metadata in commit messages unless explicitly asked.
- Commit changes in small, modular chunks organized by feature or module. Each commit should focus on a single logical change.
- Scope each commit to a specific module (e.g., `src/package-smart-parking/`, `src/utils/`, `src/components/`). Avoid bundling unrelated changes across different modules in one commit.
- Before creating a GitHub PR, first check if `gh` CLI is installed and authenticated. If not, provide the manual browser URL immediately instead of attempting and failing.

## Working Style

- When faced with a decision or choice between multiple approaches, use the AskUserQuestion tool to clarify before proceeding.
- When asked to explore or plan, ask clarifying questions upfront before doing extensive autonomous exploration. Present a brief plan and wait for confirmation before proceeding.
- When asked to optimize or review code, scope the task to the specific files or area mentioned. Ask for clarification if the scope is ambiguous (single file vs whole project).
