# Game Client/Server 
### Senior Design CEN4914 Spring 2021

Unity Version `2021.1.0b4`

## Running Linting Tools

Requires the `dotnet-sdk` package

```

# Locally install tools

dotnet tool restore

# Run static code analysis (tests)

./tools/lint-tests.sh

# Run static code analysis (main assembly directive)

./tools/lint-main.sh

# Run formatter (warning, this will fix any issues found immediately)

./tools/stylecheck.sh

```
