---
title: TypeScript React Component
date: 2026-01-19
tags: [typescript, react, frontend]
---

TypeScript React Component Example
===================================

This snippet shows a TypeScript React component with proper typing.

.. code-block:: typescript

    import React, { useState, useEffect } from 'react';
    
    interface User {
      id: number;
      name: string;
      email: string;
      role: 'admin' | 'user' | 'moderator';
    }
    
    interface UserListProps {
      initialUsers: User[];
      onUserSelect?: (user: User) => void;
    }
    
    const UserList: React.FC<UserListProps> = ({ 
      initialUsers, 
      onUserSelect 
    }) => {
      const [users, setUsers] = useState<User[]>(initialUsers);
      const [filter, setFilter] = useState<string>('');
      const [loading, setLoading] = useState<boolean>(false);
      
      useEffect(() => {
        // Fetch users from API
        const fetchUsers = async () => {
          setLoading(true);
          try {
            const response = await fetch('/api/users');
            const data = await response.json();
            setUsers(data);
          } catch (error) {
            console.error('Failed to fetch users:', error);
          } finally {
            setLoading(false);
          }
        };
        
        fetchUsers();
      }, []);
      
      const filteredUsers = users.filter(user =>
        user.name.toLowerCase().includes(filter.toLowerCase()) ||
        user.email.toLowerCase().includes(filter.toLowerCase())
      );
      
      const handleUserClick = (user: User) => {
        onUserSelect?.(user);
      };
      
      if (loading) {
        return <div>Loading users...</div>;
      }
      
      return (
        <div className="user-list">
          <input
            type="text"
            placeholder="Filter users..."
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="filter-input"
          />
          <ul className="user-items">
            {filteredUsers.map(user => (
              <li
                key={user.id}
                onClick={() => handleUserClick(user)}
                className={`user-item ${user.role}`}
              >
                <span className="user-name">{user.name}</span>
                <span className="user-email">{user.email}</span>
                <span className="user-role">{user.role}</span>
              </li>
            ))}
          </ul>
        </div>
      );
    };
    
    export default UserList;