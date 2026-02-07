---
title: "Unit Testing"
date: 2026-02-07T00:00:00
tags: ["testing", "unit-testing", "tdd"]
description: "Writing effective unit tests"
---

Unit Testing
============

Unit tests verify the functionality of a specific section of code, usually at the function or method level. They are the foundation of a solid testing strategy.

What is a Unit Test?
--------------------

A unit test is a piece of code that:

- Tests a specific piece of functionality
- Is isolated from external dependencies
- Runs quickly
- Is deterministic (same input produces same output)
- Has a clear pass/fail result

Anatomy of a Unit Test
----------------------

A good unit test follows the Arrange-Act-Assert (AAA) pattern:

.. code-block:: python

   def test_calculate_total_price():
       # Arrange - set up the test data
       cart = ShoppingCart()
       cart.add_item("Apple", 1.50)
       cart.add_item("Banana", 0.80)

       # Act - execute the code being tested
       total = cart.calculate_total()

       # Assert - verify the result
       assert total == 2.30

Writing Testable Code
---------------------

To write unit tests, your code must be testable. This means:

- **Dependency Injection**: Pass dependencies as parameters rather than creating them internally
- **Pure Functions**: Functions that don't have side effects are easier to test
- **Single Responsibility**: Each function should do one thing

.. code-block:: python

   # Hard to test
   class OrderProcessor:
       def process_order(self, order):
           db = Database()  # Hard dependency
           db.save(order)
           email_service = EmailService()  # Hard dependency
           email_service.send_confirmation(order)

   # Easy to test
   class OrderProcessor:
       def __init__(self, database, email_service):
           self.database = database
           self.email_service = email_service

       def process_order(self, order):
           self.database.save(order)
           self.email_service.send_confirmation(order)

Test Coverage
-------------

Test coverage measures how much of your code is executed by tests. However, 100% coverage doesn't mean your code is bug-free.

**What to Test:**

- Happy path (normal operation)
- Edge cases (boundary conditions)
- Error conditions
- Null/empty inputs

.. code-block:: python

   def test_string_length():
       # Happy path
       assert get_string_length("hello") == 5

       # Edge case - empty string
       assert get_string_length("") == 0

       # Edge case - single character
       assert get_string_length("a") == 1

       # Error condition - None input
       with pytest.raises(ValueError):
           get_string_length(None)

Mocking and Stubbing
--------------------

Mocking and stubbing allow you to isolate the code being tested from its dependencies.

**Stubs** provide predefined responses to function calls.

**Mocks** verify that certain functions were called with specific arguments.

.. code-block:: python

   from unittest.mock import Mock, patch

   def test_send_email():
       # Create a mock email service
       email_service = Mock()

       # Use the mock in the test
       sender = NotificationSender(email_service)
       sender.send_notification("user@example.com", "Hello")

       # Verify the email was sent
       email_service.send.assert_called_once_with(
           "user@example.com",
           "Hello"
       )

   def test_get_user_data():
       # Patch the database connection
       with patch('app.db.connection') as mock_db:
           mock_db.execute.return_value = [{"id": 1, "name": "John"}]

           user = get_user_data(1)

           assert user.name == "John"
           mock_db.execute.assert_called_once()

Best Practices
--------------

**Test Names**

Test names should be descriptive and follow a consistent convention:

.. code-block:: python

   # Good
   def test_calculate_total_with_discount_returns_correct_amount():
       pass

   # Bad
   def test_calc():
       pass

**One Assertion Per Test**

Keep tests focused. Each test should verify one thing.

.. code-block:: python

   # Good
   def test_calculate_total_returns_correct_amount():
       assert calculate_total([1, 2, 3]) == 6

   def test_calculate_total_with_empty_list_returns_zero():
       assert calculate_total([]) == 0

   # Bad - testing multiple things
   def test_calculate_total():
       assert calculate_total([1, 2, 3]) == 6
       assert calculate_total([]) == 0
       assert calculate_total([0]) == 0

**Independent Tests**

Tests should not depend on each other. Each test should be able to run in isolation.

**Fast Tests**

Unit tests should be fast. If they're slow, developers won't run them regularly.

Common Pitfalls
---------------

**Testing Implementation Details**

Don't test how the code works, test what it does.

.. code-block:: python

   # Bad - testing implementation
   def test_sort_users():
       users = sort_users(user_list)
       assert users[0] == expected_first_user
       assert users[1] == expected_second_user

   # Good - testing behavior
   def test_sort_users_returns_correct_order():
       users = sort_users(user_list)
       assert users == expected_sorted_list

**Overusing Mocks**

Don't mock everything. Only mock external dependencies.

**Testing Private Methods**

Private methods are implementation details. Test the public interface instead.