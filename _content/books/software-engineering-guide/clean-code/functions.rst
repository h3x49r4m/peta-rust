---
title: "Functions"
date: 2026-02-07T00:00:00
tags: ["clean-code", "functions", "modularity"]
description: "Writing clean, modular functions"
---

Functions
=========

Functions should be small, do one thing, and do it well. They are the first line of defense in the battle for clean code.

Small Functions
---------------

Functions should be almost never more than 20 lines long. The smaller the function, the more likely it is to do one thing.

.. code-block:: python

   # Bad - function does too many things
   def process_user_data(user_data):
       # Validate input
       if not user_data:
           return None
       if 'email' not in user_data or 'name' not in user_data:
           return None

       # Format data
       email = user_data['email'].lower().strip()
       name = user_data['name'].strip().title()

       # Save to database
       db.connect()
       db.insert({'email': email, 'name': name})
       db.close()

       # Send email
       email_service.send_welcome(email)

       return {'email': email, 'name': name}

   # Good - single responsibility functions
   def validate_user_data(user_data):
       if not user_data:
           return False
       required_fields = ['email', 'name']
       return all(field in user_data for field in required_fields)

   def format_user_data(user_data):
       return {
           'email': user_data['email'].lower().strip(),
           'name': user_data['name'].strip().title()
       }

   def save_user_to_database(user_data):
       db.connect()
       db.insert(user_data)
       db.close()

   def send_welcome_email(email):
       email_service.send_welcome(email)

   def process_user_data(user_data):
       if not validate_user_data(user_data):
           return None

       formatted_data = format_user_data(user_data)
       save_user_to_database(formatted_data)
       send_welcome_email(formatted_data['email'])

       return formatted_data

Do One Thing
------------

Functions should do one thing. They should do it well. They should do it only.

**One Level of Abstraction per Function**

The statements within a function should all be at the same level of abstraction.

.. code-block:: python

   # Bad - mixed levels of abstraction
   def get_html(page):
       # High-level concept
       page = get_page_from_server(page)
       # Low-level concept
       result = ""
       result += "<html>"
       result += "<body>"
       result += page.content
       result += "</body>"
       result += "</html>"
       return result

   # Good - consistent abstraction level
   def get_html(page):
       page = get_page_from_server(page)
       return create_html_template(page.content)

   def create_html_template(content):
       return f"<html><body>{content}</body></html>"

Descriptive Names
-----------------

The name of a function should be a verb or verb phrase that describes what the function does.

.. code-block:: python

   # Bad
   def user_data():
       pass

   # Good
   def get_user_data():
       pass

   def set_user_data(data):
       pass

   def validate_user_data(data):
       pass

**Boolean Functions**

Functions that return boolean values should be named with prefixes like `is`, `has`, `can`, or `should`.

.. code-block:: python

   # Bad
   def valid_email(email):
       return "@" in email

   # Good
   def is_valid_email(email):
       return "@" in email

   def has_permission(user, resource):
       return resource in user.permissions

   def can_access(user, resource):
       return user.has_permission(resource)

Arguments
---------

The ideal number of arguments for a function is zero. Next comes one, followed closely by two. Three arguments should be avoided where possible.

.. code-block:: python

   # Bad - too many arguments
   def create_user(name, email, age, address, phone, country):
       pass

   # Good - use data structures
   def create_user(user_info):
       name = user_info['name']
       email = user_info['email']
       # ... rest of implementation

   # Even better - use object
   def create_user(user):
       # user is an object with properties
       pass

Flag Arguments
--------------

Flag arguments are ugly. Passing a boolean into a function is a truly terrible practice.

.. code-block:: python

   # Bad
   def render_page(is_admin):
       if is_admin:
           return render_admin_page()
       else:
           return render_user_page()

   # Good
   def render_admin_page():
       pass

   def render_user_page():
       pass

No Side Effects
---------------

A function should not change the state of system variables or parameters passed to it.

.. code-block:: python

   # Bad - modifies input
   def add_timestamp(data):
       data['timestamp'] = datetime.now()
       return data

   # Good - returns new object
   def add_timestamp(data):
       new_data = data.copy()
       new_data['timestamp'] = datetime.now()
       return new_data