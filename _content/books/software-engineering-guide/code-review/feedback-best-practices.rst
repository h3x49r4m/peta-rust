---
title: "Feedback Best Practices"
date: 2026-02-07T00:00:00
tags: ["code-review", "feedback", "communication"]
description: "Best practices for giving and receiving feedback"
---

Feedback Best Practices
=======================

Effective feedback is crucial for productive code reviews. It should be constructive, specific, and actionable.

Giving Feedback
---------------

**Be Constructive**

Focus on improvement rather than criticism.

.. code-block:: text

   # Good: Constructive feedback
   "Consider using a more descriptive variable name here.
    'user_data' might be clearer than 'd'."

   # Bad: Critical feedback
   "This variable name is terrible. Rename it."

**Be Specific**

Provide clear, actionable feedback.

.. code-block:: text

   # Good: Specific feedback
   "Line 45: The SQL query is vulnerable to SQL injection.
    Use parameterized queries instead of this string formatting."

   # Bad: Vague feedback
   "Fix the SQL query."

**Provide Context**

Explain why the change is needed.

.. code-block:: text

   # Good: Contextual feedback
   "Using a dictionary comprehension here would be more Pythonic
    and faster than the for loop approach. It also reduces the
    code from 5 lines to 1."

   # Bad: Feedback without context
   "Use a dictionary comprehension instead."

**Suggest Solutions**

Don't just point out problems; suggest solutions.

.. code-block:: text

   # Good: Feedback with solution
   "This function is doing too many things. Consider splitting it into:
   1. validate_order() - validates the order data
   2. calculate_total() - calculates the order total
   3. save_order() - saves the order to the database"

   # Bad: Feedback without solution
   "This function is too long. Refactor it."

**Prioritize Issues**

Highlight critical issues over minor ones.

.. code-block:: text

   # Good: Prioritized feedback
   "Critical: Line 23 has a SQL injection vulnerability.
   Major: Missing error handling for network requests.
   Minor: Variable naming could be more descriptive."

   # Bad: Unprioritized feedback
   "Line 23: SQL injection.
   Line 45: Variable name.
   Line 67: Error handling."

**Use Positive Language**

Frame feedback positively when possible.

.. code-block:: text

   # Good: Positive language
   "Great job on the test coverage! To make it even better,
    consider adding edge case tests for empty inputs."

   # Bad: Negative language
   "You're missing edge case tests. Add them."

Receiving Feedback
------------------

**Stay Open-Minded**

View feedback as an opportunity to learn.

.. code-block:: text

   # Good: Open-minded response
   "Thanks for pointing out the SQL injection vulnerability.
   I wasn't aware of that approach. I'll update the code to use
   parameterized queries."

   # Bad: Defensive response
   "The code works fine as is. It's not a big deal."

**Ask for Clarification**

When feedback is unclear, ask questions.

.. code-block:: text

   # Good: Seeking clarification
   "I understand that this function is too long, but I'm not sure
   how to split it. Can you suggest where to draw the lines?"

   # Bad: Making assumptions
   "I'll split it in half."

**Consider the Feedback**

Give feedback serious consideration before dismissing.

.. code-block:: text

   # Good: Considering feedback
   "I see your point about using a dictionary comprehension.
   However, I chose the for loop approach because it's more
   readable for beginners. What do you think?"

   # Bad: Dismissing feedback
   "I don't agree with that suggestion."

**Respond Promptly**

Acknowledge feedback in a timely manner.

.. code-block:: text

   # Good: Prompt response
   "Thanks for the review! I'll address these comments
   and push an update shortly."

   # Bad: Delayed response
   [No response for several days]

**Explain When Disagreeing**

If you disagree, explain your reasoning.

.. code-block:: text

   # Good: Explained disagreement
   "I understand your concern about performance, but I chose
   this approach because it's more maintainable. The performance
   difference is negligible for our data size."

   # Bad: Unexplained disagreement
   "I'm going to keep it as is."

Feedback Templates
------------------

**Bug Report Template**

.. code-block:: text

   [Type: Bug]
   [Severity: Critical/Major/Minor]
   [Location: File:Line]

   Issue:
   [Description of the bug]

   Impact:
   [What happens because of this bug]

   Suggestion:
   [Specific suggestion for fixing it]

**Code Style Template**

.. code-block:: text

   [Type: Style]
   [Severity: Minor]
   [Location: File:Line]

   Issue:
   [Style guideline that's not followed]

   Reason:
   [Why this style is preferred]

   Suggestion:
   [Specific suggestion for improvement]

**Performance Template**

.. code-block:: text

   [Type: Performance]
   [Severity: Major]
   [Location: File:Line]

   Current Approach:
   [Description of current implementation]

   Issue:
   [Performance concern]

   Suggestion:
   [Alternative approach with better performance]

   Impact:
   [Expected performance improvement]

Common Feedback Scenarios
-------------------------

**Refactoring Suggestions**

.. code-block:: text

   # Good feedback
   "This function handles both user creation and email sending.
   Consider separating these concerns:

   ```
   def create_user(user_data):
       user = db.create(user_data)
       return user

   def send_welcome_email(user):
       email_service.send(user.email, 'Welcome!')
   ```

   This makes the code more testable and maintainable."

**Naming Suggestions**

.. code-block:: text

   # Good feedback
   "The variable name 'd' is not descriptive. Consider renaming
   it to 'user_data' to clearly indicate what it contains."

**Security Concerns**

.. code-block:: text

   # Good feedback
   "This code is vulnerable to SQL injection. Use parameterized
   queries instead:

   ```
   # Vulnerable
   query = f"SELECT * FROM users WHERE id = {user_id}"

   # Secure
   query = "SELECT * FROM users WHERE id = ?"
   db.execute(query, user_id)
   ```"

**Test Coverage**

.. code-block:: text

   # Good feedback
   "Great job on the unit tests! To improve coverage, consider
   adding tests for:
   - Edge cases (empty inputs, null values)
   - Error conditions
   - Boundary values

   Example:
   ```
   def test_calculate_average_with_empty_list():
       assert calculate_average([]) == 0
   ```"

Handling Disagreements
----------------------

**Focus on Facts**

Base discussions on facts, not opinions.

.. code-block:: text

   # Good: Fact-based discussion
   "According to the style guide, we should use snake_case for
   variable names in Python. This aligns with PEP 8."

   # Bad: Opinion-based discussion
   "I think camelCase looks better."

**Compromise When Appropriate**

Find middle ground when possible.

.. code-block:: text

   # Good: Compromise
   "I understand your concern about readability. How about we use
   a longer variable name that's still descriptive but not overly
   verbose?"

**Escalate When Needed**

Sometimes a third party is needed to resolve disagreements.

.. code-block:: text

   # Good: Escalation
   "We have different opinions on this approach. Let's bring in
   the tech lead to get their perspective."

Feedback Etiquette
------------------

**Be Timely**

Provide feedback promptly.

.. code-block:: text

   Review Response Times:
   - Critical issues: Within 4 hours
   - High priority: Within 24 hours
   - Normal priority: Within 48 hours

**Be Respectful**

Treat others with respect.

.. code-block:: text

   # Good: Respectful
   "I appreciate the effort you put into this. Here are some
   suggestions for improvement."

   # Bad: Disrespectful
   "This code is a mess. Fix it."

**Acknowledge Effort**

Recognize the work put into the change.

.. code-block:: text

   # Good: Acknowledging effort
   "Great job on implementing this feature! The test coverage
   is impressive."

**Learn from Each Other**

Use feedback as a learning opportunity.

.. code-block:: text

   # Learning opportunity
   "I didn't know about this approach. Thanks for teaching me
   something new!"

Best Practices Summary
----------------------

**For Reviewers**

.. code-block:: text

   ✓ Be constructive and specific
   ✓ Provide context and solutions
   ✓ Prioritize issues
   ✓ Use positive language
   ✓ Respond promptly
   ✓ Acknowledge good work

**For Authors**

.. code-block:: text

   ✓ Stay open-minded
   ✓ Ask for clarification
   ✓ Consider feedback seriously
   ✓ Respond promptly
   ✓ Explain when disagreeing
   ✓ Learn from feedback

**For Both**

.. code-block:: text

   ✓ Be respectful
   ✓ Focus on facts
   ✓ Compromise when appropriate
   ✓ Escalate when needed
   ✓ Use templates consistently
   ✓ Keep learning