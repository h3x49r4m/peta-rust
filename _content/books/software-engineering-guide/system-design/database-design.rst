---
title: "Database Design"
date: 2026-02-07T00:00:00
tags: ["system-design", "database", "data-modeling"]
description: "Database design principles and strategies"
---

Database Design
===============

Database design is the process of producing a detailed data model of a database. It's a critical aspect of system design that affects performance, scalability, and maintainability.

SQL vs NoSQL
------------

**SQL Databases**

Relational databases with structured schemas.

.. code-block:: sql

   -- Create table with schema
   CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       email VARCHAR(255) UNIQUE NOT NULL,
       name VARCHAR(100) NOT NULL,
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
   );

   -- Join tables
   SELECT u.name, o.order_date
   FROM users u
   JOIN orders o ON u.id = o.user_id
   WHERE u.id = 1;

**NoSQL Databases**

Flexible schema databases for diverse data types.

.. code-block:: python

   # MongoDB document
   user = {
       '_id': ObjectId('...'),
       'email': 'user@example.com',
       'name': 'John Doe',
       'address': {
           'street': '123 Main St',
           'city': 'New York'
       },
       'orders': [
           {'order_id': 1, 'total': 100},
           {'order_id': 2, 'total': 200}
       ]
   }

   # Insert document
   db.users.insert_one(user)

When to Use Each
----------------

**Use SQL When:**

- Data structure is well-defined
- Data integrity is critical
- Complex queries are needed
- Transactions are required
- Strong consistency is needed

.. code-block:: sql

   -- Financial transaction example
   BEGIN TRANSACTION;
   UPDATE accounts SET balance = balance - 100 WHERE id = 1;
   UPDATE accounts SET balance = balance + 100 WHERE id = 2;
   COMMIT;

**Use NoSQL When:**

- Data structure is evolving
- Large scale is required
- Flexible schema is needed
- High write throughput is needed
- Geographic distribution is needed

.. code-block:: python

   # Social media feed example
   post = {
       'user_id': 123,
       'content': 'Hello world!',
       'likes': [],
       'comments': [],
       'timestamp': datetime.utcnow()
   }
   db.posts.insert_one(post)

Normalization vs Denormalization
---------------------------------

**Normalization**

Organize data to reduce redundancy.

.. code-block:: sql

   -- Normalized schema (3NF)
   CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       name VARCHAR(100) NOT NULL,
       email VARCHAR(255) UNIQUE NOT NULL
   );

   CREATE TABLE orders (
       id SERIAL PRIMARY KEY,
       user_id INTEGER REFERENCES users(id),
       order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       total DECIMAL(10, 2)
   );

   CREATE TABLE order_items (
       id SERIAL PRIMARY KEY,
       order_id INTEGER REFERENCES orders(id),
       product_id INTEGER REFERENCES products(id),
       quantity INTEGER,
       price DECIMAL(10, 2)
   );

**Denormalization**

Duplicate data to improve read performance.

.. code-block:: sql

   -- Denormalized schema
   CREATE TABLE orders (
       id SERIAL PRIMARY KEY,
       user_id INTEGER,
       user_name VARCHAR(100),  -- Denormalized
       user_email VARCHAR(255),  -- Denormalized
       order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       total DECIMAL(10, 2),
       items JSONB  -- Embedded items
   );

.. code-block:: python

   # Example of embedded data
   order = {
       'id': 1,
       'user_id': 123,
       'user_name': 'John Doe',
       'user_email': 'john@example.com',
       'order_date': '2026-02-07',
       'total': 150.00,
       'items': [
           {'product_id': 1, 'name': 'Product A', 'quantity': 2, 'price': 50.00},
           {'product_id': 2, 'name': 'Product B', 'quantity': 1, 'price': 50.00}
       ]
   }

Indexing Strategies
-------------------

**Single-Column Indexes**

Index on a single column for fast lookups.

.. code-block:: sql

   -- Create index
   CREATE INDEX idx_user_email ON users(email);

   -- Query uses index
   SELECT * FROM users WHERE email = 'user@example.com';

**Composite Indexes**

Index on multiple columns for complex queries.

.. code-block:: sql

   -- Create composite index
   CREATE INDEX idx_order_user_date ON orders(user_id, order_date);

   -- Query uses index
   SELECT * FROM orders
   WHERE user_id = 123
   ORDER BY order_date DESC;

**Covering Indexes**

Include columns in index to avoid table lookups.

.. code-block:: sql

   -- Create covering index
   CREATE INDEX idx_order_covering ON orders(user_id, order_date)
   INCLUDE (total);

   -- Query satisfied by index alone
   SELECT user_id, order_date, total
   FROM orders
   WHERE user_id = 123;

**Full-Text Indexes**

Index for text search capabilities.

.. code-block:: sql

   -- Create full-text index
   CREATE INDEX idx_product_fulltext ON products
   USING gin(to_tsvector('english', name || ' ' || description));

   -- Full-text search
   SELECT * FROM products
   WHERE to_tsvector('english', name || ' ' || description)
   @@ to_tsquery('english', 'laptop');

Partitioning
------------

**Range Partitioning**

Partition data by range of values.

.. code-block:: sql

   -- Create partitioned table
   CREATE TABLE orders (
       id SERIAL,
       order_date DATE,
       user_id INTEGER,
       total DECIMAL(10, 2)
   ) PARTITION BY RANGE (order_date);

   -- Create partitions
   CREATE TABLE orders_2026_01 PARTITION OF orders
   FOR VALUES FROM ('2026-01-01') TO ('2026-02-01');

   CREATE TABLE orders_2026_02 PARTITION OF orders
   FOR VALUES FROM ('2026-02-01') TO ('2026-03-01');

**Hash Partitioning**

Partition data by hash function.

.. code-block:: sql

   -- Create hash partitioned table
   CREATE TABLE users (
       id SERIAL,
       email VARCHAR(255),
       name VARCHAR(100)
   ) PARTITION BY HASH (id);

   -- Create partitions
   CREATE TABLE users_0 PARTITION OF users
   FOR VALUES WITH (MODULUS 4, REMAINDER 0);

   CREATE TABLE users_1 PARTITION OF users
   FOR VALUES WITH (MODULUS 4, REMAINDER 1);

**List Partitioning**

Partition data by discrete values.

.. code-block:: sql

   -- Create list partitioned table
   CREATE TABLE orders (
       id SERIAL,
       order_date DATE,
       status VARCHAR(20),
       total DECIMAL(10, 2)
   ) PARTITION BY LIST (status);

   -- Create partitions
   CREATE TABLE orders_pending PARTITION OF orders
   FOR VALUES IN ('pending');

   CREATE TABLE orders_completed PARTITION OF orders
   FOR VALUES IN ('completed');

Sharding
--------

**Horizontal Sharding**

Distribute rows across multiple databases.

.. code-block:: python

   class ShardManager:
       def __init__(self, shards):
           self.shards = shards

       def get_shard(self, user_id):
           # Hash user_id to determine shard
           shard_index = hash(user_id) % len(self.shards)
           return self.shards[shard_index]

       def query_user(self, user_id):
           shard = self.get_shard(user_id)
           return shard.query("SELECT * FROM users WHERE id = ?", user_id)

**Vertical Sharding**

Distribute columns across multiple databases.

.. code-block:: python

   # Profile database (user profile data)
   profile_db = Database('profiles')
   profile_db.execute("""
       CREATE TABLE users (
           id INTEGER PRIMARY KEY,
           name VARCHAR(100),
           email VARCHAR(255),
           created_at TIMESTAMP
       )
   """)

   # Activity database (user activity data)
   activity_db = Database('activities')
   activity_db.execute("""
       CREATE TABLE user_activities (
           id INTEGER PRIMARY KEY,
           user_id INTEGER,
           activity_type VARCHAR(50),
           activity_data JSON,
           created_at TIMESTAMP
       )
   """)

Caching Strategies
------------------

**Read-Through Cache**

Cache misses are populated from database.

.. code-block:: python

   class ReadThroughCache:
       def __init__(self, cache, database):
           self.cache = cache
           self.database = database

       def get(self, key):
           # Try cache first
           value = self.cache.get(key)
           if value:
               return value

           # Cache miss, fetch from database
           value = self.database.get(key)
           if value:
               self.cache.set(key, value)
           return value

**Write-Through Cache**

Writes go to both cache and database.

.. code-block:: python

   class WriteThroughCache:
       def __init__(self, cache, database):
           self.cache = cache
           self.database = database

       def set(self, key, value):
           # Write to cache
           self.cache.set(key, value)

           # Write to database
           self.database.set(key, value)

**Write-Behind Cache**

Writes go to cache, then asynchronously to database.

.. code-block:: python

   class WriteBehindCache:
       def __init__(self, cache, database):
           self.cache = cache
           self.database = database

       def set(self, key, value):
           # Write to cache immediately
           self.cache.set(key, value)

           # Write to database asynchronously
           Thread(target=self.database.set, args=(key, value)).start()

Data Modeling Best Practices
----------------------------

**Use Appropriate Data Types**

Choose data types that match your data.

.. code-block:: sql

   -- Good: appropriate data types
   CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       email VARCHAR(255) NOT NULL,
       age INTEGER,
       balance DECIMAL(10, 2),
       is_active BOOLEAN,
       created_at TIMESTAMP
   );

   -- Bad: inappropriate data types
   CREATE TABLE users (
       id VARCHAR(50),
       email TEXT,
       age VARCHAR(3),
       balance VARCHAR(20),
       is_active INTEGER,
       created_at VARCHAR(20)
   );

**Define Constraints**

Use constraints to ensure data integrity.

.. code-block:: sql

   -- Add constraints
   ALTER TABLE users
   ADD CONSTRAINT chk_age CHECK (age >= 0 AND age <= 150);

   ALTER TABLE orders
   ADD CONSTRAINT chk_total CHECK (total >= 0);

   ALTER TABLE orders
   ADD CONSTRAINT fk_user
   FOREIGN KEY (user_id) REFERENCES users(id);

**Use Transactions**

Group related operations in transactions.

.. code-block:: python

   def transfer_money(from_account, to_account, amount):
       try:
           db.begin_transaction()

           # Debit from account
           db.execute(
               "UPDATE accounts SET balance = balance - ? WHERE id = ?",
               (amount, from_account)
           )

           # Credit to account
           db.execute(
               "UPDATE accounts SET balance = balance + ? WHERE id = ?",
               (amount, to_account)
           )

           db.commit_transaction()
       except Exception as e:
           db.rollback_transaction()
           raise

**Plan for Growth**

Design for future scale.

.. code-block:: python

   # Use sharding key from the beginning
   class UserService:
       def get_user(self, user_id):
           shard = self.shard_manager.get_shard(user_id)
           return shard.query("SELECT * FROM users WHERE id = ?", user_id)

Common Pitfalls
---------------

**Over-Normalization**

Too much normalization hurts performance.

**Under-Normalization**

Too little normalization causes data inconsistencies.

**Ignoring Indexes**

Missing indexes lead to slow queries.

**Poor Schema Design**

Bad schema is hard to change later.

**No Backup Strategy**

Always have a backup and recovery plan.