---
title: "Feature Engineering"
---

Feature Engineering
==================

Introduction
------------

Feature engineering is the process of selecting, creating, and transforming variables (features) to improve machine learning model performance. It's often said that "garbage in, garbage out" - the quality of your features largely determines the quality of your model.

Why Feature Engineering Matters
--------------------------------

1. **Better Model Performance**: Good features lead to better predictions
2. **Improved Interpretability**: Meaningful features make models easier to understand
3. **Reduced Complexity**: Proper features can simplify the model
4. **Faster Training**: Well-engineered features can reduce training time

Types of Feature Engineering
----------------------------

1. **Feature Creation**: Creating new features from existing ones
2. **Feature Selection**: Choosing the most relevant features
3. **Feature Transformation**: Modifying features to improve their properties
4. **Feature Scaling**: Normalizing feature ranges

Feature Creation
----------------

Polynomial Features
~~~~~~~~~~~~~~~~~~~

Creating polynomial features can capture non-linear relationships:

.. code-block:: python

    from sklearn.preprocessing import PolynomialFeatures
    import numpy as np
    
    X = np.array([[1, 2], [3, 4], [5, 6]])
    poly = PolynomialFeatures(degree=2, include_bias=False)
    X_poly = poly.fit_transform(X)
    
    # Original: [x1, x2]
    # Transformed: [x1, x2, x1², x1×x2, x2²]

Interaction Features
~~~~~~~~~~~~~~~~~~~

Interaction terms capture relationships between variables:

.. snippet-card:: cpp-algorithms

C++ allows for efficient computation of interaction features in high-performance scenarios:

.. code-block:: cpp

    #include <vector>
    #include <algorithm>
    
    std::vector<std::vector<double>> create_interaction_features(
        const std::vector<std::vector<double>>& X) {
        
        size_t n_samples = X.size();
        size_t n_features = X[0].size();
        std::vector<std::vector<double>> X_interaction(n_samples);
        
        for (size_t i = 0; i < n_samples; ++i) {
            // Original features
            X_interaction[i] = X[i];
            
            // Interaction terms
            for (size_t j = 0; j < n_features; ++j) {
                for (size_t k = j + 1; k < n_features; ++k) {
                    X_interaction[i].push_back(X[i][j] * X[i][k]);
                }
            }
        }
        
        return X_interaction;
    }

Domain-Specific Features
~~~~~~~~~~~~~~~~~~~~~~~

Creating features based on domain knowledge often provides the most value:

Example: Time Series Features
- Day of week
- Month of year
- Holiday indicator
- Seasonal indicators
- Lag features
- Rolling averages

.. snippet-card:: wave-function

In quantum mechanics, wave functions can inspire feature engineering for time series data.

Just as wave functions capture both position and momentum uncertainty, we can create features that capture both local and global patterns in time series:

- Local: Recent changes, short-term trends
- Global: Long-term cycles, seasonal patterns
- Uncertainty: Volatility measures, confidence intervals

Feature Selection
----------------

Filter Methods
~~~~~~~~~~~~~~

Filter methods select features based on their statistical properties:

.. code-block:: python

    from sklearn.feature_selection import SelectKBest, f_classif
    from sklearn.datasets import load_iris
    
    X, y = load_iris(return_X_y=True)
    
    # Select top 2 features based on ANOVA F-value
    selector = SelectKBest(f_classif, k=2)
    X_selected = selector.fit_transform(X, y)
    
    print(f"Original features: {X.shape[1]}")
    print(f"Selected features: {X_selected.shape[1]}")

Wrapper Methods
~~~~~~~~~~~~~~~

Wrapper methods use the model to evaluate feature subsets:

.. snippet-card:: integrals

Numerical integration techniques can be applied to feature selection by treating the feature space as a continuous domain.

Just as we calculate the area under a curve to understand its total value, we can evaluate the "area under the performance curve" across different feature combinations.

Embedded Methods
~~~~~~~~~~~~~~~

Embedded methods perform feature selection during model training:

- Lasso (L1) regularization
- Ridge (L2) regularization
- Decision tree feature importance
- Random forest feature importance

Feature Transformation
---------------------

Normalization
~~~~~~~~~~~~

Min-Max Scaling:
.. code-block:: python

    from sklearn.preprocessing import MinMaxScaler
    
    scaler = MinMaxScaler()
    X_scaled = scaler.fit_transform(X)

Standardization (Z-score):
.. code-block:: python

    from sklearn.preprocessing import StandardScaler
    
    scaler = StandardScaler()
    X_scaled = scaler.fit_transform(X)

Log Transformation
~~~~~~~~~~~~~~~~~

Log transformation helps handle skewed data:

.. code-block:: python

    import numpy as np
    
    # Apply log transformation
    X_log = np.log1p(X)  # log1p handles zero values

Box-Cox Transformation
~~~~~~~~~~~~~~~~~~~~

Box-Cox is a family of power transformations:

.. code-block:: python

    from sklearn.preprocessing import PowerTransformer
    
    pt = PowerTransformer(method='box-cox')
    X_transformed = pt.fit_transform(X)

.. article-card:: quantum-computing

Quantum computing principles can inspire advanced feature transformation techniques.

Just as quantum bits can exist in superposition, features can exist in transformed spaces that reveal hidden patterns:

- Quantum-inspired feature mapping
- Entanglement-inspired feature correlation
- Superposition-inspired feature combination

Handling Categorical Features
----------------------------

One-Hot Encoding
~~~~~~~~~~~~~~~~

.. code-block:: python

    from sklearn.preprocessing import OneHotEncoder
    
    encoder = OneHotEncoder(sparse=False)
    X_encoded = encoder.fit_transform(X_categorical)

Label Encoding
~~~~~~~~~~~~~~

.. code-block:: python

    from sklearn.preprocessing import LabelEncoder
    
    encoder = LabelEncoder()
    y_encoded = encoder.fit_transform(y)

Target Encoding
~~~~~~~~~~~~~~

Target encoding uses the target variable to encode categorical features:

.. code-block:: python

    import pandas as pd
    
    def target_encoding(df, categorical_col, target_col):
        # Calculate mean target for each category
        encoding_map = df.groupby(categorical_col)[target_col].mean()
        
        # Apply encoding
        df[f'{categorical_col}_encoded'] = df[categorical_col].map(encoding_map)
        
        return df

Handling Missing Values
----------------------

Deletion Strategy
~~~~~~~~~~~~~~~~~

Remove rows or columns with too many missing values:

.. code-block:: python

    # Remove rows with >50% missing values
    threshold = len(df.columns) * 0.5
    df.dropna(thresh=threshold, inplace=True)
    
    # Remove columns with >50% missing values
    df.dropna(axis=1, thresh=len(df) * 0.5, inplace=True)

Imputation Strategies
~~~~~~~~~~~~~~~~~~~~

Mean/Median/Mode Imputation:
.. code-block:: python

    from sklearn.impute import SimpleImputer
    
    # Mean imputation for numerical features
    mean_imputer = SimpleImputer(strategy='mean')
    X_imputed = mean_imputer.fit_transform(X_numerical)
    
    # Mode imputation for categorical features
    mode_imputer = SimpleImputer(strategy='most_frequent')
    X_imputed = mode_imputer.fit_transform(X_categorical)

Advanced Imputation:
- KNN imputation
- Regression imputation
- Matrix completion
- Deep learning-based imputation

Feature Engineering for Different Data Types
-----------------------------------------

Text Features
~~~~~~~~~~~~~~

- Bag of Words
- TF-IDF
- Word embeddings (Word2Vec, GloVe)
- Contextual embeddings (BERT, GPT)

Image Features
~~~~~~~~~~~~~~

- Raw pixels
- Histogram of Oriented Gradients (HOG)
- Convolutional Neural Network features
- Pre-trained model embeddings

Time Series Features
~~~~~~~~~~~~~~~~~~

- Lag features
- Rolling statistics
- Seasonal decomposition
- Fourier transforms

Best Practices
--------------

1. **Understand Your Data**: Explore data distributions and relationships
2. **Start Simple**: Begin with basic features before complex ones
3. **Validate Properly**: Use appropriate validation techniques
4. **Document Everything**: Keep track of feature engineering steps
5. **Iterate**: Continuously refine and improve features

Common Pitfalls
---------------

1. **Data Leakage**: Using future information in feature creation
2. **Overfitting**: Creating too many features
3. **Curse of Dimensionality**: Too many features relative to samples
4. **Ignoring Feature Importance**: Not understanding which features matter

Tools and Libraries
------------------

Python:
- scikit-learn
- pandas
- numpy
- featuretools
- tsfresh (for time series)

R:
- caret
- recipes
- tidyverse

Conclusion
----------

Feature engineering is both an art and a science. While there are systematic approaches, creativity and domain knowledge often lead to the best features. Remember that good features can make even simple models perform well, while poor features can make even complex models fail.

In the next chapter, we'll explore model evaluation, where we learn how to properly assess and compare the performance of our machine learning models.