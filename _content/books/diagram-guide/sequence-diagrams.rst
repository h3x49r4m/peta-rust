Sequence Diagrams
=================

Sequence diagrams visualize interactions between different actors or components over time. They're perfect for showing how systems communicate, API flows, and process interactions.

Basic Syntax
------------

The fundamental syntax for sequence diagrams shows messages passing between actors:

.. diagram:: sequence
   :title: Basic Sequence
   
   Actor A -> Actor B: Message

This creates a simple interaction between two actors.

Message Syntax
--------------

Sending Messages
~~~~~~~~~~~~~~~~

Messages are defined using arrow syntax:

.. code-block:: rst

   Actor A -> Actor B: Message content

Components:
- **Source**: The actor sending the message
- **Arrow**: `->` indicates a message
- **Destination**: The actor receiving the message
- **Message**: Description of the message (optional)

Example:

.. diagram:: sequence
   :title: Message Exchange
   
   User -> System: Login Request
   System -> User: Login Response

Creating Actors
---------------

Actor Creation
~~~~~~~~~~~~~~

Actors are automatically created when they first appear in the diagram. You don't need to declare them explicitly.

.. diagram:: sequence
   
   Alice -> Bob: Hello
   Bob -> Alice: Hi there!

Multiple Actors
~~~~~~~~~~~~~~~

Use as many actors as needed:

.. diagram:: sequence
   :title: Multi-Actor Interaction
   
   User -> Frontend: Request
   Frontend -> API: Call
   API -> Database: Query
   Database -> API: Data
   API -> Frontend: Response
   Frontend -> User: Result

Message Flows
-------------

Linear Flow
~~~~~~~~~~~

Sequential messages:

.. diagram:: sequence
   
   A -> B: Message 1
   B -> C: Message 2
   C -> D: Message 3

Request-Response Pattern
~~~~~~~~~~~~~~~~~~~~~~~~

Show request-response pairs:

.. diagram:: sequence
   :title: Request-Response
   
   Client -> Server: Request
   Server -> Client: Response

Complex Interactions
--------------------

Multiple Exchanges
~~~~~~~~~~~~~~~~~~

Show back-and-forth communication:

.. diagram:: sequence
   :title: Multiple Exchanges
   
   User -> App: Login
   App -> Auth: Validate
   Auth -> App: Success
   App -> Database: Fetch Data
   Database -> App: Data
   App -> User: Welcome

Branching Logic
~~~~~~~~~~~~~~~

Represent conditional flows:

.. diagram:: sequence
   :title: Conditional Flow
   
   User -> System: Action
   System -> Validator: Check
   Validator -> System: Valid
   System -> Processor: Execute
   Processor -> System: Done
   System -> User: Success

Practical Examples
------------------

User Authentication
~~~~~~~~~~~~~~~~~~~

.. diagram:: sequence
   :title: User Authentication Flow
   
   User -> Login Page: Submit Credentials
   Login Page -> Auth Service: Validate
   Auth Service -> Database: Query User
   Database -> Auth Service: User Data
   Auth Service -> Login Page: Valid
   Login Page -> Session Service: Create Session
   Session Service -> Login Page: Session ID
   Login Page -> User: Redirect to Dashboard

E-commerce Checkout
~~~~~~~~~~~~~~~~~~~

.. diagram:: sequence
   :title: E-commerce Checkout
   
   Customer -> Cart: Add Item
   Cart -> Inventory: Check Stock
   Inventory -> Cart: Available
   Cart -> Customer: Update Total
   Customer -> Checkout: Proceed
   Checkout -> Payment: Process
   Payment -> Bank: Charge
   Bank -> Payment: Confirmed
   Payment -> Checkout: Success
   Checkout -> Order: Create
   Order -> Email: Send Confirmation
   Email -> Customer: Receipt

API Request Handling
~~~~~~~~~~~~~~~~~~~~

.. diagram:: sequence
   :title: API Request Flow
   
   Client -> API Gateway: GET /resource
   API Gateway -> Auth: Validate Token
   Auth -> API Gateway: Valid
   API Gateway -> Load Balancer: Forward
   Load Balancer -> Service: Request
   Service -> Cache: Check Cache
   Cache -> Service: Cache Miss
   Service -> Database: Query
   Database -> Service: Data
   Service -> Cache: Store
   Service -> Load Balancer: Response
   Load Balancer -> API Gateway: Response
   API Gateway -> Client: JSON Response

Tips and Best Practices
-----------------------

1. **Clear actor names**: Use descriptive, concise names
2. **Logical flow**: Arrange messages in chronological order
3. **Meaningful messages**: Describe what each message represents
4. **Consistent direction**: Show time flowing downward
5. **Appropriate detail**: Include relevant steps without overwhelming

Common Patterns
---------------

Error Handling
~~~~~~~~~~~~~~

.. diagram:: sequence
   :title: Error Handling
   
   Client -> Service: Request
   Service -> Validator: Validate
   Validator -> Service: Error
   Service -> Client: Error Response

Parallel Processing
~~~~~~~~~~~~~~~~~~~

.. diagram:: sequence
   :title: Parallel Requests
   
   Client -> Service A: Request 1
   Client -> Service B: Request 2
   Service A -> Client: Response 1
   Service B -> Client: Response 2

Data Synchronization
~~~~~~~~~~~~~~~~~~~~

.. diagram:: sequence
   :title: Data Sync
   
   Source -> Sync Service: Send Data
   Sync Service -> Validator: Validate
   Validator -> Sync Service: Valid
   Sync Service -> Target: Forward Data
   Target -> Sync Service: Acknowledge
   Sync Service -> Source: Confirm

Next Steps
----------

Now that you understand sequence diagrams, let's explore class diagrams for visualizing system architecture in the next chapter.