---
title: "Code Review Tooling"
date: 2026-02-07T00:00:00
tags: ["code-review", "tools", "automation"]
description: "Tools and platforms for code review"
---

Code Review Tooling
===================

Effective code review requires the right tools. This chapter covers popular code review platforms and automation tools.

Platform Comparison
-------------------

**GitHub Pull Requests**

Features:
- Line-by-line comments
- File-level comments
- Approvals and request changes
- Review assignments
- Status checks integration
- Rich diff view

.. code-block:: yaml

   # GitHub Actions for automated checks
   name: Code Review Checks

   on:
     pull_request:
       types: [opened, synchronize, reopened]

   jobs:
     lint:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v2
         - name: Run linter
           run: npm run lint

     test:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v2
         - name: Run tests
           run: npm test

**GitLab Merge Requests**

Features:
- Inline comments
- Discussion threads
- Approvals
- Merge request assignments
- CI/CD integration
- Code quality reports

.. code-block:: yaml

   # GitLab CI for automated checks
   stages:
     - lint
     - test

   lint:
     stage: lint
     script:
       - npm run lint
     only:
       - merge_requests

   test:
     stage: test
     script:
       - npm test
     only:
       - merge_requests

**Bitbucket Pull Requests**

Features:
- Inline comments
- Pull request tasks
- Approvals
- Code insight reports
- Jira integration
- Branch permissions

**Review Board**

Features:
- Powerful review workflow
- Custom fields
- Diff viewer
- Review groups
- Email notifications
- API access

**Phabricator**

Features:
- Differential code review
- Audit trail
- Custom workflows
- Herald rules
- Integration with other tools
- Mobile app

Automated Code Review Tools
----------------------------

**Static Analysis Tools**

**ESLint**

.. code-block:: javascript

   // .eslintrc.json
   {
     "extends": ["eslint:recommended"],
     "rules": {
       "no-unused-vars": "error",
       "no-console": "warn",
       "semi": ["error", "always"]
     }
   }

**Pylint**

.. code-block:: python

   # .pylintrc
   [MASTER]
   disable=C0114,C0115,C0116

   [FORMAT]
   indent-string='    '

   [BASIC]
   good-names=i,j,k,ex,Run,_

**SonarQube**

.. code-block:: yaml

   # sonar-project.properties
   sonar.projectKey=my-project
   sonar.sources=src
   sonar.tests=test
   sonar.python.coverage.reportPaths=coverage.xml
   sonar.exclusions=**/migrations/**

**Security Scanners**

**Snyk**

.. code-block:: bash

   # Install Snyk
   npm install -g snyk

   # Scan for vulnerabilities
   snyk test

   # Monitor for vulnerabilities
   snyk monitor

**Dependabot**

.. code-block:: yaml

   # .github/dependabot.yml
   version: 2
   updates:
     - package-ecosystem: "npm"
       directory: "/"
       schedule:
         interval: "weekly"
       open-pull-requests-limit: 10

**CodeQL**

.. code-block:: yaml

   # .github/workflows/codeql.yml
   name: CodeQL

   on:
     push:
       branches: [main]

   jobs:
     analyze:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v2
         - uses: github/codeql-action/init@v1
         - uses: github/codeql-action/analyze@v1

**Code Quality Tools**

**Prettier**

.. code-block:: json

   // .prettierrc
   {
     "semi": true,
     "trailingComma": "es5",
     "singleQuote": true,
     "printWidth": 80
   }

**Black**

.. code-block:: python

   # pyproject.toml
   [tool.black]
   line-length = 88
   target-version = ['py38']
   include = '\.pyi?$'

**Clang Format**

.. code-block:: yaml

   # .clang-format
   BasedOnStyle: Google
   IndentWidth: 2
   ColumnLimit: 80

CI/CD Integration
-----------------

**Automated Checks**

.. code-block:: yaml

   # GitHub Actions
   name: Automated Code Review

   on:
     pull_request:
       types: [opened, synchronize, reopened]

   jobs:
     review:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v2

         - name: Run linter
           run: npm run lint

         - name: Run tests
           run: npm test

         - name: Run security scan
           run: npm audit

         - name: Check code coverage
           run: npm run test:coverage

         - name: Comment PR with results
           uses: actions/github-script@v5
           with:
             script: |
               github.rest.issues.createComment({
                 issue_number: context.issue.number,
                 owner: context.repo.owner,
                 repo: context.repo.repo,
                 body: 'All automated checks passed! ✅'
               })

**Automated Review Comments**

.. code-block:: python

   # Automated review bot
   import requests
   import os

   GITHUB_TOKEN = os.getenv('GITHUB_TOKEN')
   PR_NUMBER = os.getenv('PR_NUMBER')
   REPO = os.getenv('GITHUB_REPOSITORY')

   def add_review_comment(comment):
       url = f"https://api.github.com/repos/{REPO}/issues/{PR_NUMBER}/comments"
       headers = {
           'Authorization': f'token {GITHUB_TOKEN}',
           'Accept': 'application/vnd.github.v3+json'
       }
       data = {'body': comment}
       response = requests.post(url, headers=headers, json=data)
       return response.json()

   # Example: Comment on large files
   def check_file_sizes():
       pr = get_pr_details(PR_NUMBER)
       for file in pr['files']:
           if file['changes'] > 500:
               comment = f"⚠️ File `{file['filename']}` has {file['changes']} changes. Consider splitting it."
               add_review_comment(comment)

**Custom Review Rules**

.. code-block:: yaml

   # Custom review rules
   review_rules:
     - name: Check for TODO comments
       pattern: "TODO"
       message: "Please resolve TODO comments before merging"

     - name: Check for console.log
       pattern: "console\\.log"
       message: "Remove console.log statements before merging"

     - name: Check for hardcoded secrets
       pattern: "(api_key|secret|password)\\s*=\\s*['\"]"
       message: "Hardcoded secrets detected. Use environment variables."

Integration with IDEs
---------------------

**VS Code Extensions**

.. code-block:: json

   // .vscode/extensions.json
   {
     "recommendations": [
       "github.vscode-pull-request-github",
       "eamodio.gitlens",
       "ms-python.python",
       "dbaeumer.vscode-eslint"
     ]
   }

**IntelliJ IDEA Plugins**

- GitToolBox
- CodeGlance
- SonarLint
- CheckStyle-IDEA

**JetBrains Rider Plugins**

- GitToolBox
- CodeGlance
- SonarLint

Best Practices
--------------

**Automate Routine Checks**

.. code-block:: yaml

   # Automated checks
   automated_checks:
     - linting
     - formatting
     - security scanning
     - dependency checks
     - test coverage

**Set Up Pre-commit Hooks**

.. code-block:: yaml

   # .pre-commit-config.yaml
   repos:
     - repo: https://github.com/pre-commit/pre-commit-hooks
       rev: v4.0.1
       hooks:
         - id: trailing-whitespace
         - id: end-of-file-fixer
         - id: check-yaml
         - id: check-added-large-files

     - repo: https://github.com/psf/black
       rev: 22.1.0
       hooks:
         - id: black

**Use Review Templates**

.. code-block:: text

   # Pull Request Template
   ## Description
   <!-- Describe the changes -->

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   <!-- Describe how you tested -->

   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] No new warnings

**Track Review Metrics**

.. code-block:: python

   # Track review metrics
   class ReviewMetrics:
       def __init__(self):
           self.review_count = 0
           self.review_time = []
           self.approval_rate = 0

       def record_review(self, time_taken, approved):
           self.review_count += 1
           self.review_time.append(time_taken)
           if approved:
               self.approval_rate += 1

       def get_average_review_time(self):
           return sum(self.review_time) / len(self.review_time)

       def get_approval_rate(self):
           return (self.approval_rate / self.review_count) * 100

Common Pitfalls
---------------

**Over-Automation**

Don't automate everything. Some things need human judgment.

**Ignoring Context**

Automated tools don't understand context like humans do.

**False Positives**

Automated tools can flag issues that aren't real problems.

**Tool Fatigue**

Too many tools can overwhelm reviewers.

**No Human Review**

Automation should supplement, not replace, human review.