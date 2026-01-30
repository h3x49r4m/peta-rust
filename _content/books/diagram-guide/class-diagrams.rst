Class Diagrams
==============

Class diagrams visualize the structure and relationships between components in a system. They're perfect for showing architecture, dependencies, and data models.

Basic Syntax
------------

The fundamental syntax for class diagrams shows relationships between entities:

.. diagram:: class-diagram
   :title: Basic Class Diagram
   
   Entity A |+| Entity B

This creates a composition relationship between two entities.

Relationship Syntax
-------------------

Relationship Types
~~~~~~~~~~~~~~~~~~

Peta supports two main relationship types:

1. **Composition** (`|+|`): Strong ownership, lifecycle dependency
2. **Aggregation** (`|o|`): Weak ownership, shared reference

Composition
~~~~~~~~~~~

Use `|+|` for strong ownership relationships:

.. diagram:: class-diagram
   
   Order |+| OrderItem

This means an Order owns OrderItems, and OrderItems cannot exist without the Order.

Aggregation
~~~~~~~~~~~

Use `|o|` for weak ownership relationships:

.. diagram:: class-diagram
   
   User |o| Address

This means a User has an Address, but the Address can exist independently.

Creating Relationships
----------------------

Single Relationship
~~~~~~~~~~~~~~~~~~~

Connect two entities:

.. diagram:: class-diagram
   
   User |+| Profile

Multiple Relationships
~~~~~~~~~~~~~~~~~~~~~~

An entity can have multiple relationships:

.. diagram:: class-diagram
   
   User |+| Post
   User |+| Comment
   User |o| Settings

Complex Architectures
---------------------

System Architecture
~~~~~~~~~~~~~~~~~~~

Show complete system structure:

.. diagram:: class-diagram
   :title: System Architecture
   
   Application |+| Controller
   Application |+| Service
   Service |+| Repository
   Repository |o| Database
   Service |o| Cache
   Controller |+| Validator

Data Model
~~~~~~~~~~

Represent database schema:

.. diagram:: class-diagram
   :title: Data Model
   
   User |+| Profile
   User |+| Order
   Order |+| OrderItem
   Order |o| Payment
   Product |+| OrderItem
   Category |+| Product

Practical Examples
------------------

Blog System
~~~~~~~~~~~

.. diagram:: class-diagram
   :title: Blog System Architecture
   
   User |+| Post
   User |+| Comment
   Post |+| Comment
   Post |+| Category
   Post |+| Tag
   User |+| Subscription
   Subscription |o| Category

E-commerce Platform
~~~~~~~~~~~~~~~~~~

.. diagram:: class-diagram
   :title: E-commerce Platform
   
   Customer |+| Order
   Customer |+| Address
   Customer |o| Wishlist
   Order |+| OrderItem
   Order |o| Payment
   Product |+| OrderItem
   Product |+| Review
   Category |+| Product
   Inventory |o| Product

Social Media
~~~~~~~~~~~~

.. diagram:: class-diagram
   :title: Social Media Platform
   
   User |+| Post
   User |+| Comment
   User |+| Like
   User |+| Follow
   Post |+| Comment
   Post |+| Like
   Post |o| Media
   Comment |+| Like
   Comment |o| Media
   Group |+| Post
   Group |+| User

Content Management System
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. diagram:: class-diagram
   :title: CMS Architecture
   
   Site |+| Page
   Site |+| User
   User |+| Post
   Page |o| Template
   Post |+| Content
   Content |o| Media
   Media |o| Storage
   User |o| Role
   Role |+| Permission

Tips and Best Practices
-----------------------

1. **Clear entity names**: Use descriptive, concise names
2. **Appropriate relationships**: Choose composition vs aggregation carefully
3. **Logical grouping**: Group related entities together
4. **Consistent direction**: Generally show dependencies from dependent to provider
5. **Avoid clutter**: Don't overcomplicate the diagram

Relationship Guidelines
-----------------------

When to Use Composition
~~~~~~~~~~~~~~~~~~~~~~~

Use `|+|` when:
- The child cannot exist without the parent
- The child's lifecycle is managed by the parent
- Deleting the parent should delete the child

Examples:
- Order and OrderItems
- Document and Pages
- User and UserProfile

When to Use Aggregation
~~~~~~~~~~~~~~~~~~~~~~~~

Use `|o|` when:
- The child can exist independently
- The child is shared among multiple parents
- Deleting the parent should not delete the child

Examples:
- User and Address
- Course and Student
- Department and Employee

Common Patterns
---------------

Repository Pattern
~~~~~~~~~~~~~~~~~~

.. diagram:: class-diagram
   :title: Repository Pattern
   
   Service |+| Repository
   Repository |o| Database
   Service |o| Cache
   Controller |+| Service

Service Layer
~~~~~~~~~~~~~

.. diagram:: class-diagram
   :title: Service Layer Architecture
   
   Controller |+| Service
   Service |+| Repository
   Repository |o| Database
   Service |o| External API
   Controller |+| Validator

Module System
~~~~~~~~~~~~~

.. diagram:: class-diagram
   :title: Module System
   
   Core |+| Auth Module
   Core |+| User Module
   Core |+| Content Module
   Auth Module |+| User Module
   Content Module |+| User Module

Next Steps
----------

Now that you understand class diagrams, let's explore state diagrams for visualizing state machines in the next chapter.