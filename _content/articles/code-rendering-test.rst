---
title: Code Rendering Test Article
date: 2026-01-19
tags: [test, code-blocks, rendering]
---

This article tests the rendering of various code blocks in article pages.

Python Example
--------------

Here's a Python function that calculates factorial:

.. code-block:: python

    def factorial(n):
        """Calculate the factorial of a number."""
        if n < 0:
            raise ValueError("Factorial is not defined for negative numbers")
        elif n == 0:
            return 1
        else:
            return n * factorial(n - 1)
    
    # Test the function
    for i in range(6):
        print(f"{i}! = {factorial(i)}")

JavaScript/TypeScript Example
-----------------------------

Here's a TypeScript class for a simple counter:

.. code-block:: typescript

    class Counter {
        private value: number = 0;
        
        constructor(initialValue: number = 0) {
            this.value = initialValue;
        }
        
        increment(): void {
            this.value++;
        }
        
        decrement(): void {
            this.value--;
        }
        
        getValue(): number {
            return this.value;
        }
        
        reset(): void {
            this.value = 0;
        }
    }
    
    // Usage example
    const counter = new Counter(10);
    console.log(`Initial value: ${counter.getValue()}`);
    counter.increment();
    console.log(`After increment: ${counter.getValue()}`);

Rust Example
------------

Here's a Rust implementation of a simple struct:

.. code-block:: rust

    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    fn main() {
        let rect1 = Rectangle::new(30, 50);
        let rect2 = Rectangle::new(10, 40);
        
        println!("Rectangle 1 area: {}", rect1.area());
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    }

SQL Example
-----------

Here's a SQL query to find the top 5 users by post count:

.. code-block:: sql

    SELECT 
        u.id,
        u.username,
        u.email,
        COUNT(p.id) as post_count
    FROM 
        users u
    LEFT JOIN 
        posts p ON u.id = p.user_id
    WHERE 
        u.created_at >= '2023-01-01'
    GROUP BY 
        u.id, u.username, u.email
    ORDER BY 
        post_count DESC
    LIMIT 5;

Inline Code Examples
--------------------

You can also use inline code like `console.log("Hello")` or `def hello(): pass`.

Math with Code
--------------

Here's a mathematical formula with code:

The formula for compound interest is:
$$A = P(1 + r)^t$$

Where:
- A = final amount
- P = principal amount  
- r = annual interest rate
- t = time in years

Here's how to implement it in Python:

.. code-block:: python

    def compound_interest(principal, rate, time):
        """Calculate compound interest."""
        return principal * (1 + rate) ** time
    
    # Example: $1000 at 5% interest for 3 years
    result = compound_interest(1000, 0.05, 3)
    print(f"After 3 years: ${result:.2f}")

Conclusion
----------

This article demonstrates that code blocks are properly rendered in articles with syntax highlighting for various programming languages.
