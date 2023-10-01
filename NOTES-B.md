# Building over backpropagation

## Auto linear layers

An auto linear layer automatically adjust its hyperparameters to reduce error, it is meant to automate any linear layer.

It has some dense hidden layers with a growable number neurons

there are two types hidden neurons in the hidden layers
- with prelu activation function (for continuous values) (better than relu for vanishing gradients)
- with tanh activation function (for probality values) (better than sigmoid for vanishing gradients)

Droput layer is applied after every hidden layer (overfitting prevention, better generalization)
Also Residual connections are applied automatically (helps with vanishing gradients in deep networks, useful with "corner cases" in data)

parameters:
- input_dim
- output_dim
auto hyper parameters:
- hidden_layer_count
- prelu_neurons_count (indipendent for each hidden layer)
- tanh_neurons_count (indipendent for each hidden layer)

training algorithm:
- loss function: mean squared error (popular, generic)
- optimizer: adam (popular, generic)
- train with current hyperparameters until loss reduction is significant (use formula to address diminishing returns)
- increment a hyperparameter and train as before
- keep incrementing those hyperparameters that reduces the loss the most

## Attention Head variations

Aims to choose only element from the input based on its relative positional encoding and feture values
This introduces order invariance for variable number of input elements
The positional encoding is relative and it represent space and time

The query is computed with AutoLinearLayer(input_shape=features_count, output_shape=1)
The key is computed with AutoLinearLayer(input_shape=realtive_positional_encoding_features_count, output_shape=features_count)
The value is compued with softmax


## Generl architecture

ENCODER-LATENSPACE-DECODER principle

UNSUPERVISED LEARNING: when there is no labeled data, the network is going to learn relationships between every piece of data
for example: given a set of numerical observations, it learns how to predict one observations from the others

POSITONAL DATA:
for example images, text, events (where and when) are encoded in a variable number of input features, then relative positionning and attention heads are used
to shape a function that can predict features in any given position (so reusing weights)