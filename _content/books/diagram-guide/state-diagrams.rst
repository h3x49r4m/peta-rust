State Diagrams
==============

State diagrams visualize the lifecycle of objects, showing how they transition between different states. They're perfect for modeling workflows, lifecycles, and state machines.

Basic Syntax
------------

The fundamental syntax for state diagrams shows state transitions with events:

.. diagram:: state
   :title: Basic State Diagram
   
   State A -> State B : event

This creates a transition from State A to State B triggered by an event.

State Syntax
------------

Creating States
~~~~~~~~~~~~~~

States are created simply by naming them:

.. diagram:: state
   
   Idle -> Running

The renderer automatically:
- Creates state boxes
- Draws transitions with arrows
- Labels transitions with events

Transition Syntax
-----------------

Basic Transitions
~~~~~~~~~~~~~~~~~

Use `->` to create transitions:

.. code-block:: rst

   FromState -> ToState : event

Components:
- **FromState**: The source state
- **Arrow**: `->` indicates a transition
- **ToState**: The destination state
- **Event**: The trigger for the transition (optional)

Example:

.. diagram:: state
   :title: State Transition
   
   Off -> On : power on
   On -> Off : power off

Multiple States
---------------

Linear Flow
~~~~~~~~~~~

Sequential state changes:

.. diagram:: state
   
   Created -> Processing
   Processing -> Completed
   Completed -> Archived

Branching Paths
~~~~~~~~~~~~~~~

Multiple transitions from one state:

.. diagram:: state
   :title: Branching States
   
   Decision -> Success : pass
   Decision -> Failure : fail
   Failure -> Retry : retry
   Failure -> Abort : abort

Complex Lifecycles
------------------

Complete Workflow
~~~~~~~~~~~~~~~~~

Show full lifecycle:

.. diagram:: state
   :title: Document Lifecycle
   
   Draft -> Review : submit
   Review -> Approved : approve
   Review -> Rejected : reject
   Approved -> Published : publish
   Published -> Archived : archive
   Rejected -> Draft : revise

State Machines
~~~~~~~~~~~~~~

Model complex state machines:

.. diagram:: state
   :title: Order State Machine
   
   Created -> Pending : payment received
   Pending -> Processing : validated
   Processing -> Shipped : processed
   Processing -> Cancelled : customer request
   Shipped -> Delivered : delivered
   Delivered -> Complete : feedback received
   Cancelled -> Refunded : refund processed

Practical Examples
------------------

User Session
~~~~~~~~~~~~

.. diagram:: state
   :title: User Session Lifecycle
   
   Anonymous -> Logged In : login
   Logged In -> Active : activity
   Active -> Idle : timeout
   Idle -> Active : activity
   Active -> Logged Out : logout
   Logged Out -> Anonymous : session expired

Traffic Light
~~~~~~~~~~~~~

.. diagram:: state
   :title: Traffic Light System
   
   Red -> Green : timer
   Green -> Yellow : timer
   Yellow -> Red : timer
   Red -> Yellow : emergency

Payment Processing
~~~~~~~~~~~~~~~~~~

.. diagram:: state
   :title: Payment Processing
   
   Initiated -> Processing : submit
   Processing -> Completed : success
   Processing -> Failed : decline
   Processing -> Cancelled : timeout
   Failed -> Retrying : retry
   Retrying -> Processing : retry
   Retrying -> Failed : max retries
   Cancelled -> Refunded : refund request

Ticket Lifecycle
~~~~~~~~~~~~~~~~

.. diagram:: state
   :title: Support Ticket Lifecycle
   
   Open -> In Progress : assigned
   In Progress -> Resolved : fixed
   In Progress -> Escalated : complex
   Escalated -> In Progress : reassigned
   Resolved -> Closed : confirmed
   Resolved -> Reopened : issue persists
   Reopened -> In Progress : new info
   Open -> Closed : duplicate

Tips and Best Practices
-----------------------

1. **Clear state names**: Use descriptive, concise names
2. **Logical flow**: Arrange states in logical order
3. **Meaningful events**: Describe what triggers each transition
4. **Complete lifecycle**: Include all possible states
5. **Handle all paths**: Show all possible transitions

State Design Guidelines
-----------------------

State Characteristics
~~~~~~~~~~~~~~~~~~~~~

Good states:
- Mutually exclusive (an object is in only one state at a time)
- Clearly defined (no ambiguity about what the state means)
- Meaningful (represent real-world conditions)
- Stable (don't change too frequently)

Event Characteristics
~~~~~~~~~~~~~~~~~~~~~

Good events:
- Clear triggers (what causes the transition)
- Atomic (can't be broken down further)
- Timely (occur at a specific moment)
- Relevant (meaningful to the system)

Common Patterns
---------------

Simple Lifecycle
~~~~~~~~~~~~~~~~

.. diagram:: state
   :title: Simple Lifecycle
   
   Created -> Active : activate
   Active -> Inactive : deactivate
   Inactive -> Archived : archive

Approval Process
~~~~~~~~~~~~~~~

.. diagram:: state
   :title: Approval Process
   
   Draft -> Pending Review : submit
   Pending Review -> Approved : approve
   Pending Review -> Rejected : reject
   Approved -> Published : publish
   Rejected -> Draft : revise

Retry Logic
~~~~~~~~~~~

.. diagram:: state
   :title: Retry Logic
   
   Attempting -> Success : completed
   Attempting -> Retrying : failed
   Retrying -> Attempting : retry
   Retrying -> Failed : max retries
   Failed -> Aborted : give up

Next Steps
----------

Now that you understand all five diagram types, let's explore advanced features and best practices in the next chapter.