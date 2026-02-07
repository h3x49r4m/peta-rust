---
title: "Refactoring"
date: 2026-02-07T00:00:00
tags: ["clean-code", "refactoring", "maintenance"]
description: "Techniques for improving code structure without changing behavior"
---

Refactoring
===========

Refactoring is the process of changing a software system in such a way that it does not alter the external behavior of the code yet improves its internal structure.

When to Refactor
----------------

**Rule of Three**

1. First time, just get it done
2. Second time, you wince at the duplication, but you do it anyway
3. Third time, you refactor

**When You Add a Function**

The best time to refactor is when you add a new function. The new function gives you a chance to look at the surrounding code and see if it needs improvement.

**When You Fix a Bug**

When fixing a bug, you often gain insight into the code that can be used to improve it.

**During Code Reviews**

Code reviews are excellent opportunities to identify areas that need refactoring.

Refactoring Techniques
----------------------

Extract Method
~~~~~~~~~~~~~~

When you have a code fragment that can be grouped together, turn it into a method whose name explains the purpose of the method.

.. code-block:: python

   # Before
   def print_owing():
       print_banner()
       outstanding = calculate_outstanding()
       print_details(outstanding)

   def print_details(outstanding):
       print(f"Name: {customer.name}")
       print(f"Amount: {outstanding}")
       print(f"Date: {datetime.now().strftime('%Y-%m-%d')}")

   # After - extract the date formatting
   def print_owing():
       print_banner()
       outstanding = calculate_outstanding()
       print_details(outstanding)

   def print_details(outstanding):
       print(f"Name: {customer.name}")
       print(f"Amount: {outstanding}")
       print(f"Date: {get_formatted_date()}")

   def get_formatted_date():
       return datetime.now().strftime('%Y-%m-%d')

Inline Method
~~~~~~~~~~~~~

Sometimes a method's body is just as clear as its name. In that case, remove the method and put its code where it was called.

.. code-block:: python

   # Before
   def get_rating():
       return more_than_five_late_deliveries() ? 2 : 1

   def more_than_five_late_deliveries():
       return number_of_late_deliveries > 5

   # After
   def get_rating():
       return 2 if number_of_late_deliveries > 5 else 1

Extract Variable
~~~~~~~~~~~~~~~~

Make an expression self-documenting by putting it into a well-named variable.

.. code-block:: python

   # Before
   price = order.item_price * order.quantity * (1 - order.discount)

   # After
   base_price = order.item_price * order.quantity
   discount_factor = 1 - order.discount
   price = base_price * discount_factor

Replace Magic Numbers with Constants
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Magic numbers are numbers with special meaning that appear in code without context.

.. code-block:: python

   # Before
   if (days > 365):
       print("More than a year")

   # After
   DAYS_IN_YEAR = 365
   if (days > DAYS_IN_YEAR):
       print("More than a year")

Replace Conditional with Polymorphism
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

When you have a conditional that chooses different behavior based on the type of an object, move each leg of the conditional to a separate subclass.

.. code-block:: python

   # Before
   def calculate_pay(employee):
       if employee.type == "hourly":
           return employee.hourly_rate * employee.hours_worked
       elif employee.type == "salaried":
           return employee.monthly_salary
       elif employee.type == "commissioned":
           return employee.monthly_salary + (employee.commission_rate * employee.sales)

   # After
   class Employee:
       def calculate_pay(self):
           raise NotImplementedError

   class HourlyEmployee(Employee):
       def calculate_pay(self):
           return self.hourly_rate * self.hours_worked

   class SalariedEmployee(Employee):
       def calculate_pay(self):
           return self.monthly_salary

   class CommissionedEmployee(Employee):
       def calculate_pay(self):
           return self.monthly_salary + (self.commission_rate * self.sales)

Refactoring Principles
----------------------

**Two Hats**

When you refactor, you wear two hats:

1. Adding function hat: Add new capability
2. Refactoring hat: Restructure code without changing behavior

Never wear both hats at the same time.

**Small Steps**

Refactor in small steps. Test after each step. If something breaks, you'll know exactly where.

**Tests**

You can't refactor safely without a comprehensive test suite. Tests give you confidence that your refactoring hasn't broken anything.

**Version Control**

Commit often. If a refactoring goes wrong, you can easily revert.

Common Refactoring Smells
--------------------------

**Duplicated Code**

When you see the same code in more than one place, you know you have a problem.

**Long Method**

Long methods are hard to understand and often do too many things.

**Large Class**

Classes that try to do too many things become bloated and hard to maintain.

**Long Parameter List**

Methods with many parameters are hard to understand and use.

**Divergent Change**

When one class is commonly changed in different ways for different reasons.

**Shotgun Surgery**

When every time you make a kind of change, you have to make a lot of little changes to a lot of different classes.

**Feature Envy**

A method that seems more interested in a class other than the one it actually is in.