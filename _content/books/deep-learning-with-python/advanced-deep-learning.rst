Advanced Deep Learning
======================

Advanced Architectures
----------------------

Residual Networks (ResNets)
~~~~~~~~~~~~~~~~~~~~~~~~~~

ResNets introduce skip connections to enable training of very deep networks:

.. code-block:: python

    from tensorflow.keras import layers, Model
    
    # Residual block
    def residual_block(x, filters, kernel_size=3):
        shortcut = x
        
        x = layers.Conv2D(filters, kernel_size, padding='same')(x)
        x = layers.BatchNormalization()(x)
        x = layers.ReLU()(x)
        x = layers.Conv2D(filters, kernel_size, padding='same')(x)
        x = layers.BatchNormalization()(x)
        
        # Add skip connection
        if shortcut.shape[-1] != filters:
            shortcut = layers.Conv2D(filters, 1, padding='same')(shortcut)
            shortcut = layers.BatchNormalization()(shortcut)
        
        x = layers.Add()([x, shortcut])
        x = layers.ReLU()(x)
        return x
    
    # Build ResNet
    inputs = layers.Input(shape=(224, 224, 3))
    x = layers.Conv2D(64, 7, strides=2, padding='same')(inputs)
    x = layers.BatchNormalization()(x)
    x = layers.ReLU()(x)
    x = layers.MaxPooling2D(3, strides=2, padding='same')(x)
    
    x = residual_block(x, 64)
    x = residual_block(x, 64)
    x = residual_block(x, 128)
    x = residual_block(x, 128)
    
    x = layers.GlobalAveragePooling2D()(x)
    outputs = layers.Dense(1000, activation='softmax')(x)
    
    model = Model(inputs, outputs)

Inception Networks
~~~~~~~~~~~~~~~~~~

Inception modules use multiple filter sizes in parallel:

.. code-block:: python

    def inception_module(x, filters):
        branch1x1 = layers.Conv2D(filters[0], 1, padding='same', activation='relu')(x)
        
        branch3x3 = layers.Conv2D(filters[1], 1, padding='same', activation='relu')(x)
        branch3x3 = layers.Conv2D(filters[2], 3, padding='same', activation='relu')(branch3x3)
        
        branch5x5 = layers.Conv2D(filters[3], 1, padding='same', activation='relu')(x)
        branch5x5 = layers.Conv2D(filters[4], 5, padding='same', activation='relu')(branch5x5)
        
        branch_pool = layers.MaxPooling2D(3, strides=1, padding='same')(x)
        branch_pool = layers.Conv2D(filters[5], 1, padding='same', activation='relu')(branch_pool)
        
        x = layers.Concatenate()([branch1x1, branch3x3, branch5x5, branch_pool])
        return x

Dense Networks
~~~~~~~~~~~~~~

DenseNet connects each layer to every other layer:

.. code-block:: python

    def dense_block(x, blocks, growth_rate):
        for i in range(blocks):
            x = conv_block(x, growth_rate)
        return x
    
    def conv_block(x, growth_rate):
        x = layers.BatchNormalization()(x)
        x = layers.ReLU()(x)
        x = layers.Conv2D(growth_rate, 3, padding='same')(x)
        return x

Attention Mechanisms
---------------------

Self-Attention
~~~~~~~~~~~~~~

.. code-block:: python

    class SelfAttention(layers.Layer):
        def __init__(self, units):
            super(SelfAttention, self).__init__()
            self.units = units
            
        def build(self, input_shape):
            self.query = layers.Dense(self.units)
            self.key = layers.Dense(self.units)
            self.value = layers.Dense(self.units)
            
        def call(self, inputs):
            q = self.query(inputs)
            k = self.key(inputs)
            v = self.value(inputs)
            
            # Scaled dot-product attention
            scores = tf.matmul(q, k, transpose_b=True)
            scores = scores / tf.math.sqrt(tf.cast(self.units, tf.float32))
            weights = tf.nn.softmax(scores)
            
            output = tf.matmul(weights, v)
            return output

Multi-Head Attention
~~~~~~~~~~~~~~~~~~

.. code-block:: python

    class MultiHeadAttention(layers.Layer):
        def __init__(self, d_model, num_heads):
            super(MultiHeadAttention, self).__init__()
            self.num_heads = num_heads
            self.d_model = d_model
            self.depth = d_model // num_heads
            
            self.wq = layers.Dense(d_model)
            self.wk = layers.Dense(d_model)
            self.wv = layers.Dense(d_model)
            self.wo = layers.Dense(d_model)
            
        def split_heads(self, x):
            x = tf.reshape(x, (-1, x.shape[1], self.num_heads, self.depth))
            return tf.transpose(x, perm=[0, 2, 1, 3])
            
        def call(self, q, k, v):
            q = self.split_heads(self.wq(q))
            k = self.split_heads(self.wk(k))
            v = self.split_heads(self.wv(v))
            
            # Scaled dot-product attention
            scores = tf.matmul(q, k, transpose_b=True)
            scores = scores / tf.math.sqrt(tf.cast(self.depth, tf.float32))
            weights = tf.nn.softmax(scores)
            
            attention = tf.matmul(weights, v)
            attention = tf.transpose(attention, perm=[0, 2, 1, 3])
            attention = tf.reshape(attention, (-1, attention.shape[1], self.d_model))
            
            return self.wo(attention)

Generative Models
-----------------

Variational Autoencoders (VAEs)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Encoder
    encoder_inputs = layers.Input(shape=(28, 28, 1))
    x = layers.Conv2D(32, 3, activation='relu', strides=2, padding='same')(encoder_inputs)
    x = layers.Conv2D(64, 3, activation='relu', strides=2, padding='same')(x)
    x = layers.Flatten()(x)
    
    # Latent space
    z_mean = layers.Dense(2)(x)
    z_log_var = layers.Dense(2)(x)
    
    # Sampling
    def sampling(args):
        z_mean, z_log_var = args
        epsilon = tf.keras.backend.random_normal(shape=tf.shape(z_mean))
        return z_mean + tf.exp(0.5 * z_log_var) * epsilon
    
    z = layers.Lambda(sampling)([z_mean, z_log_var])
    
    # Decoder
    decoder_inputs = layers.Input(shape=(2,))
    x = layers.Dense(7 * 7 * 64, activation='relu')(decoder_inputs)
    x = layers.Reshape((7, 7, 64))(x)
    x = layers.Conv2DTranspose(64, 3, activation='relu', strides=2, padding='same')(x)
    x = layers.Conv2DTranspose(32, 3, activation='relu', strides=2, padding='same')(x)
    decoder_outputs = layers.Conv2DTranspose(1, 3, activation='sigmoid', padding='same')(x)
    
    # Models
    encoder = Model(encoder_inputs, [z_mean, z_log_var])
    decoder = Model(decoder_inputs, decoder_outputs)
    
    # VAE model
    vae_outputs = decoder(encoder(encoder_inputs)[1])
    vae = Model(encoder_inputs, vae_outputs)

Generative Adversarial Networks (GANs)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Generator
    generator = models.Sequential([
        layers.Dense(7 * 7 * 256, use_bias=False, input_shape=(100,)),
        layers.BatchNormalization(),
        layers.ReLU(),
        layers.Reshape((7, 7, 256)),
        layers.Conv2DTranspose(128, 5, use_bias=False, strides=1, padding='same'),
        layers.BatchNormalization(),
        layers.ReLU(),
        layers.Conv2DTranspose(64, 5, strides=2, padding='same'),
        layers.BatchNormalization(),
        layers.ReLU(),
        layers.Conv2DTranspose(1, 5, strides=2, padding='same', activation='tanh')
    ])
    
    # Discriminator
    discriminator = models.Sequential([
        layers.Conv2D(64, 5, strides=2, padding='same', input_shape=[28, 28, 1]),
        layers.LeakyReLU(0.2),
        layers.Dropout(0.3),
        layers.Conv2D(128, 5, strides=2, padding='same'),
        layers.LeakyReLU(0.2),
        layers.Dropout(0.3),
        layers.Flatten(),
        layers.Dense(1, activation='sigmoid')
    ])
    
    # Combined model
    discriminator.compile(optimizer='adam', loss='binary_crossentropy')
    discriminator.trainable = False
    
    gan_input = layers.Input(shape=(100,))
    gan_output = discriminator(generator(gan_input))
    gan = Model(gan_input, gan_output)
    gan.compile(optimizer='adam', loss='binary_crossentropy')

Diffusion Models
----------------

.. code-block:: python

    class DiffusionModel(tf.keras.Model):
        def __init__(self, network):
            super().__init__()
            self.network = network
            self.num_timesteps = 1000
            
        def call(self, images):
            # Add noise
            noise = tf.random.normal(tf.shape(images))
            t = tf.random.uniform((tf.shape(images)[0],), maxval=self.num_timesteps)
            
            # Predict noise
            predicted_noise = self.network([images, t])
            return predicted_noise

Reinforcement Learning with Deep Networks
-----------------------------------------

Deep Q-Networks (DQN)
~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    class DQN(tf.keras.Model):
        def __init__(self, num_actions):
            super(DQN, self).__init__()
            self.dense1 = layers.Dense(128, activation='relu')
            self.dense2 = layers.Dense(128, activation='relu')
            self.values = layers.Dense(num_actions)
            
        def call(self, inputs):
            x = self.dense1(inputs)
            x = self.dense2(x)
            return self.values(x)

Policy Gradients
~~~~~~~~~~~~~~~

.. code-block:: python

    class PolicyNetwork(tf.keras.Model):
        def __init__(self, num_actions):
            super(PolicyNetwork, self).__init__()
            self.dense1 = layers.Dense(128, activation='relu')
            self.dense2 = layers.Dense(128, activation='relu')
            self.action_logits = layers.Dense(num_actions)
            
        def call(self, inputs):
            x = self.dense1(inputs)
            x = self.dense2(x)
            return self.action_logits(x)

Meta-Learning
-------------

Model-Agnostic Meta-Learning (MAML)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    class MAML(tf.keras.Model):
        def __init__(self, model):
            super(MAML, self).__init__()
            self.model = model
            
        def adapt(self, support_data, learning_rate=0.01):
            # Perform gradient steps on support data
            with tf.GradientTape() as tape:
                loss = self.compute_loss(support_data)
            gradients = tape.gradient(loss, self.model.trainable_variables)
            
            # Update weights
            adapted_weights = []
            for weight, grad in zip(self.model.trainable_variables, gradients):
                adapted_weights.append(weight - learning_rate * grad)
            
            return adapted_weights

Advanced Training Techniques
--------------------------

Learning Rate Scheduling
~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Cosine annealing
    lr_schedule = tf.keras.optimizers.schedules.CosineDecay(
        initial_learning_rate=0.001,
        decay_steps=1000,
        alpha=0.0
    )
    
    optimizer = tf.keras.optimizers.Adam(learning_rate=lr_schedule)

Mixed Precision Training
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras import mixed_precision
    
    policy = mixed_precision.Policy('mixed_float16')
    mixed_precision.set_global_policy(policy)

Distributed Training
~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    strategy = tf.distribute.MirroredStrategy()
    
    with strategy.scope():
        model = create_model()
        model.compile(optimizer='adam', loss='sparse_categorical_crossentropy')

Custom Layers and Losses
------------------------

Custom Layer Example
~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    class CustomLayer(layers.Layer):
        def __init__(self, units):
            super(CustomLayer, self).__init__()
            self.units = units
            
        def build(self, input_shape):
            self.w = self.add_weight(
                shape=(input_shape[-1], self.units),
                initializer='random_normal',
                trainable=True
            )
            
        def call(self, inputs):
            return tf.matmul(inputs, self.w)

Custom Loss Function
~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def custom_loss(y_true, y_pred):
        # Custom loss calculation
        error = y_true - y_pred
        squared_error = tf.square(error)
        return tf.reduce_mean(squared_error)

Best Practices
--------------

1. **Architecture Selection**: Choose appropriate architecture for the task
2. **Regularization**: Use dropout, batch norm, weight decay
3. **Optimization**: Use appropriate optimizer and learning rate
4. **Monitoring**: Track training and validation metrics
5. **Experimentation**: Try different hyperparameters

Current Research Directions
--------------------------

1. **Efficient Transformers**: Reducing computational cost
2. **Self-Supervised Learning**: Learning without labels
3. **Neural Architecture Search**: Automated architecture design
4. **Explainable AI**: Understanding model decisions
5. **Continual Learning**: Learning from new data without forgetting