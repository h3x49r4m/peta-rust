---
title: "Diagram Feature Test"
date: 2026-01-30
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

Gantt Chart Example
-------------------

.. diagram:: gantt
   
   Planning [2024-01-01] : 5d
   Development [2024-01-06] : 10d
   Testing [2024-01-16] : 5d
   Deployment [2024-01-21] : 2d

This Gantt chart shows a typical software development timeline with tasks, dates, and durations.

Sequence Diagram Example
------------------------

.. diagram:: sequence
   
   User -> System: Login Request
   System -> Database: Query User
   Database -> System: User Data
   System -> User: Login Success

This sequence diagram illustrates the interaction between a user, system, and database during a login process.

Class Diagram Example
--------------------

.. diagram:: class-diagram
   
   User |+| Database
   User |+| API
   API |o| Cache

This class diagram shows the relationships between different components in a system, including composition (+|+) and aggregation (|o|).

State Diagram Example
--------------------

.. diagram:: state
   
   Idle -> Running : start
   Running -> Paused : pause
   Paused -> Running : resume
   Running -> Idle : stop

This state diagram demonstrates the state transitions in a simple system with start, pause, resume, and stop actions.

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
