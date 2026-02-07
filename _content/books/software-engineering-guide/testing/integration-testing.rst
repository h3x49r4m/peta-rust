---
title: "Integration Testing"
date: 2026-02-07T00:00:00
tags: ["testing", "integration-testing", "api"]
description: "Testing interactions between components"
---

Integration Testing
===================

Integration tests verify that different modules or services work together correctly. They fill the gap between unit tests and end-to-end tests.

What is Integration Testing?
----------------------------

Integration testing focuses on:

- API contracts
- Database interactions
- External service integrations
- Component interactions

Unlike unit tests, integration tests may use real dependencies or test doubles that simulate realistic behavior.

When to Use Integration Tests
------------------------------

- When you need to test how components interact
- When you need to verify database queries work correctly
- When you need to test API contracts
- When you need to test external service integrations

Database Integration Testing
----------------------------

Testing database interactions is crucial for applications that persist data.

.. code-block:: python

   import pytest
   from sqlalchemy import create_engine
   from sqlalchemy.orm import sessionmaker

   @pytest.fixture
   def test_db():
       # Create an in-memory database for testing
       engine = create_engine("sqlite:///:memory:")
       Base.metadata.create_all(engine)
       Session = sessionmaker(bind=engine)
       session = Session()
       yield session
       session.close()

   def test_create_user(test_db):
       user = User(name="John", email="john@example.com")
       test_db.add(user)
       test_db.commit()

       retrieved_user = test_db.query(User).filter_by(email="john@example.com").first()
       assert retrieved_user is not None
       assert retrieved_user.name == "John"

API Integration Testing
------------------------

Testing API contracts ensures that your API behaves as expected.

.. code-block:: python

   from fastapi.testclient import TestClient

   @pytest.fixture
   def client():
       return TestClient(app)

   def test_create_user_api(client):
       response = client.post(
           "/users",
           json={"name": "John", "email": "john@example.com"}
       )

       assert response.status_code == 201
       data = response.json()
       assert data["name"] == "John"
       assert data["email"] == "john@example.com"
       assert "id" in data

   def test_get_user_api(client, test_db):
       user = User(name="John", email="john@example.com")
       test_db.add(user)
       test_db.commit()

       response = client.get(f"/users/{user.id}")

       assert response.status_code == 200
       data = response.json()
       assert data["name"] == "John"

External Service Testing
------------------------

When testing integrations with external services, use mocking to avoid dependencies on external systems.

.. code-block:: python

   from unittest.mock import patch, MagicMock

   @patch('app.services.payment_gateway.PaymentGateway')
   def test_process_payment_success(mock_gateway):
       # Set up the mock
       mock_gateway.charge.return_value = {"status": "success", "transaction_id": "12345"}

       # Test the payment processing
       result = process_payment(user_id=1, amount=100)

       # Verify the result
       assert result["status"] == "success"
       assert result["transaction_id"] == "12345"

       # Verify the gateway was called correctly
       mock_gateway.charge.assert_called_once_with(amount=100, user_id=1)

Test Containers
---------------

For more realistic integration tests, you can use Docker containers to spin up real services like databases, message brokers, or other dependencies.

.. code-block:: python

   import pytest
   from testcontainers.postgres import PostgresContainer

   @pytest.fixture(scope="session")
   def postgres_container():
       with PostgresContainer("postgres:15") as postgres:
           yield postgres.get_connection_url()

   @pytest.fixture
   def db_session(postgres_container):
       engine = create_engine(postgres_container)
       Base.metadata.create_all(engine)
       Session = sessionmaker(bind=engine)
       session = Session()
       yield session
       session.close()

Integration Test Strategies
----------------------------

**Contract Testing**

Contract testing ensures that services agree on their API contracts.

.. code-block:: python

   def test_user_service_contract():
       response = requests.get("http://localhost:8000/users/1")
       assert response.status_code == 200

       schema = {
           "type": "object",
           "properties": {
               "id": {"type": "integer"},
               "name": {"type": "string"},
               "email": {"type": "string", "format": "email"}
           },
           "required": ["id", "name", "email"]
       }

       validate(response.json(), schema)

**State Testing**

State testing verifies that the system maintains correct state across operations.

.. code-block:: python

   def test_order_state_transitions(db_session):
       # Create order
       order = Order(status="pending")
       db_session.add(order)
       db_session.commit()

       # Update status
       order.status = "processing"
       db_session.commit()

       # Verify state transition
       updated_order = db_session.query(Order).get(order.id)
       assert updated_order.status == "processing"

Best Practices
--------------

**Use Test Databases**

Use a separate database for testing to avoid affecting production data.

**Clean Up After Tests**

Ensure each test cleans up after itself to avoid interference between tests.

.. code-block:: python

   @pytest.fixture(autouse=True)
   def cleanup_db(db_session):
       yield
       db_session.rollback()
       db_session.query(Order).delete()
       db_session.commit()

**Test Realistic Scenarios**

Test realistic scenarios that your application will encounter in production.

**Run Tests in Isolation**

Ensure tests can run in any order and don't depend on each other.

**Mock External Services**

Use mocks for external services to avoid dependencies on third-party systems.

Common Pitfalls
---------------

**Testing Too Much**

Don't test what unit tests should cover. Focus on interactions between components.

**Brittle Tests**

Avoid tests that break when implementation details change.

**Slow Tests**

Integration tests can be slow. Keep them focused and run them separately from unit tests.

**Flaky Tests**

Tests that sometimes pass and sometimes fail are worse than no tests. Ensure tests are deterministic.