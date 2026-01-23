---
title: SQL Queries
date: 2026-01-19
tags: [sql, database, queries]
---

SQL Queries Example
===================

This snippet demonstrates various SQL queries for data manipulation.

.. code-block:: sql

    -- Create tables
    CREATE TABLE employees (
        id INT PRIMARY KEY AUTO_INCREMENT,
        first_name VARCHAR(50) NOT NULL,
        last_name VARCHAR(50) NOT NULL,
        email VARCHAR(100) UNIQUE NOT NULL,
        department_id INT,
        salary DECIMAL(10, 2),
        hire_date DATE,
        is_active BOOLEAN DEFAULT TRUE,
        FOREIGN KEY (department_id) REFERENCES departments(id)
    );
    
    CREATE TABLE departments (
        id INT PRIMARY KEY AUTO_INCREMENT,
        name VARCHAR(50) NOT NULL,
        location VARCHAR(100),
        manager_id INT,
        budget DECIMAL(12, 2)
    );
    
    CREATE TABLE projects (
        id INT PRIMARY KEY AUTO_INCREMENT,
        name VARCHAR(100) NOT NULL,
        start_date DATE,
        end_date DATE,
        budget DECIMAL(12, 2),
        status ENUM('Planning', 'In Progress', 'Completed', 'On Hold') DEFAULT 'Planning'
    );
    
    CREATE TABLE employee_projects (
        employee_id INT,
        project_id INT,
        role VARCHAR(50),
        hours_worked INT DEFAULT 0,
        PRIMARY KEY (employee_id, project_id),
        FOREIGN KEY (employee_id) REFERENCES employees(id),
        FOREIGN KEY (project_id) REFERENCES projects(id)
    );
    
    -- Insert sample data
    INSERT INTO departments (name, location, budget) VALUES
    ('Engineering', 'San Francisco', 500000.00),
    ('Marketing', 'New York', 300000.00),
    ('Sales', 'Chicago', 400000.00),
    ('HR', 'Remote', 200000.00);
    
    INSERT INTO employees (first_name, last_name, email, department_id, salary, hire_date) VALUES
    ('John', 'Doe', 'john.doe@company.com', 1, 95000.00, '2022-01-15'),
    ('Jane', 'Smith', 'jane.smith@company.com', 1, 105000.00, '2021-03-20'),
    ('Mike', 'Johnson', 'mike.johnson@company.com', 2, 75000.00, '2022-06-10'),
    ('Sarah', 'Williams', 'sarah.williams@company.com', 3, 85000.00, '2020-11-05'),
    ('David', 'Brown', 'david.brown@company.com', 4, 65000.00, '2023-02-28');
    
    -- Basic SELECT queries
    -- 1. Select all employees
    SELECT * FROM employees;
    
    -- 2. Select specific columns
    SELECT first_name, last_name, email, salary FROM employees;
    
    -- 3. Filter with WHERE clause
    SELECT * FROM employees WHERE salary > 80000;
    
    -- 4. Multiple conditions
    SELECT * FROM employees 
    WHERE department_id = 1 AND salary >= 90000 
    AND hire_date >= '2022-01-01';
    
    -- JOIN queries
    -- 5. Inner join with departments
    SELECT 
        e.first_name,
        e.last_name,
        e.salary,
        d.name AS department_name,
        d.location
    FROM employees e
    INNER JOIN departments d ON e.department_id = d.id;
    
    -- 6. Left join to include all departments
    SELECT 
        d.name AS department_name,
        COUNT(e.id) AS employee_count,
        AVG(e.salary) AS average_salary
    FROM departments d
    LEFT JOIN employees e ON d.id = e.department_id
    GROUP BY d.id, d.name;
    
    -- Aggregate functions
    -- 7. Count, AVG, SUM, MAX, MIN
    SELECT 
        COUNT(*) AS total_employees,
        AVG(salary) AS average_salary,
        MAX(salary) AS highest_salary,
        MIN(salary) AS lowest_salary,
        SUM(salary) AS total_payroll
    FROM employees
    WHERE is_active = TRUE;
    
    -- 8. Group by with HAVING
    SELECT 
        department_id,
        COUNT(*) AS employee_count,
        AVG(salary) AS avg_salary
    FROM employees
    GROUP BY department_id
    HAVING COUNT(*) > 1
    ORDER BY avg_salary DESC;
    
    -- Subqueries
    -- 9. Subquery in WHERE clause
    SELECT first_name, last_name, salary
    FROM employees
    WHERE salary > (
        SELECT AVG(salary) 
        FROM employees
    );
    
    -- 10. Subquery in FROM clause
    SELECT dept_name, avg_salary
    FROM (
        SELECT 
            d.name AS dept_name,
            AVG(e.salary) AS avg_salary,
            COUNT(e.id) AS emp_count
        FROM departments d
        LEFT JOIN employees e ON d.id = e.department_id
        GROUP BY d.id, d.name
    ) AS dept_stats
    WHERE emp_count > 0;
    
    -- Window functions
    -- 11. ROW_NUMBER, RANK, DENSE_RANK
    SELECT 
        first_name,
        last_name,
        salary,
        ROW_NUMBER() OVER (ORDER BY salary DESC) AS row_num,
        RANK() OVER (ORDER BY salary DESC) AS rank_num,
        DENSE_RANK() OVER (ORDER BY salary DESC) AS dense_rank
    FROM employees
    ORDER BY salary DESC;
    
    -- 12. LAG, LEAD functions
    SELECT 
        first_name,
        last_name,
        salary,
        LAG(salary, 1, 0) OVER (ORDER BY salary) AS prev_salary,
        LEAD(salary, 1, 0) OVER (ORDER BY salary) AS next_salary
    FROM employees
    ORDER BY salary;
    
    -- 13. Window functions with PARTITION BY
    SELECT 
        e.first_name,
        e.last_name,
        e.salary,
        d.name AS department,
        AVG(e.salary) OVER (PARTITION BY e.department_id) AS dept_avg_salary,
        e.salary - AVG(e.salary) OVER (PARTITION BY e.department_id) AS salary_diff_from_avg
    FROM employees e
    JOIN departments d ON e.department_id = d.id
    ORDER BY d.name, e.salary DESC;
    
    -- CTE (Common Table Expression)
    -- 14. Simple CTE
    WITH high_earners AS (
        SELECT first_name, last_name, salary, department_id
        FROM employees
        WHERE salary > 80000
    )
    SELECT 
        he.first_name,
        he.last_name,
        he.salary,
        d.name AS department
    FROM high_earners he
    JOIN departments d ON he.department_id = d.id;
    
    -- 15. Recursive CTE
    WITH RECURSIVE employee_hierarchy AS (
        SELECT id, first_name, last_name, department_id, 0 AS level
        FROM employees
        WHERE department_id = 1 AND salary = (
            SELECT MAX(salary) 
            FROM employees 
            WHERE department_id = 1
        )
        
        UNION ALL
        
        SELECT 
            e.id, 
            e.first_name, 
            e.last_name, 
            e.department_id, 
            eh.level + 1
        FROM employees e
        JOIN employee_hierarchy eh ON e.department_id = eh.department_id
        WHERE e.salary < eh.salary + 20000 AND eh.level < 3
    )
    SELECT * FROM employee_hierarchy;