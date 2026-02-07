---
title: "Structural Design Patterns"
date: 2026-02-07T00:00:00
tags: ["design-patterns", "structural", "composition"]
description: "Structural design patterns for object composition"
---

Structural Design Patterns
==========================

Structural design patterns explain how to assemble objects and classes into larger structures while keeping these structures flexible and efficient.

Adapter Pattern
---------------

The Adapter pattern allows incompatible interfaces to work together. It acts as a bridge between two incompatible interfaces.

**Use Cases:**

- Integrating third-party libraries
- Making existing classes work with others without modifying their source code
- When you need to use several existing subclasses, but by adapting their interface to a common one

**Example:**

.. code-block:: python

   class EuropeanSocket:
       def voltage(self):
           return 220

   class USPlug:
       def voltage(self):
           return 110

   class SocketAdapter:
       def __init__(self, socket):
           self.socket = socket

       def voltage(self):
           volts = self.socket.voltage()
           if volts == 220:
               return 110  # Convert to US voltage
           return volts

Decorator Pattern
------------------

The Decorator pattern adds new functionality to an existing object without altering its structure.

**Use Cases:**

- Adding responsibilities to individual objects dynamically
- When extension by subclassing is impractical
- When you want to add functionality without affecting other objects

**Example:**

.. code-block:: python

   class Coffee:
       def cost(self):
           return 5

       def description(self):
           return "Simple Coffee"

   class MilkDecorator:
       def __init__(self, coffee):
           self.coffee = coffee

       def cost(self):
           return self.coffee.cost() + 1

       def description(self):
           return f"{self.coffee.description()}, Milk"

   class SugarDecorator:
       def __init__(self, coffee):
           self.coffee = coffee

       def cost(self):
           return self.coffee.cost() + 0.5

       def description(self):
           return f"{self.coffee.description()}, Sugar"

Facade Pattern
--------------

The Facade pattern provides a simplified interface to a library, a framework, or any other complex set of classes.

**Use Cases:**

- When you want to provide a simple interface to a complex subsystem
- When the subsystem is getting complex and you want to use a simpler interface
- When you want to layer your subsystems

**Example:**

.. code-block:: python

   class CPU:
       def freeze(self):
           pass

       def jump(self, position):
           pass

       def execute(self):
           pass

   class Memory:
       def load(self, position, data):
           pass

   class HardDrive:
       def read(self, lba, size):
           return "data"

   class ComputerFacade:
       def __init__(self):
           self.cpu = CPU()
           self.memory = Memory()
           self.hard_drive = HardDrive()

       def start(self):
           self.cpu.freeze()
           self.memory.load(0, self.hard_drive.read(0, 1024))
           self.cpu.jump(0)
           self.cpu.execute()