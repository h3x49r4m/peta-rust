---
title: Code Block Showcase
date: 2026-01-20
tags: [showcase, code-blocks, syntax-highlighting]
author: Peta Team
description: A comprehensive showcase of code blocks with various programming languages and syntax highlighting
github_url: https://github.com/h3x49r4m/peta
demo_url: https://peta.example.com
---

Code Block Showcase
===================

This project demonstrates the beautiful and consistent code block rendering across all pages of the Peta website. It showcases the One Dark Pro theme with line numbers and syntax highlighting for multiple programming languages.

Features
--------

- **Consistent Styling**: All code blocks use the same One Dark Pro theme
- **Line Numbers**: Every code block displays line numbers for better readability
- **Syntax Highlighting**: Support for 17+ programming languages
- **Copy Functionality**: One-click code copying with visual feedback
- **Responsive Design**: Code blocks adapt to different screen sizes

Python Example
--------------

Here's a Python implementation of a binary search tree:

.. code-block:: python

    class TreeNode:
        def __init__(self, val=0, left=None, right=None):
            self.val = val
            self.left = left
            self.right = right
    
    class BinarySearchTree:
        def __init__(self):
            self.root = None
        
        def insert(self, val):
            if not self.root:
                self.root = TreeNode(val)
                return
            
            current = self.root
            while True:
                if val < current.val:
                    if current.left:
                        current = current.left
                    else:
                        current.left = TreeNode(val)
                        break
                elif val > current.val:
                    if current.right:
                        current = current.right
                    else:
                        current.right = TreeNode(val)
                        break
                else:
                    break  # Value already exists
        
        def search(self, val):
            current = self.root
            while current:
                if val == current.val:
                    return True
                elif val < current.val:
                    current = current.left
                else:
                    current = current.right
            return False
        
        def inorder_traversal(self):
            result = []
            
            def traverse(node):
                if node:
                    traverse(node.left)
                    result.append(node.val)
                    traverse(node.right)
            
            traverse(self.root)
            return result

TypeScript Example
-----------------

TypeScript interface for a user management system:

.. code-block:: typescript

    interface User {
        id: number;
        username: string;
        email: string;
        firstName: string;
        lastName: string;
        role: UserRole;
        createdAt: Date;
        lastLogin?: Date;
        isActive: boolean;
    }
    
    enum UserRole {
        ADMIN = 'admin',
        MODERATOR = 'moderator',
        USER = 'user',
        GUEST = 'guest'
    }
    
    class UserService {
        private users: Map<number, User> = new Map();
        private nextId: number = 1;
        
        createUser(userData: Omit<User, 'id' | 'createdAt'>): User {
            const user: User = {
                id: this.nextId++,
                ...userData,
                createdAt: new Date()
            };
            
            this.users.set(user.id, user);
            return user;
        }
        
        getUserById(id: number): User | undefined {
            return this.users.get(id);
        }
        
        updateUser(id: number, updates: Partial<User>): User | null {
            const user = this.users.get(id);
            if (!user) return null;
            
            const updatedUser = { ...user, ...updates };
            this.users.set(id, updatedUser);
            return updatedUser;
        }
        
        deleteUser(id: number): boolean {
            return this.users.delete(id);
        }
        
        getAllUsers(): User[] {
            return Array.from(this.users.values());
        }
    }

Rust Example
------------

Rust implementation of a thread-safe counter using Arc and Mutex:

.. code-block:: rust

    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    
    struct Counter {
        value: Mutex<i32>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                value: Mutex::new(0),
            }
        }
        
        fn increment(&self) {
            let mut num = self.value.lock().unwrap();
            *num += 1;
        }
        
        fn get(&self) -> i32 {
            let num = self.value.lock().unwrap();
            *num
        }
    }
    
    fn main() {
        let counter = Arc::new(Counter::new());
        let mut handles = vec![];
        
        // Spawn 10 threads that each increment the counter 1000 times
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    counter_clone.increment();
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        println!("Final counter value: {}", counter.get());
    }

SQL Example
-----------

Complex SQL query for analytics dashboard:

.. code-block:: sql

    WITH monthly_revenue AS (
        SELECT 
            DATE_TRUNC('month', order_date) AS month,
            SUM(total_amount) AS revenue,
            COUNT(*) AS order_count
        FROM orders
        WHERE order_date >= CURRENT_DATE - INTERVAL '12 months'
        GROUP BY DATE_TRUNC('month', order_date)
    ),
    
    customer_segments AS (
        SELECT 
            customer_id,
            CASE 
                WHEN total_spent > 1000 THEN 'High Value'
                WHEN total_spent > 500 THEN 'Medium Value'
                ELSE 'Low Value'
            END AS segment
        FROM (
            SELECT 
                customer_id,
                SUM(total_amount) AS total_spent
            FROM orders
            GROUP BY customer_id
        ) customer_totals
    ),
    
    top_products AS (
        SELECT 
            product_id,
            product_name,
            SUM(quantity) AS total_sold,
            SUM(total_amount) AS product_revenue
        FROM order_items
        GROUP BY product_id, product_name
        ORDER BY product_revenue DESC
        LIMIT 10
    )
    
    SELECT 
        mr.month,
        mr.revenue,
        mr.order_count,
        COUNT(cs.customer_id) AS customer_count,
        SUM(CASE WHEN cs.segment = 'High Value' THEN 1 ELSE 0 END) AS high_value_customers,
        STRING_AGG(tp.product_name, ', ') AS top_products
    FROM monthly_revenue mr
    LEFT JOIN orders o ON DATE_TRUNC('month', o.order_date) = mr.month
    LEFT JOIN customer_segments cs ON o.customer_id = cs.customer_id
    LEFT JOIN top_products tp ON TRUE
    GROUP BY mr.month, mr.revenue, mr.order_count
    ORDER BY mr.month DESC;

Go Example
----------

Go implementation of a concurrent web server with middleware:

.. code-block:: go

    package main
    
    import (
        "fmt"
        "log"
        "net/http"
        "sync"
        "time"
    )
    
    type Middleware func(http.HandlerFunc) http.HandlerFunc
    
    func logging(next http.HandlerFunc) http.HandlerFunc {
        return func(w http.ResponseWriter, r *http.Request) {
            start := time.Now()
            log.Printf("Started %s %s", r.Method, r.URL.Path)
            
            next.ServeHTTP(w, r)
            
            log.Printf("Completed %s %s in %v", r.Method, r.URL.Path, time.Since(start))
        }
    }
    
    func method(method string) Middleware {
        return func(next http.HandlerFunc) http.HandlerFunc {
            return func(w http.ResponseWriter, r *http.Request) {
                if r.Method != method {
                    http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
                    return
                }
                next(w, r)
            }
        }
    }
    
    func chain(f http.HandlerFunc, middlewares ...Middleware) http.HandlerFunc {
        for _, m := range middlewares {
            f = m(f)
        }
        return f
    }
    
    func home(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "Welcome to the Go Web Server!")
    }
    
    func health(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        fmt.Fprintf(w, `{"status": "healthy", "timestamp": "%s"}`, time.Now().Format(time.RFC3339))
    }
    
    type Server struct {
        server *http.Server
        wg     sync.WaitGroup
    }
    
    func NewServer() *Server {
        mux := http.NewServeMux()
        
        // Register handlers with middleware
        mux.HandleFunc("/", chain(home, logging))
        mux.HandleFunc("/health", chain(health, logging, method("GET")))
        
        return &Server{
            server: &http.Server{
                Addr:    ":8080",
                Handler: mux,
            },
        }
    }
    
    func (s *Server) Start() error {
        s.wg.Add(1)
        go func() {
            defer s.wg.Done()
            log.Println("Server starting on :8080")
            if err := s.server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
                log.Printf("Server error: %v", err)
            }
        }()
        return nil
    }
    
    func (s *Server) Stop() error {
        ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
        defer cancel()
        
        log.Println("Server shutting down...")
        if err := s.server.Shutdown(ctx); err != nil {
            return err
        }
        
        s.wg.Wait()
        return nil
    }

C++ Example
-----------

C++ implementation of a template-based smart pointer:

.. code-block:: cpp

    #include <iostream>
    #include <memory>
    #include <stdexcept>
    
    template <typename T>
    class SmartPointer {
    private:
        T* ptr;
        size_t* refCount;
    
    public:
        // Constructor
        explicit SmartPointer(T* p = nullptr) : ptr(p), refCount(new size_t(1)) {}
        
        // Copy constructor
        SmartPointer(const SmartPointer& other) : ptr(other.ptr), refCount(other.refCount) {
            (*refCount)++;
        }
        
        // Destructor
        ~SmartPointer() {
            (*refCount)--;
            if (*refCount == 0) {
                delete ptr;
                delete refCount;
            }
        }
        
        // Assignment operator
        SmartPointer& operator=(const SmartPointer& other) {
            if (this != &other) {
                (*refCount)--;
                if (*refCount == 0) {
                    delete ptr;
                    delete refCount;
                }
                
                ptr = other.ptr;
                refCount = other.refCount;
                (*refCount)++;
            }
            return *this;
        }
        
        // Dereference operators
        T& operator*() const {
            if (!ptr) {
                throw std::runtime_error("Dereferencing null pointer");
            }
            return *ptr;
        }
        
        T* operator->() const {
            return ptr;
        }
        
        // Get raw pointer
        T* get() const {
            return ptr;
        }
        
        // Check if pointer is null
        explicit operator bool() const {
            return ptr != nullptr;
        }
        
        // Reset pointer
        void reset(T* p = nullptr) {
            (*refCount)--;
            if (*refCount == 0) {
                delete ptr;
                delete refCount;
            }
            
            ptr = p;
            refCount = new size_t(1);
        }
        
        // Get reference count
        size_t use_count() const {
            return *refCount;
        }
    };
    
    // Usage example
    int main() {
        SmartPointer<int> sp1(new int(42));
        SmartPointer<int> sp2 = sp1;
        
        std::cout << "Value: " << *sp1 << std::endl;
        std::cout << "Ref count: " << sp1.use_count() << std::endl;
        
        {
            SmartPointer<int> sp3 = sp1;
            std::cout << "Ref count after sp3: " << sp1.use_count() << std::endl;
        }
        
        std::cout << "Ref count after sp3 scope: " << sp1.use_count() << std::endl;
        
        return 0;
    }

Conclusion
----------

This showcase demonstrates that the Peta website now provides:

1. **Beautiful Code Blocks**: Consistent One Dark Pro theme across all pages
2. **Line Numbers**: Every code block displays line numbers for easy reference
3. **Syntax Highlighting**: Proper highlighting for all major programming languages
4. **Responsive Design**: Code blocks work perfectly on all screen sizes
5. **Accessibility**: Copy buttons and proper contrast for readability

The code blocks are now fully functional and consistent across articles, books, snippets, and projects pages! 🎉