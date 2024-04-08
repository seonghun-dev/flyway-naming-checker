# Flyway Naming Convention Checker

![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-enabled-brightgreen)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

GitHub Actions CI workflow for validating Flyway migration script names to ensure they follow the naming convention. 

This Github action generated from [dbanty/rust-github-action-template](https://github.com/dbanty/rust-github-action-template).


## Description

Flyway's file convention is clearly specified. This action checks that all files conform to the flyway file convention.

[Flyway: Naming Patterns Matter | Redgate](https://www.red-gate.com/blog/database-devops/flyway-naming-patterns-matter)



![flyway_naming](https://github.com/seonghun-dev/flyway-naming-checker/assets/80201773/7d448c33-125f-45b2-a6f5-027264b9a9d6)


## Example

To use this GitHub Actions workflow in your repository.


Create a YAML file (e.g., `.github/workflows/flyway-naming-check.yml`) with the following content:

```yaml

name: Flyway Naming Check

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]


jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: flyway-naming-checker
        uses: seonghun-dev/flyway-naming-checker@v0.1.0
        with:
          path: 'db/migartion' # Adjust the path to match your migration script location
```

## Preview

This script verifies that all files in the current folder match flyway naming convention.


<img width="1077" alt="screenshot 2024-04-08 오후 6 03 09" src="https://github.com/seonghun-dev/flyway-naming-checker/assets/80201773/3bc9289a-566a-447b-bf12-a392adb3815f">

