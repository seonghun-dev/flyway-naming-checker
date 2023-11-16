# Flyway Naming Convention Checker

![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-enabled-brightgreen)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

GitHub Actions CI workflow for validating Flyway migration script names to ensure they follow the naming convention. This Github action generated from [dbanty/rust-github-action-template](https://github.com/dbanty/rust-github-action-template).

## Example

To use this GitHub Actions workflow in your repository, create a YAML file (e.g., `.github/workflows/flyway-naming-check.yml`) with the following content:

```yaml
name: Flyway Naming Check

on:
  push:
    branches:
      - main

jobs:
  validate-flyway-scripts:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout Repository
      uses: actions/checkout@v2

    - name: Checkout Repository
      uses: seonghun-dev/flyway-naming-checker@v0.0.1
      with: 
        paths: 'db/migration' # Adjust the path to match your migration script location
