---
title: "Design Principles"
date: 2026-02-07T00:00:00
tags: ["system-design", "principles", "architecture"]
description: "Core system design principles"
---

Design Principles
=================

System design principles provide the foundation for building robust, scalable, and maintainable software systems.

Core Principles
---------------

**Modularity**

Separate concerns into independent, interchangeable modules.

.. code-block:: python

   # Good - modular design
   class UserService:
       def __init__(self, user_repository):
           self.user_repository = user_repository

       def get_user(self, user_id):
           return self.user_repository.find(user_id)

   class EmailService:
       def send_welcome_email(self, user):
           # Send email logic
           pass

   # Bad - monolithic design
   class UserService:
       def get_user(self, user_id):
           # Database logic mixed with business logic
           return db.query("SELECT * FROM users WHERE id = ?", user_id)

       def send_welcome_email(self, user):
           # Email logic mixed with user logic
           email_service.send(user.email, "Welcome!")

**Separation of Concerns**

Each component should have a single, well-defined responsibility.

.. code-block:: python

   # Good - separated concerns
   class UserController:
       def __init__(self, user_service):
           self.user_service = user_service

       def get_user(self, request):
           user_id = request.get('user_id')
           return self.user_service.get_user(user_id)

   class UserService:
       def get_user(self, user_id):
           return self.user_repository.find(user_id)

   # Bad - mixed concerns
   class UserController:
       def get_user(self, request):
           # Database query in controller
           user = db.query("SELECT * FROM users WHERE id = ?", user_id)
           # Business logic in controller
           if not user.is_active:
               raise Exception("User is not active")
           return user

**Loose Coupling**

Minimize dependencies between components.

.. code-block:: python

   # Good - loose coupling via interfaces
   class OrderProcessor:
       def __init__(self, payment_gateway, notification_service):
           self.payment_gateway = payment_gateway
           self.notification_service = notification_service

       def process_order(self, order):
           self.payment_gateway.charge(order.amount)
           self.notification_service.notify(order.user)

   # Bad - tight coupling
   class OrderProcessor:
       def process_order(self, order):
           # Direct dependency on specific implementations
           StripePaymentGateway().charge(order.amount)
           SendGridEmailService().send(order.user.email, "Order confirmed")

**High Cohesion**

Group related functionality together.

.. code-block:: python

   # Good - high cohesion
   class UserService:
       def create_user(self, user_data):
           pass

       def update_user(self, user_id, user_data):
           pass

       def delete_user(self, user_id):
           pass

       def get_user(self, user_id):
           pass

   # Bad - low cohesion
   class UserService:
       def create_user(self, user_data):
           pass

       def send_email(self, user, message):
           pass

       def log_transaction(self, transaction):
           pass

SOLID Principles
----------------

**Single Responsibility Principle (SRP)**

A class should have only one reason to change.

.. code-block:: python

   # Good - single responsibility
   class UserValidator:
       def validate(self, user_data):
           if not user_data.get('email'):
               raise ValueError('Email is required')

   class UserRepository:
       def save(self, user):
           # Save to database
           pass

   # Bad - multiple responsibilities
   class UserManager:
       def validate(self, user_data):
           # Validation logic
           pass

       def save(self, user):
           # Database logic
           pass

**Open/Closed Principle (OCP)**

Software entities should be open for extension but closed for modification.

.. code-block:: python

   # Good - open for extension
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

   # Bad - requires modification
   class PaymentProcessor:
       def process_payment(self, payment_type, amount):
           if payment_type == 'credit_card':
               # Credit card logic
           elif payment_type == 'paypal':
               # PayPal logic
           # Add new payment type requires modification

**Liskov Substitution Principle (LSP)**

Objects of a superclass should be replaceable with objects of a subclass.

.. code-block:: python

   # Good - LSP compliant
   class Rectangle:
       def set_width(self, width):
           self.width = width

       def set_height(self, height):
           self.height = height

       def area(self):
           return self.width * self.height

   class Square(Rectangle):
       def set_width(self, width):
           self.width = width
           self.height = width  # Maintain square property

       def set_height(self, height):
           self.width = height
           self.height = height

   # Bad - violates LSP
   class Rectangle:
       def set_width(self, width):
           self.width = width

       def set_height(self, height):
           self.height = height

   class Square(Rectangle):
       def set_width(self, width):
           self.width = width
           self.height = width  # Breaks Rectangle contract

**Interface Segregation Principle (ISP)**

Clients shouldn't be forced to depend on interfaces they don't use.

.. code-block:: python

   # Good - segregated interfaces
   class Readable:
       def read(self):
           pass

   class Writable:
       def write(self, data):
           pass

   class ReadableFile(Readable):
       def read(self):
           # Read-only implementation
           pass

   # Bad - fat interface
   class File:
       def read(self):
           pass

       def write(self, data):
           pass

       def delete(self):
           pass

**Dependency Inversion Principle (DIP)**

Depend on abstractions, not concretions.

.. code-block:: python

   # Good - depends on abstraction
   class UserController:
       def __init__(self, user_repository: UserRepository):
           self.user_repository = user_repository

   # Bad - depends on concrete implementation
   class UserController:
       def __init__(self):
           self.user_repository = SQLUserRepository()

Design Patterns in System Design
---------------------------------

**Layered Architecture**

Organize code into layers with clear responsibilities.

.. code-block:: python

   # Presentation Layer
   class UserAPI:
       def get_user(self, user_id):
           user = self.user_service.get_user(user_id)
           return self.serialize(user)

   # Business Logic Layer
   class UserService:
       def get_user(self, user_id):
           return self.user_repository.find(user_id)

   # Data Access Layer
   class UserRepository:
       def find(self, user_id):
           return db.query("SELECT * FROM users WHERE id = ?", user_id)

**Microkernel Pattern**

Separate minimal core from plug-in functionality.

.. code-block:: python

   class Microkernel:
       def __init__(self):
           self.plugins = {}

       def register_plugin(self, name, plugin):
           self.plugins[name] = plugin

       def execute(self, plugin_name, *args):
           plugin = self.plugins.get(plugin_name)
           if plugin:
               return plugin.execute(*args)

   class AuthenticationPlugin:
       def execute(self, credentials):
           # Authentication logic
           pass

**Event-Driven Architecture**

Components communicate through events.

.. code-block:: python

   class EventBus:
       def __init__(self):
           self.subscribers = {}

       def subscribe(self, event_type, handler):
           if event_type not in self.subscribers:
               self.subscribers[event_type] = []
           self.subscribers[event_type].append(handler)

       def publish(self, event_type, event_data):
           handlers = self.subscribers.get(event_type, [])
           for handler in handlers:
               handler(event_data)

Design Considerations
---------------------

**Performance**

Consider performance implications of design decisions.

.. code-block:: python

   # Good - efficient data structure choice
   def find_user(users, user_id):
       # Using dictionary for O(1) lookup
       return users.get(user_id)

   # Bad - inefficient
   def find_user(users, user_id):
       # Linear search O(n)
       for user in users:
           if user.id == user_id:
               return user

**Scalability**

Design for horizontal and vertical scaling.

.. code-block:: python

   # Good - stateless design for horizontal scaling
   class RequestHandler:
       def handle_request(self, request):
           # No session state stored
           result = process(request)
           return result

   # Bad - stateful design
   class RequestHandler:
       def __init__(self):
           self.session_data = {}

       def handle_request(self, request):
           # Session state limits scalability
           self.session_data[request.id] = process(request)
           return self.session_data[request.id]

**Maintainability**

Design for easy maintenance and evolution.

.. code-block:: python

   # Good - clear, maintainable code
   def calculate_discount(price, discount_rate):
       if discount_rate < 0 or discount_rate > 1:
           raise ValueError("Discount rate must be between 0 and 1")
       return price * (1 - discount_rate)

   # Bad - cryptic, hard to maintain
   def calc(p, d):
       return p - (p * d) if 0 <= d <= 1 else p

Best Practices
--------------

**Start Simple**

Begin with simple design and evolve as needed.

**Document Decisions**

Record design decisions and their rationale.

**Review Regularly**

Regularly review and refine system design.

**Iterate**

Continuously improve based on feedback and requirements.