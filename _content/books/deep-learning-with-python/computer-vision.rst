Computer Vision
===============

Introduction to Computer Vision
--------------------------------

Computer vision is a field of artificial intelligence that trains computers to interpret and understand the visual world. Using digital images from cameras and videos and deep learning models, machines can accurately identify and classify objects.

Convolutional Neural Networks (CNNs)
------------------------------------

CNNs are a class of deep neural networks most commonly applied to analyzing visual imagery. They are inspired by the visual cortex of animals.

Key Components of CNNs
~~~~~~~~~~~~~~~~~~~~~~~

1. **Convolutional Layers**
   - Apply filters to input images
   - Detect features like edges, textures, shapes
   - Learn hierarchical representations

2. **Pooling Layers**
   - Reduce spatial dimensions
   - Provide translation invariance
   - Reduce computational complexity

3. **Fully Connected Layers**
   - Perform classification based on extracted features
   - Combine high-level features for decision making

Convolution Operations
-----------------------

Mathematical formulation:

$$ (I * K)(i,j) = \sum_{m}\sum_{n} I(i+m, j+n) \cdot K(m,n) $$

Where:
- $I$ is the input image
- $K$ is the kernel/filter
- $*$ denotes convolution

Common CNN Architectures
------------------------

LeNet-5 (1998)
~~~~~~~~~~~~~~

- One of the first successful CNNs
- 7 layers (conv + pool + conv + pool + fc)
- Used for handwritten digit recognition

AlexNet (2012)
~~~~~~~~~~~~~~

- Won ImageNet competition 2012
- 8 layers (5 conv + 3 fc)
- Used ReLU activation and dropout
- Sparked deep learning revolution

VGGNet (2014)
~~~~~~~~~~~~

- Simpler architecture using only 3×3 filters
- 16-19 layers
- Demonstrated network depth importance

GoogLeNet (2014)
~~~~~~~~~~~~~~~

- Introduced inception modules
- Multi-scale processing
- Efficient use of parameters

ResNet (2015)
~~~~~~~~~~~~

- Introduced residual connections
- Enabled training of very deep networks (152+ layers)
- Solved vanishing gradient problem

Building a CNN for Image Classification
---------------------------------------

Data Preparation
~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras.preprocessing.image import ImageDataGenerator
    
    # Data augmentation for training
    train_datagen = ImageDataGenerator(
        rescale=1./255,
        rotation_range=20,
        width_shift_range=0.2,
        height_shift_range=0.2,
        horizontal_flip=True,
        fill_mode='nearest'
    )
    
    # Only rescale for validation
    val_datagen = ImageDataGenerator(rescale=1./255)
    
    # Load data from directories
    train_generator = train_datagen.flow_from_directory(
        'data/train',
        target_size=(224, 224),
        batch_size=32,
        class_mode='categorical'
    )
    
    validation_generator = val_datagen.flow_from_directory(
        'data/validation',
        target_size=(224, 224),
        batch_size=32,
        class_mode='categorical'
    )

Model Architecture
~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras import layers, models
    
    model = models.Sequential([
        # First convolutional block
        layers.Conv2D(32, (3, 3), activation='relu', input_shape=(224, 224, 3)),
        layers.MaxPooling2D((2, 2)),
        
        # Second convolutional block
        layers.Conv2D(64, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),
        
        # Third convolutional block
        layers.Conv2D(128, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),
        
        # Fourth convolutional block
        layers.Conv2D(128, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),
        
        # Flatten and classify
        layers.Flatten(),
        layers.Dense(512, activation='relu'),
        layers.Dropout(0.5),
        layers.Dense(10, activation='softmax')
    ])
    
    model.compile(
        optimizer='adam',
        loss='categorical_crossentropy',
        metrics=['accuracy']
    )

Training the Model
~~~~~~~~~~~~~~~~~~

.. code-block:: python

    history = model.fit(
        train_generator,
        steps_per_epoch=len(train_generator),
        epochs=50,
        validation_data=validation_generator,
        validation_steps=len(validation_generator)
    )

Transfer Learning
----------------

Transfer learning leverages pre-trained models for new tasks:

Benefits:
- Reduced training time
- Better performance with less data
- Access to state-of-the-art architectures

Popular Pre-trained Models
~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **VGG16/19**
   - Simple architecture
   - Good feature extractor

2. **ResNet50/101/152**
   - Deep residual networks
   - Excellent performance

3. **InceptionV3**
   - Efficient architecture
   - Good for mobile deployment

4. **EfficientNet**
   - State-of-the-art efficiency
   - Best accuracy/parameter ratio

Example with Transfer Learning
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras.applications import VGG16
    from tensorflow.keras import Model
    
    # Load pre-trained model
    base_model = VGG16(weights='imagenet', include_top=False, input_shape=(224, 224, 3))
    
    # Freeze convolutional layers
    base_model.trainable = False
    
    # Add custom classifier
    x = base_model.output
    x = layers.GlobalAveragePooling2D()(x)
    x = layers.Dense(256, activation='relu')(x)
    predictions = layers.Dense(10, activation='softmax')(x)
    
    model = Model(inputs=base_model.input, outputs=predictions)
    
    # Train only the classifier
    model.compile(optimizer='adam', loss='categorical_crossentropy')
    model.fit(train_generator, epochs=10)

Object Detection
---------------

Beyond classification, CNNs can detect and localize objects:

Popular Approaches:
- R-CNN family
- YOLO (You Only Look Once)
- SSD (Single Shot Detector)
- RetinaNet

Semantic Segmentation
---------------------

Pixel-level classification:
- FCN (Fully Convolutional Networks)
- U-Net
- DeepLab
- Mask R-CNN

Practical Tips
--------------

1. **Data Augmentation**: Essential for preventing overfitting
2. **Batch Normalization**: Stabilizes training
3. **Learning Rate Scheduling**: Improves convergence
4. **Early Stopping**: Prevents overfitting
5. **Model Checkpoints**: Save best models

Common Challenges
-----------------

1. **Small Dataset**: Use transfer learning or data augmentation
2. **Class Imbalance**: Use class weights or focal loss
3. **Overfitting**: Add regularization, get more data
4. **Computational Cost**: Use smaller models or transfer learning

Applications
-----------

- Autonomous vehicles
- Medical imaging
- Face recognition
- Agricultural monitoring
- Quality control in manufacturing
- Augmented reality