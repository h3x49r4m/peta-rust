---
title: "Creational Design Patterns"
date: 2026-02-07T00:00:00
tags: ["design-patterns", "creational", "object-creation"]
description: "Creational design patterns for object creation"
---

Creational Design Patterns
==========================

Creational design patterns deal with object creation mechanisms, trying to create objects in a manner suitable to the situation. The basic form of object creation could result in design problems or added complexity to the design.

Singleton Pattern
-----------------

The Singleton pattern ensures a class has only one instance and provides a global point of access to it.

**Use Cases:**

- Configuration managers
- Database connection pools
- Logging services
- Caching systems

**Example:**

.. code-block:: python

   class Singleton:
       _instance = None

       def __new__(cls):
           if cls._instance is None:
               cls._instance = super().__new__(cls)
               cls._instance.initialized = False
           return cls._instance

       def initialize(self):
           if not self.initialized:
               # Initialize resources
               self.initialized = True

Factory Method Pattern
-----------------------

The Factory Method pattern defines an interface for creating an object, but let subclasses decide which class to instantiate.

**Use Cases:**

- When a class cannot anticipate the class of objects it must create
- When a class wants its subclasses to specify the objects it creates
- When classes delegate responsibility to one of several helper subclasses

**Example:**

.. code-block:: python

   from abc import ABC, abstractmethod

   class Animal(ABC):
       @abstractmethod
       def speak(self):
           pass

   class Dog(Animal):
       def speak(self):
           return "Woof!"

   class Cat(Animal):
       def speak(self):
           return "Meow!"

   class AnimalFactory:
       @staticmethod
       def create_animal(animal_type):
           if animal_type == "dog":
               return Dog()
           elif animal_type == "cat":
               return Cat()
           raise ValueError(f"Unknown animal type: {animal_type}")

Builder Pattern
---------------

The Builder pattern separates the construction of a complex object from its representation, allowing the same construction process to create different representations.

**Use Cases:**

- Constructing complex objects step by step
- When the construction process should be independent of the parts
- When the object must be created in different ways

**Example:**

.. code-block:: python

   class Pizza:
       def __init__(self):
           self.dough = None
           self.sauce = None
           self.toppings = []

       def __str__(self):
           return f"Pizza with {self.dough}, {self.sauce}, and {self.toppings}"

   class PizzaBuilder:
       def __init__(self):
           self.pizza = Pizza()

       def set_dough(self, dough):
           self.pizza.dough = dough
           return self

       def set_sauce(self, sauce):
           self.pizza.sauce = sauce
           return self

       def add_topping(self, topping):
           self.pizza.toppings.append(topping)
           return self

       def build(self):
           return self.pizza