---
title: "Supervised Learning"
---

Supervised Learning
===================

Introduction
------------

Supervised learning is the most common type of machine learning, where the algorithm learns from labeled training data. The goal is to learn a mapping function that can predict the output for new, unseen data.

Types of Supervised Learning
----------------------------

1. **Classification**: Predicting a discrete class label
2. **Regression**: Predicting a continuous value

Classification
--------------

Classification algorithms are used when the output variable is a category, such as "spam" or "not spam", "disease" or "no disease".

Common Classification Algorithms:

- Logistic Regression
- Decision Trees
- Random Forest
- Support Vector Machines
- Neural Networks
- k-Nearest Neighbors (k-NN)

Example: Email Classification
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Consider an email classification system that categorizes emails as "spam" or "not spam":

Features might include:
- Email length
- Number of capital letters
- Presence of certain keywords
- Sender's domain

.. snippet-card:: python-data-processing

Data preprocessing is crucial for email classification. We need to:
- Convert text to numerical features
- Handle missing values
- Normalize features
- Split data into training and testing sets

Here's how we might preprocess email data:

Text preprocessing involves:
- Tokenization (splitting text into words)
- Removing stop words
- Stemming or lemmatization
- Converting to numerical representations (TF-IDF, word embeddings)

Regression
---------

Regression algorithms are used when the output variable is a real or continuous value, such as predicting house prices, stock prices, or temperature.

Common Regression Algorithms:

- Linear Regression
- Polynomial Regression
- Ridge Regression
- Lasso Regression
- Support Vector Regression (SVR)
- Decision Tree Regression
- Random Forest Regression

Example: House Price Prediction
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

For predicting house prices, features might include:
- Square footage
- Number of bedrooms
- Location (latitude, longitude)
- Age of the house
- Proximity to amenities

.. snippet-card:: derivatives

Understanding derivatives is crucial for optimizing regression models.

When using gradient descent to minimize the mean squared error in linear regression, we need to compute derivatives of the loss function with respect to the model parameters.

The derivative tells us the direction of steepest ascent, so we move in the opposite direction to minimize the error.

Model Evaluation Metrics
------------------------

Classification Metrics:
- Accuracy
- Precision
- Recall
- F1 Score
- ROC-AUC
- Confusion Matrix

Regression Metrics:
- Mean Absolute Error (MAE)
- Mean Squared Error (MSE)
- Root Mean Squared Error (RMSE)
- R-squared (R²)

Overfitting and Underfitting
--------------------------

**Overfitting**: Model performs well on training data but poorly on test data
**Underfitting**: Model is too simple and captures neither training nor test data patterns

Techniques to prevent overfitting:
- Cross-validation
- Regularization (L1, L2)
- Dropout (for neural networks)
- Early stopping
- Data augmentation

.. snippet-card:: sql-queries

When working with large datasets, SQL queries are essential for data extraction and preprocessing.

For example, to extract training data from a database:

.. code-block:: sql

    SELECT 
        house_id,
        square_feet,
        bedrooms,
        bathrooms,
        price,
        year_built
    FROM 
        houses
    WHERE 
        price IS NOT NULL
        AND square_feet > 0
    ORDER BY 
        price DESC
    LIMIT 10000;

This query extracts relevant features for house price prediction, ensuring data quality by filtering out invalid entries.

Practical Considerations
------------------------

When building supervised learning models:

1. **Feature Selection**: Choose relevant features
2. **Data Quality**: Ensure clean, consistent data
3. **Model Selection**: Choose appropriate algorithm
4. **Hyperparameter Tuning**: Optimize model parameters
5. **Cross-validation**: Validate model performance
6. **Interpretability**: Understand model decisions

Conclusion
----------

Supervised learning forms the foundation of many real-world ML applications. By understanding both classification and regression, along with proper evaluation techniques, we can build effective predictive models.

In the next chapter, we'll explore unsupervised learning, where we work with unlabeled data to discover hidden patterns and structures.