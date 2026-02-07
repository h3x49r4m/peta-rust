---
title: "Comments"
date: 2026-02-07T00:00:00
tags: ["clean-code", "comments", "documentation"]
description: "When and how to use comments effectively"
---

Comments
========

Comments are not like Schindler's List. They are not "pure good." In fact, comments are, at best, a necessary evil.

What Comments Should Explain
----------------------------

Comments should explain **why** something is done, not **what** is done. The code itself should be self-explanatory.

.. code-block:: python

   # Bad - explains what
   # Loop through all users and check if they are active
   for user in users:
       if user.is_active:
           send_email(user)

   # Good - explains why
   # Send emails only to active users to avoid bouncing and maintain sender reputation
   for user in users:
       if user.is_active:
           send_email(user)

Informative Comments
--------------------

Sometimes it is useful to provide basic information in a comment.

.. code-block:: python

   # The pattern used here matches the format: YYYY-MM-DD
   date_pattern = r'\d{4}-\d{2}-\d{2}'

   # Using a timeout of 30 seconds to prevent hanging on slow connections
   response = requests.get(url, timeout=30)

Explanation of Intent
---------------------

Sometimes a comment goes beyond just useful information about the implementation and provides the intent behind a decision.

.. code-block:: python

   # Using a simple array instead of a Set for performance reasons
   # since we only need to store a small number of items
   # and the overhead of Set is not justified
   allowed_roles = ['admin', 'editor', 'viewer']

Warning of Consequences
-----------------------

Sometimes it is useful to warn other programmers about certain consequences.

.. code-block:: python

   # WARNING: This function modifies the original list in place
   # Make a copy if you need to preserve the original data
   def sort_and_deduplicate(items):
       items.sort()
       del items[1:]  # Keep only first occurrence
       return items

TODO Comments
-------------

TODOs are jobs that the programmer thinks should be done, but for some reason can't be done at the moment.

.. code-block:: python

   # TODO: Refactor this to use the new user service API
   # TODO: Add error handling for network failures
   # TODO: Consider using async/await for better performance

When Not to Comment
-------------------

**Redundant Comments**

Comments that add nothing to the understanding of the code are worse than useless.

.. code-block:: python

   # Bad
   # Check if user is logged in
   if user.is_logged_in:
       # Redirect to dashboard
       return redirect('dashboard')

**Misleading Comments**

Comments that are inaccurate or out of date are actively harmful.

.. code-block:: python

   # Bad - comment is outdated
   # Returns the user's age as an integer
   def get_user_age(user):
       return user.birthdate  # Actually returns the birthdate, not age

**Mandated Comments**

Comments that are written only because someone told you to write them are worse than useless.

.. code-block:: python

   # Bad
   # Author: John Doe
   # Date: 2026-02-07
   # Description: This function validates user input
   def validate_input(input_data):
       pass

**Commented-Out Code**

Don't comment out code. Delete it. The version control system has your back.

.. code-block:: python

   # Bad
   # def old_function():
   #     # Old implementation
   #     pass

   def new_function():
       # New implementation
       pass

Comment Formatting
------------------

**Block Comments**

Block comments should be at the same indentation level as the code they describe.

.. code-block:: python

   def calculate_total(prices):
       """
       Calculate the total price from a list of prices.

       Args:
           prices: List of price values

       Returns:
           The sum of all prices
       """
       return sum(prices)

**Inline Comments**

Inline comments should be rare and only used when they add significant value.

.. code-block:: python

   def process_data(data):
       result = []
       for item in data:
           # Skip items with invalid format
           if not validate_format(item):
               continue
           result.append(transform(item))
       return result