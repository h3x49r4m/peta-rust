Flowchart Diagrams
==================

Flowcharts are the most versatile diagram type in Peta. They're perfect for visualizing processes, workflows, decision trees, and algorithmic logic.

Basic Syntax
------------

The fundamental syntax for flowcharts uses nodes connected by arrows:

.. diagram:: flowchart
   :title: Basic Flowchart
   
   Start -> Process -> End

This creates a linear flow from left to right.

Node Syntax
-----------

Creating Nodes
~~~~~~~~~~~~

Nodes are created simply by typing their name. The renderer automatically:
- Calculates optimal positioning
- Draws rounded rectangles for nodes
- Adds appropriate spacing

.. diagram:: flowchart
   
   Node A -> Node B -> Node C

Naming Conventions
~~~~~~~~~~~~~~~~~~

- Use descriptive names
- Spaces are allowed in node names
- Keep names concise
- Use consistent capitalization

Arrow Syntax
------------

Basic Arrows
~~~~~~~~~~~~

Use `->` to connect nodes:

.. diagram:: flowchart
   
   Source -> Destination

Multiple Connections
~~~~~~~~~~~~~~~~~~~~

A node can connect to multiple nodes:

.. diagram:: flowchart
   
   Start -> Process A
   Start -> Process B
   Process A -> End
   Process B -> End

Branching
---------

Decision Points
~~~~~~~~~~~~~~

Create decision points by branching from a single node:

.. diagram:: flowchart
   :title: Decision Flow
   
   Decision -> Option A
   Decision -> Option B

Labelled Branches
~~~~~~~~~~~~~~~~~

Add labels to branches using the arrow label syntax:

.. diagram:: flowchart
   :title: Labelled Decision
   
   Decision -> Yes -> Continue
   Decision -> No -> Stop

The text after the arrow becomes the branch label.

Complex Flows
-------------

Sequential Flow
~~~~~~~~~~~~~~~

Chain multiple nodes together:

.. diagram:: flowchart
   
   Step 1 -> Step 2 -> Step 3 -> Step 4 -> Step 5

Parallel Processing
~~~~~~~~~~~~~~~~~~

Show parallel processes:

.. diagram:: flowchart
   :title: Parallel Processing
   
   Start -> Fork
   Fork -> Process A
   Fork -> Process B
   Process A -> Join
   Process B -> Join
   Join -> End

Feedback Loops
~~~~~~~~~~~~~~

Create loops and iterations:

.. diagram:: flowchart
   :title: Feedback Loop
   
   Start -> Process
   Process -> Check
   Check -> Passed -> End
   Check -> Failed -> Process

Advanced Features
-----------------

Complex Workflows
~~~~~~~~~~~~~~~~

Combine multiple concepts:

.. diagram:: flowchart
   :title: Complete Workflow
   
   User Input -> Validation
   Validation -> Valid -> Processing
   Validation -> Invalid -> Show Error
   Show Error -> User Input
   Processing -> Save -> Database
   Save -> Success Message -> End
   Save -> Error Message -> End

Practical Examples
------------------

User Authentication
~~~~~~~~~~~~~~~~~~~

.. diagram:: flowchart
   :title: User Authentication
   
   User -> Login Page
   Login Page -> Validate Credentials
   Validate Credentials -> Valid -> Dashboard
   Validate Credentials -> Invalid -> Show Error
   Show Error -> Login Page
   Dashboard -> Logout -> Login Page

CI/CD Pipeline
~~~~~~~~~~~~~~

.. diagram:: flowchart
   :title: CI/CD Pipeline
   
   Code Push -> Build
   Build -> Tests
   Tests -> Passed -> Deploy
   Tests -> Failed -> Notify Developer
   Deploy -> Staging -> Production
   Notify Developer -> Fix -> Code Push

Order Processing
~~~~~~~~~~~~~~~~

.. diagram:: flowchart
   :title: Order Processing
   
   Order Received -> Check Inventory
   Check Inventory -> In Stock -> Process Payment
   Check Inventory -> Out of Stock -> Notify Customer
   Process Payment -> Payment Confirmed -> Ship Order
   Ship Order -> Delivery Confirmation -> Complete
   Notify Customer -> End
   Process Payment -> Payment Failed -> Cancel Order

Tips and Best Practices
-----------------------

1. **Keep it simple**: Avoid overly complex flows
2. **Use clear labels**: Make node names descriptive
3. **Logical flow**: Arrange nodes in logical order
4. **Consistent direction**: Generally left-to-right or top-to-bottom
5. **Balance branches**: Try to keep branches balanced

Common Patterns
---------------

Decision Tree
~~~~~~~~~~~~~

.. diagram:: flowchart
   :title: Decision Tree
   
   Root -> Question 1
   Question 1 -> Yes -> Action A
   Question 1 -> No -> Question 2
   Question 2 -> Yes -> Action B
   Question 2 -> No -> Action C

Error Handling
~~~~~~~~~~~~~

.. diagram:: flowchart
   :title: Error Handling
   
   Operation -> Success -> Next Step
   Operation -> Failure -> Log Error
   Log Error -> Retry -> Operation
   Log Error -> Max Retries -> Abort

Next Steps
----------

Now that you've mastered flowcharts, let's move on to Gantt charts for timeline visualization in the next chapter.