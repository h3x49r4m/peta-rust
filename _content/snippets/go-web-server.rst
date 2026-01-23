---
title: Go Web Server
date: 2026-01-19
tags: [go, web-server, backend]
---

Go Web Server Example
=====================

This snippet shows a simple REST API server in Go with middleware.

.. code-block:: go

    package main
    
    import (
        "context"
        "encoding/json"
        "fmt"
        "log"
        "net/http"
        "strconv"
        "time"
    
        "github.com/gorilla/mux"
    )
    
    type User struct {
        ID    int    `json:"id"`
        Name  string `json:"name"`
        Email string `json:"email"`
    }
    
    type Server struct {
        router *mux.Router
        users  []User
    }
    
    func NewServer() *Server {
        s := &Server{
            router: mux.NewRouter(),
            users: []User{
                {ID: 1, Name: "John Doe", Email: "john@example.com"},
                {ID: 2, Name: "Jane Smith", Email: "jane@example.com"},
            },
        }
        s.setupRoutes()
        return s
    }
    
    func (s *Server) setupRoutes() {
        // Middleware
        s.router.Use(s.loggingMiddleware)
        s.router.Use(s.corsMiddleware)
        
        // Routes
        s.router.HandleFunc("/api/users", s.getUsers).Methods("GET")
        s.router.HandleFunc("/api/users/{id}", s.getUser).Methods("GET")
        s.router.HandleFunc("/api/users", s.createUser).Methods("POST")
        s.router.HandleFunc("/api/users/{id}", s.updateUser).Methods("PUT")
        s.router.HandleFunc("/api/users/{id}", s.deleteUser).Methods("DELETE")
    }
    
    func (s *Server) loggingMiddleware(next http.Handler) http.Handler {
        return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
            start := time.Now()
            next.ServeHTTP(w, r)
            log.Printf(
                "%s %s %s %v",
                r.Method,
                r.RequestURI,
                r.RemoteAddr,
                time.Since(start),
            )
        })
    }
    
    func (s *Server) corsMiddleware(next http.Handler) http.Handler {
        return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
            w.Header().Set("Access-Control-Allow-Origin", "*")
            w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
            w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
            
            if r.Method == "OPTIONS" {
                w.WriteHeader(http.StatusOK)
                return
            }
            
            next.ServeHTTP(w, r)
        })
    }
    
    func (s *Server) getUsers(w http.ResponseWriter, r *http.Request) {
        w.Header().Set("Content-Type", "application/json")
        json.NewEncoder(w).Encode(s.users)
    }
    
    func (s *Server) getUser(w http.ResponseWriter, r *http.Request) {
        vars := mux.Vars(r)
        id, _ := strconv.Atoi(vars["id"])
        
        for _, user := range s.users {
            if user.ID == id {
                w.Header().Set("Content-Type", "application/json")
                json.NewEncoder(w).Encode(user)
                return
            }
        }
        
        http.NotFound(w, r)
    }
    
    func (s *Server) createUser(w http.ResponseWriter, r *http.Request) {
        var user User
        if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
            http.Error(w, err.Error(), http.StatusBadRequest)
            return
        }
        
        user.ID = len(s.users) + 1
        s.users = append(s.users, user)
        
        w.Header().Set("Content-Type", "application/json")
        w.WriteHeader(http.StatusCreated)
        json.NewEncoder(w).Encode(user)
    }
    
    func (s *Server) updateUser(w http.ResponseWriter, r *http.Request) {
        vars := mux.Vars(r)
        id, _ := strconv.Atoi(vars["id"])
        
        var updatedUser User
        if err := json.NewDecoder(r.Body).Decode(&updatedUser); err != nil {
            http.Error(w, err.Error(), http.StatusBadRequest)
            return
        }
        
        for i, user := range s.users {
            if user.ID == id {
                updatedUser.ID = id
                s.users[i] = updatedUser
                w.Header().Set("Content-Type", "application/json")
                json.NewEncoder(w).Encode(updatedUser)
                return
            }
        }
        
        http.NotFound(w, r)
    }
    
    func (s *Server) deleteUser(w http.ResponseWriter, r *http.Request) {
        vars := mux.Vars(r)
        id, _ := strconv.Atoi(vars["id"])
        
        for i, user := range s.users {
            if user.ID == id {
                s.users = append(s.users[:i], s.users[i+1:]...)
                w.WriteHeader(http.StatusNoContent)
                return
            }
        }
        
        http.NotFound(w, r)
    }
    
    func main() {
        server := NewServer()
        
        fmt.Println("Server starting on port 8080...")
        log.Fatal(http.ListenAndServe(":8080", server.router))
    }