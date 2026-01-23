---
title: Python Data Processing
date: 2026-01-19
tags: [python, data-science, pandas]
---

Python Data Processing Example
=============================

This snippet demonstrates data processing using pandas and numpy.

.. code-block:: python

    import pandas as pd
    import numpy as np
    from sklearn.preprocessing import StandardScaler
    
    # Create sample data
    data = {
        'name': ['Alice', 'Bob', 'Charlie', 'Diana', 'Eve'],
        'age': [25, 30, 35, 28, 32],
        'salary': [50000, 60000, 70000, 55000, 65000],
        'department': ['IT', 'HR', 'Finance', 'IT', 'Marketing']
    }
    
    # Create DataFrame
    df = pd.DataFrame(data)
    print("Original DataFrame:")
    print(df)
    
    # Data preprocessing
    # 1. Handle missing values
    df.fillna({'salary': df['salary'].mean()}, inplace=True)
    
    # 2. Standardize numerical columns
    scaler = StandardScaler()
    numerical_cols = ['age', 'salary']
    df[numerical_cols] = scaler.fit_transform(df[numerical_cols])
    
    # 3. One-hot encode categorical columns
    df_encoded = pd.get_dummies(df, columns=['department'])
    
    print("\nProcessed DataFrame:")
    print(df_encoded)
    
    # 4. Group by department and calculate mean salary
    dept_salary = df.groupby('department')['salary'].mean()
    print("\nAverage salary by department:")
    print(dept_salary)