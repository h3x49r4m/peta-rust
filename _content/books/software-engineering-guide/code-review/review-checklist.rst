---
title: "Review Checklist"
date: 2026-02-07T00:00:00
tags: ["code-review", "checklist", "quality"]
description: "Comprehensive code review checklist"
---

Review Checklist
================

A comprehensive checklist for effective code reviews.

Functionality
-------------

**Correctness**

Does the code work as intended?

.. code-block:: text

   ✓ Logic is correct
   ✓ Edge cases are handled
   ✓ Error conditions are managed
   ✓ No obvious bugs
   ✓ Matches requirements

**Example**

.. code-block:: python

   # Before review
   def calculate_average(numbers):
       return sum(numbers) / len(numbers)  # Fails for empty list

   # After review
   def calculate_average(numbers):
       if not numbers:
           return 0
       return sum(numbers) / len(numbers)

**Completeness**

Is the implementation complete?

.. code-block:: text

   ✓ All requirements are met
   ✓ No TODO comments left
   ✓ Error handling is comprehensive
   ✓ Logging is appropriate
   ✓ Documentation is complete

**Example**

.. code-block:: python

   # Before review
   def process_payment(amount):
       # TODO: Add error handling
       payment_gateway.charge(amount)

   # After review
   def process_payment(amount):
       try:
           payment_gateway.charge(amount)
           logger.info(f"Payment of {amount} processed")
       except PaymentError as e:
           logger.error(f"Payment failed: {e}")
           raise

Code Quality
------------

**Readability**

Is the code easy to understand?

.. code-block:: text

   ✓ Clear variable names
   ✓ Descriptive function names
   ✓ Appropriate comments
   ✓ Logical structure
   ✓ Consistent style

**Example**

.. code-block:: python

   # Before review
   def calc(d):
       r = 0
       for i in d:
           r += i
       return r

   # After review
   def calculate_total(data):
       total = 0
       for item in data:
           total += item
       return total

**Simplicity**

Is the code unnecessarily complex?

.. code-block:: text

   ✓ No over-engineering
   ✓ Simple solutions preferred
   ✓ Clear and direct
   ✓ Easy to maintain
   ✓ No unnecessary abstractions

**Example**

.. code-block:: python

   # Before review
   def process_data(data):
       result = []
       for item in data:
           if item is not None:
               if isinstance(item, int):
                   if item > 0:
                       result.append(item)
       return result

   # After review
   def process_data(data):
       return [item for item in data if item and isinstance(item, int) and item > 0]

**Consistency**

Does the code follow project conventions?

.. code-block:: text

   ✓ Follows style guide
   ✓ Consistent naming
   ✓ Consistent formatting
   ✓ Uses existing patterns
   ✓ Follows team conventions

**Example**

.. code-block:: python

   # Inconsistent naming
   def getUserData(user_id):
       pass

   def create_user(userId):
       pass

   # Consistent naming
   def get_user_data(user_id):
       pass

   def create_user(user_id):
       pass

Testing
-------

**Test Coverage**

Are there sufficient tests?

.. code-block:: text

   ✓ Unit tests included
   ✓ Integration tests included
   ✓ Edge cases tested
   ✓ Error conditions tested
   ✓ High coverage percentage

**Example**

.. code-block:: python

   # Test coverage
   def test_calculate_average():
       # Happy path
       assert calculate_average([1, 2, 3]) == 2

       # Edge case: empty list
       assert calculate_average([]) == 0

       # Edge case: single item
       assert calculate_average([5]) == 5

       # Edge case: negative numbers
       assert calculate_average([-1, 1]) == 0

**Test Quality**

Are the tests good quality?

.. code-block:: text

   ✓ Tests are independent
   ✓ Tests are fast
   ✓ Tests are maintainable
   ✓ Tests are descriptive
   ✓ Tests are reliable

**Example**

.. code-block:: python

   # Good test
   def test_user_registration_with_valid_data_creates_user():
       user = register_user("john@example.com", "password123")
       assert user.id is not None
       assert user.email == "john@example.com"

   # Bad test
   def test_registration():
       # What does this test?
       assert register_user("john@example.com", "password123")

Documentation
-------------

**Code Comments**

Are comments helpful and accurate?

.. code-block:: text

   ✓ Comments explain why, not what
   ✓ No redundant comments
   ✓ No outdated comments
   ✓ Comments are clear
   ✓ No commented-out code

**Example**

.. code-block:: python

   # Bad comment
   # Increment counter
   counter += 1

   # Good comment
   # Counter tracks the number of retry attempts
   counter += 1

**API Documentation**

Is the API well documented?

.. code-block:: text

   ✓ Function documentation
   ✓ Parameter descriptions
   ✓ Return value documentation
   ✓ Exception documentation
   ✓ Usage examples

**Example**

.. code-block:: python

   def divide(a, b):
       """
       Divide two numbers.

       Args:
           a (float): The dividend
           b (float): The divisor

       Returns:
           float: The result of division

       Raises:
           ValueError: If b is zero

       Example:
           >>> divide(10, 2)
           5.0
       """
       if b == 0:
           raise ValueError("Cannot divide by zero")
       return a / b

Security
--------

**Input Validation**

Are inputs properly validated?

.. code-block:: text

   ✓ User input is validated
   ✓ SQL injection prevention
   ✓ XSS prevention
   ✓ CSRF protection
   ✓ File upload validation

**Example**

.. code-block:: python

   # Before review
   def get_user(user_id):
       query = f"SELECT * FROM users WHERE id = {user_id}"
       return db.execute(query)

   # After review
   def get_user(user_id):
       if not isinstance(user_id, int) or user_id <= 0:
           raise ValueError("Invalid user ID")
       query = "SELECT * FROM users WHERE id = ?"
       return db.execute(query, user_id)

**Authentication & Authorization**

Are security checks in place?

.. code-block:: text

   ✓ Authentication implemented
   ✓ Authorization checked
   ✓ Session management
   ✓ Password hashing
   ✓ Sensitive data protection

**Example**

.. code-block:: python

   # Authentication check
   @login_required
   def get_user_profile(user_id):
       current_user = get_current_user()

       # Authorization check
       if current_user.id != user_id and not current_user.is_admin:
           raise PermissionError("Access denied")

       return user_service.get_profile(user_id)

Performance
-----------

**Efficiency**

Is the code efficient?

.. code-block:: text

   ✓ Appropriate algorithms
   ✓ No unnecessary loops
   ✓ Efficient data structures
   ✓ Caching where appropriate
   ✓ Database optimization

**Example**

.. code-block:: python

   # Before review
   def find_user(users, user_id):
       for user in users:
           if user.id == user_id:
               return user

   # After review
   def find_user(users, user_id):
       # Using dictionary for O(1) lookup
       return users_by_id.get(user_id)

**Scalability**

Will the code scale?

.. code-block:: text

   ✓ Handles large datasets
   ✓ No memory leaks
   ✓ Efficient queries
   ✓ Pagination implemented
   ✓ Async where appropriate

**Example**

.. code-block:: python

   # Before review
   def get_all_users():
       return db.query("SELECT * FROM users")

   # After review
   def get_users(page=1, per_page=50):
       offset = (page - 1) * per_page
       return db.query(
           "SELECT * FROM users LIMIT ? OFFSET ?",
           per_page, offset
       )

Error Handling
--------------

**Error Management**

Are errors handled properly?

.. code-block:: text

   ✓ Try-except blocks used
   ✓ Specific exceptions caught
   ✓ Error messages are clear
   ✓ Logging errors
   ✓ Graceful degradation

**Example**

.. code-block:: python

   # Before review
   def process_file(filename):
       with open(filename) as f:
           data = f.read()
       return parse_data(data)

   # After review
   def process_file(filename):
       try:
           with open(filename) as f:
               data = f.read()
           return parse_data(data)
       except FileNotFoundError:
           logger.error(f"File not found: {filename}")
           raise
       except IOError as e:
           logger.error(f"Error reading file: {e}")
           raise

**Exception Propagation**

Are exceptions handled at appropriate levels?

.. code-block:: text

   ✓ Low-level exceptions caught
   ✓ High-level exceptions raised
   ✓ Exception chaining
   ✓ No silent failures
   ✓ Context preserved

**Example**

.. code-block:: python

   # Before review
   def process_order(order):
       try:
           validate_order(order)
           charge_payment(order)
       except Exception:
           pass  # Silent failure

   # After review
   def process_order(order):
       try:
           validate_order(order)
           charge_payment(order)
       except ValidationError as e:
           logger.warning(f"Validation failed: {e}")
           raise
       except PaymentError as e:
           logger.error(f"Payment failed: {e}")
           raise

Maintainability
---------------

**Modularity**

Is the code modular?

.. code-block:: text

   ✓ Single responsibility
   ✓ Small functions
   ✓ Low coupling
   ✓ High cohesion
   ✓ Clear interfaces

**Example**

.. code-block:: python

   # Before review
   def process_order(order):
       # Validate
       if not order.items:
           raise ValueError("Order has no items")
       if not order.user:
           raise ValueError("Order has no user")

       # Calculate total
       total = sum(item.price for item in order.items)

       # Save
       db.save(order)

       # Send email
       email_service.send(order.user.email, "Order confirmed")

       return order

   # After review
   def process_order(order):
       validate_order(order)
       total = calculate_order_total(order)
       save_order(order)
       send_order_confirmation(order)
       return order

**Extensibility**

Is the code easy to extend?

.. code-block:: text

   ✓ Open for extension
   ✓ Closed for modification
   ✓ Plugin support
   ✓ Configuration driven
   ✓ Hook points

**Example**

.. code-block:: python

   # Extensible design
   class PaymentProcessor:
       def __init__(self):
           self.strategies = {}

       def register_strategy(self, payment_type, strategy):
           self.strategies[payment_type] = strategy

       def process_payment(self, payment_type, amount):
           strategy = self.strategies.get(payment_type)
           if not strategy:
               raise ValueError(f"Unknown payment type: {payment_type}")
           return strategy.process(amount)

Final Checklist
---------------

Before approving a code review, ensure:

.. code-block:: text

   ✓ All review comments addressed
   ✓ Tests passing
   ✓ Documentation updated
   ✓ No blocking issues
   ✓ Code quality standards met
   ✓ Security concerns addressed
   ✓ Performance acceptable
   ✓ Ready to merge