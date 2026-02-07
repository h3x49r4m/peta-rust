---
title: "Test-Driven Development"
date: 2026-02-07T00:00:00
tags: ["testing", "tdd", "methodology"]
description: "Test-Driven Development methodology and practices"
---

Test-Driven Development (TDD)
==============================

Test-Driven Development is a software development process where tests are written before the implementation. It emphasizes writing clean, working, and maintainable code.

The TDD Cycle
-------------

TDD follows a simple iterative cycle:

1. **Red**: Write a failing test
2. **Green**: Write the minimum code to make the test pass
3. **Refactor**: Improve the code while keeping tests passing

.. code-block:: python

   # Step 1: Write a failing test (RED)
   def test_calculate_area_of_circle():
       assert calculate_area_of_circle(5) == 78.54

   # This will fail because the function doesn't exist yet

   # Step 2: Write minimum code to pass (GREEN)
   def calculate_area_of_circle(radius):
       return 3.14 * radius * radius

   # Now the test passes, but the code might need improvement

   # Step 3: Refactor (REFACTOR)
   import math

   def calculate_area_of_circle(radius):
       return math.pi * radius * radius

   # Tests still pass, and code is better

Benefits of TDD
---------------

**Better Design**

Writing tests first forces you to think about the design before implementation.

**Confidence**

With comprehensive tests, you can refactor without fear of breaking things.

**Living Documentation**

Tests serve as documentation of how your code should behave.

**Fewer Bugs**

Catching bugs early in the development cycle saves time and effort.

**Debugging is Easier**

When a test fails, you know exactly what broke.

Writing Good TDD Tests
-----------------------

**Start Simple**

Don't try to write complex tests initially. Start with simple cases and build complexity.

.. code-block:: python

   # Start with the simplest case
   def test_calculate_total_with_empty_list():
       assert calculate_total([]) == 0

   # Then add more complexity
   def test_calculate_total_with_single_item():
       assert calculate_total([10]) == 10

   def test_calculate_total_with_multiple_items():
       assert calculate_total([10, 20, 30]) == 60

**Write Descriptive Test Names**

Test names should describe the behavior being tested.

.. code-block:: python

   # Good
   def test_calculate_total_with_discount_applied():
       pass

   # Bad
   def test_calc_tot():
       pass

**One Test at a Time**

Focus on one test at a time. Don't write multiple tests before implementing.

**Keep Tests Small**

Each test should focus on a single behavior.

TDD in Practice
---------------

Example: Building a String Calculator
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   # Test 1: Empty string returns 0
   def test_add_empty_string():
       assert add("") == 0

   # Implementation
   def add(numbers):
       if not numbers:
           return 0
       return int(numbers)

   # Test 2: Single number returns the number
   def test_add_single_number():
       assert add("1") == 1

   # Implementation
   def add(numbers):
       if not numbers:
           return 0
       return int(numbers)

   # Test 3: Two numbers separated by comma
   def test_add_two_numbers():
       assert add("1,2") == 3

   # Implementation
   def add(numbers):
       if not numbers:
           return 0
       nums = numbers.split(",")
       return sum(int(n) for n in nums)

   # Test 4: Multiple numbers
   def test_add_multiple_numbers():
       assert add("1,2,3,4") == 10

   # Implementation already handles this

   # Test 5: New line as separator
   def test_add_with_newline_separator():
       assert add("1\n2,3") == 6

   # Implementation
   def add(numbers):
       if not numbers:
           return 0
       numbers = numbers.replace("\n", ",")
       nums = numbers.split(",")
       return sum(int(n) for n in nums)

Advanced TDD Techniques
------------------------

**Mocking in TDD**

Use mocks to isolate dependencies during TDD.

.. code-block:: python

   from unittest.mock import Mock

   # Write test first
   def test_send_welcome_email():
       email_service = Mock()
       user = User(name="John", email="john@example.com")

       send_welcome_email(user, email_service)

       email_service.send.assert_called_once_with(
           "john@example.com",
           "Welcome, John!"
       )

   # Implement after test
   def send_welcome_email(user, email_service):
       email_service.send(
           user.email,
           f"Welcome, {user.name}!"
       )

**Test Doubles**

Use test doubles (stubs, mocks, fakes) to simulate dependencies.

**Property-Based Testing**

Test properties of your functions rather than specific examples.

.. code-block:: python

   from hypothesis import given, strategies as st

   @given(st.integers(), st.integers())
   def test_addition_is_commutative(a, b):
       assert add(a, b) == add(b, a)

   @given(st.integers())
   def test_addition_with_zero_returns_original(x):
       assert add(x, 0) == x

Common TDD Misconceptions
--------------------------

**"TDD is too slow"**

TDD can actually be faster because you spend less time debugging.

**"TDD is only for unit tests"**

TDD can be applied at all levels of testing.

**"TDD requires 100% test coverage"**

TDD doesn't require 100% coverage, but it does encourage comprehensive testing.

**"TDD is only for greenfield projects"**

TDD can be applied to legacy code as well, though it may require refactoring first.

When TDD May Not Be Appropriate
--------------------------------

- Exploratory coding or prototyping
- When requirements are unclear
- When working with legacy code that has no tests
- When time is extremely constrained and risks are low

TDD Best Practices
------------------

**Run Tests Frequently**

Run your tests after each change to catch issues early.

**Keep the Cycle Fast**

If tests are slow, developers won't run them frequently.

**Refactor Regularly**

Don't skip the refactor step. It's essential for maintaining code quality.

**Pair with Pair Programming**

TDD works exceptionally well with pair programming.

**Use Continuous Integration**

Automate running your tests in CI/CD pipelines.