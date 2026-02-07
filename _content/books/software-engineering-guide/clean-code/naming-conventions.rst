---
title: "Naming Conventions"
date: 2026-02-07T00:00:00
tags: ["clean-code", "naming", "readability"]
description: "Best practices for naming variables, functions, and classes"
---

Naming Conventions
==================

Good names are the foundation of readable code. A name should reveal intent, be pronounceable, and be searchable.

General Principles
------------------

**Reveal Intent**

The name of a variable, function, or class should answer the following questions: why does it exist, what does it do, and how is it used?

.. code-block:: python

   # Bad
   d = 10  # elapsed time in days

   # Good
   elapsed_time_in_days = 10
   days_since_creation = 10
   days_since_modification = 10

**Avoid Disinformation**

Avoid names that have different meanings in different contexts or that are similar to other names.

.. code-block:: python

   # Bad - could be a list or something else
   data_list = [1, 2, 3]

   # Good
   customer_ids = [1, 2, 3]

**Make Meaningful Distinctions**

Don't use names that differ only by noise words like number, string, or data.

.. code-block:: python

   # Bad
   product_data
   product_info

   # Good
   product
   product_description

**Use Pronounceable Names**

If you can't pronounce it, you can't discuss it without looking like an idiot.

.. code-block:: python

   # Bad
   class GenDTX {}
   dtx = GenDTX()

   # Good
   class DataTransferX {}
   data_transfer = DataTransferX()

**Use Searchable Names**

Single-letter names and numeric constants are hard to find in a codebase.

.. code-block:: python

   # Bad
   for i in range(0, 34):
       s = (t[i] * 4) / 5

   # Good
   real_days_per_ideal_day = 4
   WORK_DAYS_PER_WEEK = 5
   task_count = 34

   for task_index in range(0, task_count):
       real_task_days = task_estimate[task_index] * real_days_per_ideal_day
       real_task_days /= WORK_DAYS_PER_WEEK

Language-Specific Conventions
------------------------------

**Python**

- Use snake_case for variables and functions
- Use PascalCase for classes
- Use UPPER_CASE for constants

.. code-block:: python

   user_name = "John"
   def calculate_total():
       pass
   class UserAccount:
       pass
   MAX_RETRIES = 3

**JavaScript/TypeScript**

- Use camelCase for variables and functions
- Use PascalCase for classes and interfaces
- Use UPPER_CASE for constants

.. code-block:: javascript

   const userName = "John";
   function calculateTotal() {}
   class UserAccount {}
   const MAX_RETRIES = 3;

**Java**

- Use camelCase for variables and methods
- Use PascalCase for classes and interfaces
- Use UPPER_CASE for constants

.. code-block:: java

   String userName = "John";
   public void calculateTotal() {}
   public class UserAccount {}
   public static final int MAX_RETRIES = 3;