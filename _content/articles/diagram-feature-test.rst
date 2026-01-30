---
title: "Diagram Feature Test"
date: 2026-01-30T00:00:00
tags: ["diagrams", "rst", "features"]
author: "Peta"
---


Diagram Feature Test
====================

This article tests all the diagram types supported by Peta's diagram rendering feature.

Flowchart Example
-----------------

.. diagram:: flowchart
   
   Start -> Process -> Decision -> End
   Decision -> No -> Process
   Decision -> Yes -> End

This is a simple flowchart showing a decision-making process. The flowchart renderer automatically calculates node positions and draws edges with arrowheads.

Flowchart with Title
--------------------

.. diagram:: flowchart
   :title: Order Processing Flow
   
   Customer -> Order Received -> Check Stock -> Process Order -> Ship Order
   Check Stock -> Out of Stock -> Notify Customer
   Notify Customer -> End
   Process Order -> End
   Ship Order -> End

This flowchart demonstrates the order processing workflow with a custom title. Notice how the title is displayed at the top of the diagram.

Complex Flowchart
-----------------

.. diagram:: flowchart
   :title: Software Development Lifecycle
   
   Requirements -> Design -> Development -> Testing -> Deployment
   Design -> Approved -> Development
   Design -> Rejected -> Requirements
   Testing -> Passed -> Deployment
   Testing -> Failed -> Development

This flowchart shows the iterative nature of software development with feedback loops.

Gantt Chart Example
-------------------

.. diagram:: gantt
   
   Planning [2024-01-01] : 5d
   Development [2024-01-06] : 10d
   Testing [2024-01-16] : 5d
   Deployment [2024-01-21] : 2d

This Gantt chart shows a typical software development timeline with tasks, dates, and durations.

Gantt Chart with Title
----------------------

.. diagram:: gantt
   :title: Q1 2024 Project Timeline
   
   Research [2024-01-01] : 10d
   Design [2024-01-11] : 7d
   Implementation [2024-01-18] : 15d
   Testing [2024-02-02] : 8d
   Documentation [2024-02-10] : 5d

This Gantt chart displays a quarterly project timeline with overlapping phases.

Sequence Diagram Example
------------------------

.. diagram:: sequence
   
   User -> System: Login Request
   System -> Database: Query User
   Database -> System: User Data
   System -> User: Login Success

This sequence diagram illustrates the interaction between a user, system, and database during a login process.

Sequence Diagram with Title
---------------------------

.. diagram:: sequence
   :title: E-commerce Checkout Flow
   
   Customer -> Website: Add to Cart
   Website -> Cart: Store Item
   Cart -> Website: Update Total
   Customer -> Website: Proceed to Checkout
   Website -> Payment: Process Payment
   Payment -> Website: Payment Confirmed
   Website -> Order: Create Order
   Order -> Website: Order Created
   Website -> Customer: Confirmation

This sequence diagram shows the complete e-commerce checkout process with multiple actors.

Class Diagram Example
--------------------

.. diagram:: class-diagram
   
   User |+| Database
   User |+| API
   API |o| Cache

This class diagram shows the relationships between different components in a system, including composition (+|+) and aggregation (|o|).

Class Diagram with Title
-----------------------

.. diagram:: class-diagram
   :title: Blog System Architecture
   
   Post |+| Comment
   Post |+| Category
   Post |+| Tag
   User |+| Post
   User |+| Comment
   Comment |o| Post

This class diagram illustrates the relationships in a blog system with multiple interconnected entities.

State Diagram Example
--------------------

.. diagram:: state
   
   Idle -> Running : start
   Running -> Paused : pause
   Paused -> Running : resume
   Running -> Idle : stop

This state diagram demonstrates the state transitions in a simple system with start, pause, resume, and stop actions.

State Diagram with Title
------------------------

.. diagram:: state
   :title: Traffic Light System
   
   Red -> Green : timer
   Green -> Yellow : timer
   Yellow -> Red : timer
   Red -> Yellow : emergency

This state diagram models a traffic light system with both automatic and emergency state transitions.

How Diagrams Work
-----------------

All diagrams are rendered in Rust during the build time. The SVG markup is generated directly and embedded in the final HTML output. This means:

- No client-side JavaScript rendering required
- Fast page load times
- Static, cacheable output
- Full control over styling and theming

The diagram syntax is text-based and easy to write, similar to Mermaid but with Peta's own custom rendering engine.

Supported Diagram Types
-----------------------

* **Flowchart**: Node-based process diagrams with arrows
* **Gantt Chart**: Timeline-based project schedules
* **Sequence Diagram**: Interaction sequences between actors
* **Class Diagram**: UML class structures and relationships
* **State Diagram**: State transitions and events

Future Enhancements
-------------------

The diagram system is designed to be extensible. Future versions may include:

* Advanced layout algorithms (force-directed, hierarchical)
* Interactive hover effects
* Pan and zoom capabilities
* Export to PNG/SVG
* Customizable node shapes
* Subdiagram support
