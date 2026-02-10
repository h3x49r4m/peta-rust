Code Wrapping Feature Demo
============================

This snippet demonstrates the code block wrapping feature where long lines automatically wrap to fit the screen width without horizontal scrolling.

:date: 2026-02-10
:tags: code, css, feature, demo
:slug: code-wrapping-feature

Long URL Example
----------------

Here's a very long URL that would normally require horizontal scrolling but now wraps automatically:

.. code-block:: javascript

    const longUrl = "https://example.com/api/v1/users/12345/posts/67890/comments/98765/replies/54321/attachments/11111/download?token=abcdefghijklmnopqrstuvwxyz1234567890&format=pdf&version=2.0";

Complex Function Call
---------------------

This shows a complex function call with multiple parameters that wraps nicely:

.. code-block:: python

    def process_complex_data(
        user_id: int,
        session_token: str,
        preferences: dict,
        metadata: dict,
        callback_url: str,
        timeout: int = 30,
        retry_count: int = 3,
        debug_mode: bool = False
    ) -> dict:
        """Process complex data with many parameters that would cause line overflow."""
        result = {
            "status": "success",
            "data": preferences,
            "metadata": metadata,
            "processed_at": datetime.now().isoformat(),
            "processing_time_ms": 150.23,
            "user_identifier": user_id,
            "session_hash": hashlib.sha256(session_token.encode()).hexdigest()
        }
        return result

Very Long String
----------------

A very long string that demonstrates text wrapping:

.. code-block:: rust

    let long_message = "This is an extremely long string that demonstrates how code blocks handle text wrapping when the content exceeds the container width. The text will automatically wrap to the next line while maintaining proper alignment with line numbers.";

SQL Query Example
-----------------

A complex SQL query with long JOIN conditions:

.. code-block:: sql

    SELECT u.id, u.username, u.email, p.title, p.content, p.created_at, c.name as category_name, t.name as tag_name
    FROM users u
    INNER JOIN posts p ON u.id = p.user_id
    INNER JOIN categories c ON p.category_id = c.id
    LEFT JOIN post_tags pt ON p.id = pt.post_id
    LEFT JOIN tags t ON pt.tag_id = t.id
    WHERE u.status = 'active' AND p.published = true AND p.created_at >= '2024-01-01'
    ORDER BY p.created_at DESC LIMIT 50;

Complex HTML Element
--------------------

A long HTML element with many attributes:

.. code-block:: html

    <button type="submit" class="btn btn-primary btn-lg rounded-lg shadow-md hover:shadow-lg transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2" aria-label="Submit form" data-testid="submit-button" id="main-submit-btn" name="submit" value="submit">
        Submit Form
    </button>

Conclusion
----------

The code wrapping feature ensures that all code is readable without horizontal scrolling, while maintaining proper alignment between line numbers and code content using CSS Grid layout.