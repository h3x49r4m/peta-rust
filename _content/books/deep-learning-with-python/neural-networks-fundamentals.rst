Neural Networks Fundamentals
===========================

The Building Blocks of Neural Networks
-------------------------------------

Neural networks are composed of interconnected nodes or neurons that process and transmit information. Understanding these fundamental components is essential for mastering deep learning.

Neurons: The Basic Unit
----------------------

A neuron receives inputs, processes them, and produces an output:

$$y = f(\sum_{i=1}^{n} w_i x_i + b)$$

Where:
- $x_i$ are the inputs
- $w_i$ are the weights
- $b$ is the bias
- $f$ is the activation function
- $y$ is the output

Activation Functions
--------------------

Activation functions introduce non-linearity into neural networks, enabling them to learn complex patterns.

Common Activation Functions:

1. **Sigmoid**: $\sigma(x) = \frac{1}{1 + e^{-x}}$
   - Range: (0, 1)
   - Used in: Output layer for binary classification

2. **Tanh**: $\tanh(x) = \frac{e^x - e^{-x}}{e^x + e^{-x}}$
   - Range: (-1, 1)
   - Used in: Hidden layers (historically)

3. **ReLU**: $f(x) = \max(0, x)$
   - Range: [0, ∞)
   - Used in: Most hidden layers (modern default)

4. **Softmax**: $\sigma(z)_i = \frac{e^{z_i}}{\sum_{j=1}^{K} e^{z_j}}$
   - Range: (0, 1), sums to 1
   - Used in: Output layer for multi-class classification

Network Architectures
---------------------

Feedforward Networks
~~~~~~~~~~~~~~~~~~~
The simplest type of neural network where information flows in one direction:
- Input layer
- Hidden layers
- Output layer

Convolutional Neural Networks (CNNs)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Specialized for processing grid-like data (e.g., images):
- Convolutional layers
- Pooling layers
- Fully connected layers

Recurrent Neural Networks (RNNs)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Designed for sequential data:
- Memory of past inputs
- Time series analysis
- Natural language processing

Training Neural Networks
------------------------

The training process involves adjusting weights to minimize a loss function:

1. **Forward Propagation**: Compute predictions
2. **Loss Calculation**: Measure prediction error
3. **Backpropagation**: Compute gradients
4. **Weight Update**: Adjust weights using gradients

Loss Functions
--------------

Common loss functions for different tasks:

Regression:
- Mean Squared Error: $MSE = \frac{1}{n}\sum_{i=1}^{n}(y_i - \hat{y}_i)^2$

Binary Classification:
- Binary Cross-Entropy: $BCE = -\frac{1}{n}\sum_{i=1}^{n}[y_i\log(\hat{y}_i) + (1-y_i)\log(1-\hat{y}_i)]$

Multi-class Classification:
- Categorical Cross-Entropy: $CCE = -\frac{1}{n}\sum_{i=1}^{n}\sum_{c=1}^{C}y_{ic}\log(\hat{y}_{ic})$

Optimization Algorithms
-----------------------

Gradient Descent Variants:

1. **Batch Gradient Descent**
   - Uses entire dataset for each update
   - Stable but slow

2. **Stochastic Gradient Descent (SGD)**
   - Uses one sample per update
   - Fast but noisy

3. **Mini-batch Gradient Descent**
   - Uses small batches per update
   - Balance between speed and stability

4. **Adam Optimizer**
   - Adaptive learning rates
   - Most popular choice

Regularization Techniques
--------------------------

Preventing overfitting:

1. **L1/L2 Regularization**
   - Add penalty to loss function
   - L1: $L_1 = \lambda\sum|w_i|$
   - L2: $L_2 = \lambda\sum w_i^2$

2. **Dropout**
   - Randomly disable neurons during training
   - Prevents co-adaptation

3. **Early Stopping**
   - Monitor validation loss
   - Stop when validation error increases

4. **Data Augmentation**
   - Create variations of training data
   - Increases effective dataset size

Common Pitfalls
---------------

1. **Vanishing Gradients**
   - Gradients become very small in deep networks
   - Solution: ReLU activation, proper initialization

2. **Exploding Gradients**
   - Gradients become very large
   - Solution: Gradient clipping

3. **Overfitting**
   - Model performs well on training data but poorly on test data
   - Solution: Regularization, more data, simpler model

4. **Underfitting**
   - Model is too simple to capture patterns
   - Solution: More complex model, longer training