Text and Sequences
==================

Working with Text Data
---------------------

Text data presents unique challenges for machine learning:
- Variable length sequences
- Discrete tokens (words/characters)
- Semantic meaning and context
- Temporal dependencies

Text Preprocessing
-------------------

Tokenization
~~~~~~~~~~~~

Breaking text into smaller units:

.. code-block:: python

    from tensorflow.keras.preprocessing.text import Tokenizer
    
    # Word-level tokenization
    tokenizer = Tokenizer(num_words=10000)
    tokenizer.fit_on_texts(texts)
    sequences = tokenizer.texts_to_sequences(texts)
    
    # Character-level tokenization
    char_tokenizer = Tokenizer(char_level=True)
    char_tokenizer.fit_on_texts(texts)
    char_sequences = char_tokenizer.texts_to_sequences(texts)

Padding Sequences
~~~~~~~~~~~~~~~~

Making all sequences the same length:

.. code-block:: python

    from tensorflow.keras.preprocessing.sequence import pad_sequences
    
    # Pad sequences to maximum length
    padded_sequences = pad_sequences(sequences, maxlen=100)
    
    # Pad with post-padding
    padded_sequences = pad_sequences(sequences, maxlen=100, padding='post')
    
    # Truncate sequences
    padded_sequences = pad_sequences(sequences, maxlen=100, truncating='post')

Word Embeddings
----------------

Word embeddings are dense vector representations of words:

Benefits:
- Capture semantic relationships
- Reduce dimensionality
- Enable better generalization

Types of Embeddings
~~~~~~~~~~~~~~~~~~

1. **Learned Embeddings**: Trained with the model
2. **Pre-trained Embeddings**: GloVe, Word2Vec, fastText
3. **Contextual Embeddings**: BERT, GPT

Implementing Embeddings in Keras
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras import layers
    
    # Embedding layer
    embedding_layer = layers.Embedding(
        input_dim=vocab_size,      # Size of vocabulary
        output_dim=embedding_dim,  # Dimension of embedding
        input_length=max_length     # Length of input sequences
    )
    
    # In a model
    model = keras.Sequential([
        embedding_layer,
        layers.Flatten(),
        layers.Dense(1, activation='sigmoid')
    ])

Using Pre-trained Embeddings
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    import numpy as np
    
    # Load GloVe embeddings
    embeddings_index = {}
    with open('glove.6B.100d.txt', encoding='utf8') as f:
        for line in f:
            values = line.split()
            word = values[0]
            coefs = np.asarray(values[1:], dtype='float32')
            embeddings_index[word] = coefs
    
    # Create embedding matrix
    embedding_matrix = np.zeros((vocab_size, embedding_dim))
    for word, i in tokenizer.word_index.items():
        if i < vocab_size:
            embedding_vector = embeddings_index.get(word)
            if embedding_vector is not None:
                embedding_matrix[i] = embedding_vector
    
    # Use in embedding layer
    embedding_layer = layers.Embedding(
        vocab_size,
        embedding_dim,
        weights=[embedding_matrix],
        input_length=max_length,
        trainable=False  # Freeze embeddings
    )

Recurrent Neural Networks (RNNs)
--------------------------------

RNNs are designed to process sequential data by maintaining an internal state or memory.

Simple RNN
~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras.layers import SimpleRNN
    
    model = keras.Sequential([
        layers.Embedding(vocab_size, 128),
        layers.SimpleRNN(64),
        layers.Dense(1, activation='sigmoid')
    ])

Long Short-Term Memory (LSTM)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

LSTMs address the vanishing gradient problem in RNNs:

.. code-block:: python

    from tensorflow.keras.layers import LSTM
    
    model = keras.Sequential([
        layers.Embedding(vocab_size, 128),
        layers.LSTM(64),
        layers.Dense(1, activation='sigmoid')
    ])
    
    # Stacked LSTM
    model = keras.Sequential([
        layers.Embedding(vocab_size, 128),
        layers.LSTM(64, return_sequences=True),
        layers.LSTM(32),
        layers.Dense(1, activation='sigmoid')
    ])

Gated Recurrent Unit (GRU)
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Simpler variant of LSTM with comparable performance:

.. code-block:: python

    from tensorflow.keras.layers import GRU
    
    model = keras.Sequential([
        layers.Embedding(vocab_size, 128),
        layers.GRU(64),
        layers.Dense(1, activation='sigmoid')
    ])

Bidirectional RNNs
~~~~~~~~~~~~~~~~~

Process sequences in both directions:

.. code-block:: python

    from tensorflow.keras.layers import Bidirectional, LSTM
    
    model = keras.Sequential([
        layers.Embedding(vocab_size, 128),
        Bidirectional(LSTM(64)),
        layers.Dense(1, activation='sigmoid')
    ])

Attention Mechanisms
---------------------

Attention allows models to focus on relevant parts of the input:

.. code-block:: python

    from tensorflow.keras.layers import Attention
    
    # Query, Value, Key attention
    query = layers.Dense(64)(inputs)
    value = layers.Dense(64)(inputs)
    key = layers.Dense(64)(inputs)
    
    attention_output = Attention()([query, value, key])

Transformers
------------

Transformer architecture revolutionized NLP:

Key Components:
- Multi-head self-attention
- Positional encoding
- Feed-forward networks
- Layer normalization

Using Pre-trained Transformers
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from transformers import TFAutoModel, TFAutoTokenizer
    
    # Load pre-trained model
    model_name = 'bert-base-uncased'
    tokenizer = TFAutoTokenizer.from_pretrained(model_name)
    model = TFAutoModel.from_pretrained(model_name)
    
    # Tokenize text
    inputs = tokenizer(texts, padding=True, truncation=True, return_tensors='tf')
    
    # Get embeddings
    outputs = model(inputs)

Text Classification
-----------------

Sentiment Analysis Example
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from tensorflow.keras import layers, models
    
    model = models.Sequential([
        layers.Embedding(vocab_size, 128),
        layers.Bidirectional(LSTM(64)),
        layers.Dense(32, activation='relu'),
        layers.Dropout(0.5),
        layers.Dense(1, activation='sigmoid')
    ])
    
    model.compile(
        optimizer='adam',
        loss='binary_crossentropy',
        metrics=['accuracy']
    )

Text Generation
---------------

Character-level text generation:

.. code-block:: python

    model = models.Sequential([
        layers.Embedding(vocab_size, 256, input_length=seq_length),
        layers.LSTM(128, return_sequences=True),
        layers.LSTM(128),
        layers.Dense(vocab_size, activation='softmax')
    ])
    
    # Generate text
    def generate_text(seed_text, num_chars):
        for _ in range(num_chars):
            # Convert to sequences
            sequences = tokenizer.texts_to_sequences([seed_text])
            padded = pad_sequences(sequences, maxlen=seq_length)
            
            # Predict next character
            preds = model.predict(padded, verbose=0)
            pred_index = np.argmax(preds[0])
            
            # Convert back to character
            next_char = index_to_char[pred_index]
            seed_text += next_char
            
        return seed_text

Machine Translation
-----------------

Sequence-to-sequence models:

.. code-block:: python

    # Encoder
    encoder_inputs = layers.Input(shape=(None,))
    encoder_embedding = layers.Embedding(vocab_size, 256)(encoder_inputs)
    encoder_lstm = layers.LSTM(512, return_state=True)
    _, state_h, state_c = encoder_lstm(encoder_embedding)
    encoder_states = [state_h, state_c]
    
    # Decoder
    decoder_inputs = layers.Input(shape=(None,))
    decoder_embedding = layers.Embedding(vocab_size, 256)(decoder_inputs)
    decoder_lstm = layers.LSTM(512, return_sequences=True, return_state=True)
    decoder_outputs, _, _ = decoder_lstm(decoder_embedding, initial_state=encoder_states)
    decoder_dense = layers.Dense(vocab_size, activation='softmax')
    decoder_outputs = decoder_dense(decoder_outputs)
    
    model = Model([encoder_inputs, decoder_inputs], decoder_outputs)

Practical Tips
--------------

1. **Use Pre-trained Models**: Better performance with less data
2. **Attention Mechanisms**: Improve performance on long sequences
3. **Layer Normalization**: Stabilize training
4. **Gradient Clipping**: Prevent exploding gradients
5. **Teacher Forcing**: Improve sequence generation

Common Challenges
-----------------

1. **Vanishing Gradients**: Use LSTM/GRU or residual connections
2. **Overfitting**: Use dropout, regularization
3. **Long Sequences**: Use attention or transformers
4. **Computational Cost**: Use smaller models or truncation

Applications
-----------

- Machine translation
- Chatbots and dialogue systems
- Text summarization
- Sentiment analysis
- Question answering
- Document classification
- Speech recognition