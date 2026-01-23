---
title: Rust Concurrent Programming
date: 2026-01-19
tags: [rust, concurrent, systems-programming]
---

Rust Concurrent Programming Example
===================================

This snippet demonstrates concurrent programming in Rust using threads and channels.

.. code-block:: rust

    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;
    
    fn main() {
        println!("=== Rust Concurrent Programming Example ===\n");
        
        // Example 1: Basic thread spawning
        basic_thread_example();
        
        // Example 2: Shared state with Arc<Mutex<>>
        shared_state_example();
        
        // Example 3: Channel communication
        channel_example();
    }
    
    fn basic_thread_example() {
        println!("1. Basic Thread Example:");
        
        let handles: Vec<_> = (0..5)
            .map(|i| {
                thread::spawn(move || {
                    println!("Thread {} started", i);
                    thread::sleep(Duration::from_millis(100 * i));
                    println!("Thread {} finished", i);
                    i * 2
                })
            })
            .collect();
        
        for handle in handles {
            let result = handle.join().unwrap();
            println!("Thread result: {}", result);
        }
        println!();
    }
    
    fn shared_state_example() {
        println!("2. Shared State Example:");
        
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        println!("Result: {}", *counter.lock().unwrap());
        println!();
    }
    
    fn channel_example() {
        println!("3. Channel Communication Example:");
        
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        for received in rx {
            println!("Got: {}", received);
        }
        println!();
    }