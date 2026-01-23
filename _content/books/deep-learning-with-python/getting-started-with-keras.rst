Getting Started with Keras
===========================

Introduction to Keras
--------------------

Keras is a high-level neural network API written in Python. It provides a clean, intuitive interface for building and training deep learning models. Keras runs on top of TensorFlow, Theano, or CNTK, but TensorFlow is the most commonly used backend.

Why Keras?
~~~~~~~~~~

- **User-friendly**: Simple and consistent API
- **Modular**: Easy to build complex models
- **Extensible**: Easy to write custom layers
- **Fast**: Optimized for both CPU and GPU

Installation
------------

Install Keras and TensorFlow:

.. code-block:: bash

    pip install tensorflow

    # or with conda
    conda install tensorflow

Verify installation:

.. code-block:: python

    import tensorflow as tf
    from tensorflow import keras
    print(f"TensorFlow version: {tf.__version__}")
    print(f"Keras version: {keras.__version__}")

Building Your First Model
-------------------------

The Sequential API
~~~~~~~~~~~~~~~~~~

The simplest way to build models in Keras:

.. code-block:: python

    from tensorflow import keras
    from tensorflow.keras import layers
    
    # Create a sequential model
    model = keras.Sequential([
        layers.Dense(128, activation='relu', input_shape=(784,)),
        layers.Dense(64, activation='relu'),
        layers.Dense(10, activation='softmax')
    ])
    
    # Display model architecture
    model.summary()

The Functional API
~~~~~~~~~~~~~~~~~~

For more complex architectures:

.. code-block:: python

    inputs = keras.Input(shape=(784,))
    x = layers.Dense(128, activation='relu')(inputs)
    x = layers.Dense(64, activation='relu')(x)
    outputs = layers.Dense(10, activation='softmax')(x)
    
    model = keras.Model(inputs=inputs, outputs=outputs)
    model.summary()

Compiling the Model
-------------------

Configure the learning process:

.. code-block:: python

    model.compile(
        optimizer='adam',
        loss='sparse_categorical_crossentropy',
        metrics=['accuracy']
    )

Common optimizers:
- SGD
- RMSprop
- Adam
- Adagrad

Training the Model
------------------

Fit the model to training data:

.. code-block:: python

    history = model.fit(
        x_train, y_train,
        batch_size=32,
        epochs=10,
        validation_data=(x_val, y_val)
    )

Evaluating the Model
--------------------

Assess model performance:

.. code-block:: python

    # Evaluate on test data
    test_loss, test_acc = model.evaluate(x_test, y_test)
    print(f"Test accuracy: {test_acc:.4f}")
    
    # Make predictions
    predictions = model.predict(x_test)
    predicted_classes = predictions.argmax(axis=1)

Working with Data
-----------------

Data Preprocessing
~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras.preprocessing import image
    
    # Load and preprocess images
    train_datagen = image.ImageDataGenerator(
        rescale=1./255,
        rotation_range=20,
        width_shift_range=0.2,
        height_shift_range=0.2,
        horizontal_flip=True
    )
    
    train_generator = train_datagen.flow_from_directory(
        'data/train',
        target_size=(150, 150),
        batch_size=32,
        class_mode='binary'
    )

Text Data Processing
~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras.preprocessing.text import Tokenizer
    from tensorflow.keras.preprocessing.sequence import pad_sequences
    
    # Tokenize text
    tokenizer = Tokenizer(num_words=10000)
    tokenizer.fit_on_texts(texts)
    sequences = tokenizer.texts_to_sequences(texts)
    
    # Pad sequences
    padded_sequences = pad_sequences(sequences, maxlen=100)

Common Layer Types
------------------

Dense Layers
~~~~~~~~~~~~

Fully connected layers:

.. code-block:: python

    layers.Dense(128, activation='relu')

Convolutional Layers
~~~~~~~~~~~~~~~~~~~~

For image processing:

.. code-block:: python

    layers.Conv2D(32, (3, 3), activation='relu', padding='same')
    layers.MaxPooling2D((2, 2))

Recurrent Layers
~~~~~~~~~~~~~~~~

For sequence data:

.. code-block:: python

    layers.LSTM(64, return_sequences=True)
    layers.GRU(32)

Dropout Layers
~~~~~~~~~~~~~~

For regularization:

.. code-block:: python

    layers.Dropout(0.5)

Batch Normalization
~~~~~~~~~~~~~~~~~~~

For stable training:

.. code-block:: python

    layers.BatchNormalization()

Saving and Loading Models
------------------------

Save the entire model:

.. code-block:: python

    model.save('my_model.h5')

Load the model:

.. code-block:: python

    loaded_model = keras.models.load_model('my_model.h5')

Save only weights:

.. code-block:: python

    model.save_weights('my_weights.h5')
    model.load_weights('my_weights.h5')

Callbacks
---------

Monitor and control training:

.. code-block:: python

    from tensorflow.keras.callbacks import EarlyStopping, ModelCheckpoint
    
    callbacks = [
        EarlyStopping(patience=3, restore_best_weights=True),
        ModelCheckpoint('best_model.h5', save_best_only=True)
    ]
    
    model.fit(
        x_train, y_train,
        epochs=100,
        callbacks=callbacks
    )

Best Practices
--------------

1. **Start Simple**: Begin with simple models
2. **Monitor Metrics**: Track loss and accuracy
3. **Use Validation**: Always have a validation set
4. **Regularize**: Prevent overfitting early
5. **Experiment**: Try different architectures
6. **Document**: Keep track of experiments

Troubleshooting
---------------

Common issues and solutions:

1. **Model not learning**
   - Check learning rate
   - Verify data preprocessing
   - Ensure correct loss function

2. **Overfitting**
   - Add dropout
   - Use regularization
   - Get more data

3. **Slow training**
   - Use GPU
   - Reduce batch size
   - Optimize data pipeline

4. **Memory issues**
   - Reduce batch size
   - Use data generators
   - Implement gradient checkpointing