---
title: "Review Guidelines"
date: 2026-02-07T00:00:00
tags: ["code-review", "guidelines", "best-practices"]
description: "Code review guidelines and principles"
---

Review Guidelines
=================

Code review is the systematic examination of source code intended to find mistakes overlooked in the initial development phase, improving the overall quality of software.

Purpose of Code Review
----------------------

**Find Bugs and Vulnerabilities**

Catch errors before they reach production.

.. code-block:: python

   # Before review
   def divide(a, b):
       return a / b  # Potential division by zero

   # After review
   def divide(a, b):
       if b == 0:
           raise ValueError("Cannot divide by zero")
       return a / b

**Ensure Code Quality**

Maintain high standards across the codebase.

.. code-block:: python

   # Before review
   def process_data(d):
       r = []
       for i in d:
           if i > 0:
               r.append(i * 2)
       return r

   # After review
   def process_positive_data(data):
       return [item * 2 for item in data if item > 0]

**Share Knowledge**

Help team members learn from each other.

.. code-block:: python

   # Reviewer comment
   # Great use of list comprehension! This is more Pythonic
   # than the traditional for loop approach.

**Improve Maintainability**

Make code easier to understand and modify.

.. code-block:: python

   # Before review
   def get_user(u):
       return db.query("SELECT * FROM users WHERE id = ?", u)

   # After review
   def get_user_by_id(user_id):
       query = "SELECT * FROM users WHERE id = ?"
       return db.query(query, user_id)

When to Review
--------------

**Before Merging to Main Branch**

All code should be reviewed before merging.

.. code-block:: bash

   # Git flow with code review
   git checkout -b feature/new-feature
   # Make changes
   git add .
   git commit -m "Add new feature"
   git push origin feature/new-feature
   # Create pull request for review

**For Critical Security Changes**

Security-related changes require extra scrutiny.

.. code-block:: python

   # Security change requiring review
   def authenticate_user(username, password):
       # Review needed: password hashing implementation
       hashed_password = hash_password(password)
       user = db.get_user(username)
       if user and verify_hash(hashed_password, user.password_hash):
           return user

**When Introducing New Features**

New features should be reviewed for design and implementation.

.. code-block:: python

   # New feature requiring review
   class PaymentProcessor:
       def process_payment(self, amount, payment_method):
           # Review needed: payment processing logic
           pass

**For Refactoring Efforts**

Refactoring should be reviewed to ensure correctness.

.. code-block:: python

   # Refactoring requiring review
   def calculate_total(order):
       # Review needed: refactored calculation logic
       items = order.get_items()
       discounts = order.get_discounts()
       return sum(item.price for item in items) - sum(discounts)

Review Process
--------------

**Initial Review**

First pass to understand the change.

.. code-block:: text

   Review Checklist for Initial Review:
   ✓ Understand the purpose of the change
   ✓ Verify the change addresses the issue
   ✓ Check for obvious bugs
   ✓ Verify tests are included
   ✓ Review documentation updates

**Detailed Review**

Deep dive into code quality and implementation.

.. code-block:: text

   Review Checklist for Detailed Review:
   ✓ Code style and formatting
   ✓ Variable and function naming
   ✓ Error handling
   ✓ Performance considerations
   ✓ Security vulnerabilities
   ✓ Test coverage

**Final Approval**

Confirm all issues are addressed.

.. code-block:: text

   Review Checklist for Final Approval:
   ✓ All review comments addressed
   ✓ Tests passing
   ✓ Documentation updated
   ✓ No blocking issues
   ✓ Ready to merge

Review Etiquette
----------------

**Be Constructive**

Focus on improvement, not criticism.

.. code-block:: text

   # Good: Constructive feedback
   "Consider using a more descriptive variable name here
    to improve readability. 'user_data' might be clearer."

   # Bad: Critical feedback
   "This variable name is terrible. Rename it."

**Be Specific**

Provide clear, actionable feedback.

.. code-block:: text

   # Good: Specific feedback
   "Line 45: The SQL query is vulnerable to SQL injection.
    Use parameterized queries instead."

   # Bad: Vague feedback
   "Fix the SQL query."

**Be Respectful**

Treat reviewers and authors with respect.

.. code-block:: text

   # Good: Respectful feedback
   "I noticed this approach might have performance implications
    for large datasets. Have you considered using an index?"

   # Bad: Disrespectful feedback
   "This code is slow. You should know better."

**Acknowledge Good Work**

Recognize when code is well-written.

.. code-block:: text

   # Good: Positive feedback
   "Great job on the test coverage! The edge cases are
    well handled."

**Ask Questions**

When unsure, ask clarifying questions.

.. code-block:: text

   # Good: Question-based feedback
   "I'm curious about the choice of this algorithm.
    Was there a specific reason for using it over
    the alternative approach?"

Timeframe for Reviews
---------------------

**Respond Quickly**

Aim to review within 24-48 hours.

.. code-block:: text

   Review Response Times:
   - Critical fixes: Within 4 hours
   - High priority: Within 24 hours
   - Normal priority: Within 48 hours
   - Low priority: Within 1 week

**Provide Estimates**

If you can't review quickly, provide an estimate.

.. code-block:: text

   # Example response
   "I won't be able to review this until Thursday.
    Can someone else review it in the meantime?"

**Batch Reviews**

Review multiple changes together when appropriate.

.. code-block:: text

   # Good: Batch related changes
   "I'll review these three related PRs together
    since they're all part of the same feature."

Review Tools
------------

**GitHub Pull Requests**

.. code-block:: text

   GitHub Review Features:
   - Line-by-line comments
   - File-level comments
   - Approvals and request changes
   - Review assignments
   - Status checks

**GitLab Merge Requests**

.. code-block:: text

   GitLab Review Features:
   - Inline comments
   - Discussion threads
   - Approvals
   - Merge request assignments
   - CI/CD integration

**Code Review Platforms**

.. code-block:: text

   Specialized Platforms:
   - Review Board: Powerful code review tool
   - Phabricator: Open-source review platform
   - Crucible: Enterprise code review

Best Practices
--------------

**Keep Reviews Focused**

Review one change at a time.

.. code-block:: text

   # Good: Focused review
   "This PR addresses the login bug. The fix looks
    good and tests are comprehensive."

   # Bad: Unfocused review
   "While reviewing this login fix, I noticed
    several other issues in the codebase."

**Use Templates**

Standardize review comments with templates.

.. code-block:: text

   Review Comment Template:
   [Type: Bug/Improvement/Question]
   [Severity: Critical/Major/Minor]
   [Location: File:Line]

   Description:
   [Clear description of the issue]

   Suggestion:
   [Specific suggestion for improvement]

**Follow Up**

Ensure review comments are addressed.

.. code-block:: text

   # Good: Follow-up
   "I noticed my previous comment about SQL injection
    wasn't addressed. Please update the code to use
    parameterized queries."

**Learn from Reviews**

Use reviews as learning opportunities.

.. code-block:: text

   # Learning from reviews
   "Thanks for pointing out the more Pythonic approach.
   I'll use list comprehensions in future code."

Common Pitfalls
---------------

**Reviewing Too Quickly**

Rushed reviews miss important issues.

**Being Too Critical**

Excessive criticism discourages contributors.

**Ignoring Context**

Reviewing code without understanding the context.

**Delaying Reviews**

Slow reviews block progress.

**Not Providing Guidance**

Pointing out problems without suggesting solutions.