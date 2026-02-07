---
title: "Behavioral Design Patterns"
date: 2026-02-07T00:00:00
tags: ["design-patterns", "behavioral", "communication"]
description: "Behavioral design patterns for communication between objects"
---

Behavioral Design Patterns
==========================

Behavioral design patterns are concerned with algorithms and the assignment of responsibilities between objects. They describe not just patterns of objects or classes but also the patterns of communication between them.

Observer Pattern
----------------

The Observer pattern defines a one-to-many dependency between objects so that when one object changes state, all its dependents are notified and updated automatically.

**Use Cases:**

- Event handling systems
- Model-View-Controller (MVC) architectures
- Distributed event handling systems
- News feed updates

**Example:**

.. code-block:: python

   from abc import ABC, abstractmethod

   class Observer(ABC):
       @abstractmethod
       def update(self, message):
           pass

   class Subject:
       def __init__(self):
           self._observers = []

       def attach(self, observer):
           self._observers.append(observer)

       def detach(self, observer):
           self._observers.remove(observer)

       def notify(self, message):
           for observer in self._observers:
               observer.update(message)

   class NewsAgency(Subject):
       def add_news(self, news):
           self.notify(news)

   class NewsChannel(Observer):
       def __init__(self, name):
           self.name = name

       def update(self, news):
           print(f"{self.name} received: {news}")

Strategy Pattern
----------------

The Strategy pattern defines a family of algorithms, encapsulates each one, and makes them interchangeable. It lets the algorithm vary independently from clients that use it.

**Use Cases:**

- When you want to define a family of algorithms
- When you need different variants of an algorithm
- When algorithms use data that clients shouldn't know about

**Example:**

.. code-block:: python

   from abc import ABC, abstractmethod

   class PaymentStrategy(ABC):
       @abstractmethod
       def pay(self, amount):
           pass

   class CreditCardPayment(PaymentStrategy):
       def pay(self, amount):
           print(f"Paid ${amount} via Credit Card")

   class PayPalPayment(PaymentStrategy):
       def pay(self, amount):
           print(f"Paid ${amount} via PayPal")

   class ShoppingCart:
       def __init__(self, payment_strategy):
           self.payment_strategy = payment_strategy

       def checkout(self, amount):
           self.payment_strategy.pay(amount)

Command Pattern
---------------

The Command pattern encapsulates a request as an object, thereby letting you parameterize clients with different requests, queue or log requests, and support undoable operations.

**Use Cases:**

- GUI buttons and menus
- Macro recording
- Multi-level undo
- Transaction processing

**Example:**

.. code-block:: python

   from abc import ABC, abstractmethod

   class Command(ABC):
       @abstractmethod
       def execute(self):
           pass

   class Light:
       def turn_on(self):
           print("Light is ON")

       def turn_off(self):
           print("Light is OFF")

   class LightOnCommand(Command):
       def __init__(self, light):
           self.light = light

       def execute(self):
           self.light.turn_on()

   class LightOffCommand(Command):
       def __init__(self, light):
           self.light = light

       def execute(self):
           self.light.turn_off()

   class RemoteControl:
       def __init__(self):
           self.command = None

       def set_command(self, command):
           self.command = command

       def press_button(self):
           if self.command:
               self.command.execute()