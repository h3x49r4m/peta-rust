---
title: "Error Handling"
date: 2026-02-07T00:00:00
tags: ["clean-code", "error-handling", "robustness"]
description: "Best practices for handling errors gracefully"
---

Error Handling
==============

Error handling is one of those areas where clean code is especially important. It can get messy quickly if not handled properly.

Use Exceptions Rather Than Return Codes
----------------------------------------

Return codes are error-prone because callers can easily forget to check them.

.. code-block:: python

   # Bad - return codes
   def divide(a, b):
       if b == 0:
           return -1  # Error code
       return a / b

   result = divide(10, 0)
   # Caller might forget to check the return code

   # Good - exceptions
   def divide(a, b):
       if b == 0:
           raise ValueError("Cannot divide by zero")
       return a / b

   try:
       result = divide(10, 0)
   except ValueError as e:
       print(f"Error: {e}")

Provide Context with Exceptions
--------------------------------

When throwing exceptions, provide enough context to understand what went wrong.

.. code-block:: python

   # Bad
   def get_user(user_id):
       user = db.find(user_id)
       if not user:
           raise Exception("User not found")

   # Good
   def get_user(user_id):
       user = db.find(user_id)
       if not user:
           raise ValueError(f"User with ID {user_id} not found in database")

Define Exception Classes by Application
----------------------------------------

Differentiate exceptions by application behavior.

.. code-block:: python

   class ApplicationError(Exception):
       pass

   class UserNotFoundError(ApplicationError):
       pass

   class AuthenticationError(ApplicationError):
       pass

   class PermissionDeniedError(ApplicationError):
       pass

   def get_user(user_id):
       user = db.find(user_id)
       if not user:
           raise UserNotFoundError(f"User {user_id} not found")
       return user

Don't Ignore Exceptions
-----------------------

Ignoring exceptions is a sure way to introduce bugs.

.. code-block:: python

   # Bad
   try:
       risky_operation()
   except:
       pass  # Silent failure

   # Good
   try:
       risky_operation()
   except SpecificError as e:
       logger.error(f"Operation failed: {e}")
       raise  # Re-raise if appropriate

   # Better - handle the error appropriately
   try:
       risky_operation()
   except SpecificError as e:
       logger.error(f"Operation failed: {e}")
       # Provide fallback or alternative behavior
       return fallback_value

Don't Return Null
-----------------

Returning null is error-prone because callers might forget to check for it.

.. code-block:: python

   # Bad
   def get_user(user_id):
       user = db.find(user_id)
       if user:
           return user
       return None

   # Caller might forget to check
   user = get_user(123)
   print(user.name)  # Crashes if user is None

   # Good - use exceptions
   def get_user(user_id):
       user = db.find(user_id)
       if not user:
           raise UserNotFoundError(f"User {user_id} not found")
       return user

   # Or use Option/Maybe pattern
   from typing import Optional

   def get_user(user_id) -> Optional[User]:
       return db.find(user_id)

   # Caller must handle both cases
   user = get_user(123)
   if user:
       print(user.name)
   else:
       print("User not found")

Clean Up Resources
------------------

Always clean up resources, even when exceptions occur.

.. code-block:: python

   # Bad
   def process_file(filename):
       file = open(filename)
       data = file.read()
       # If an exception occurs here, file is not closed
       process(data)
       file.close()

   # Good - use context managers
   def process_file(filename):
       with open(filename) as file:
           data = file.read()
           process(data)
       # File is automatically closed

   # Good - use finally block when needed
   def connect_to_database():
       connection = None
       try:
           connection = db.connect()
           # Do work with connection
           return connection
       except DatabaseError as e:
           logger.error(f"Database error: {e}")
           raise
       finally:
           if connection:
               connection.close()

Handle Exceptions at the Appropriate Level
------------------------------------------

Don't catch exceptions too early. Let them propagate to where they can be handled properly.

.. code-block:: python

   # Bad - catching too early
   def process_order(order):
       try:
           validate_order(order)
           calculate_total(order)
           charge_payment(order)
           send_confirmation(order)
       except Exception as e:
           print(f"Error: {e}")  # Too generic

   # Good - handle at appropriate level
   def process_order(order):
       validate_order(order)
       calculate_total(order)
       charge_payment(order)
       send_confirmation(order)

   def handle_order_request(order_data):
       try:
           order = create_order(order_data)
           process_order(order)
           return success_response(order)
       except ValidationError as e:
           return error_response(400, str(e))
       except PaymentError as e:
           return error_response(402, str(e))
       except Exception as e:
           logger.error(f"Unexpected error: {e}")
           return error_response(500, "Internal server error")