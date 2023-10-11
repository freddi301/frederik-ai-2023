# Building over backpropagation

## Auto linear layers

An auto linear layer automatically adjust its hyperparameters to reduce error, it is meant to automate any linear layer.

It works only increasing the hyperparameters.

The structure is a sequential model of dense layers with a custom activation function

The activation function approximates automatically the following functions:
- linear
- relu
- sigmoid
- tanh

tf.where(
    inputs > 0,
    tf.minimum(inputs, self.upper_treshold),
    tf.maximum(inputs, self.lower_treshold),
)

each hidden layer has normalization:
- L1 + L2 kernel normalization (for overfitting prevention)
- Droput (for overfitting prevention)

- Also consider Residual connections are applied automatically (helps with vanishing gradients in deep networks, useful with "corner cases" in data)

parameters:
- input_dim
- output_dim
auto hyper parameters:
- hidden_layer_count
- neurons_count (indipendent for each hidden layer)

when a hyperparameter is incremented, the layer it belongs to and the next one are reinitialized with new random weights
the other layers are freezed (trainable = false)
and the changed layers are retrained
this is also known as "fine tuning"

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

## Hypothesis validation learning

Let's assume we have an autoencoder

latent_space = encoder(input)
output = decoder(latent_space)
trained with: decoder(encoder(input)) = input

after some training with training set, we can also train it more with
encoder(decoder(random_latent_space)) = random_latent_space

In this case the latent space must be a probability distribution as in variational autoencoders (so we can generate probable latent spaces)

this mimics human thought process of validating discovered rules against unseen observations
also it mimcs bijection between input and latent space

## Image autoencoder

Goals and features:
- accept any kind of size, resolution, ratio images in input and output
- complete images
- denoise images
- expand images (input image size < output image size)
- compress images (input image resolution > output image size)
- super-sample images (input image resolution < output image size)

inputs:
- image of any size, resolution, ratio + mask that says where the image is defined
- mask size, resolution, ratio that says what areas are to be predicted
outputs:
- image of mask size, resolution, ratio with predicted values

## Variable size latent space (aka: Disentangled representations learning)

The goal is to train the network to create the smallest possible latent space that can still be used to reconstruct the input correctly

2D geometric shapes example for explanation:

we generate input samples with a variable number of non overlapping circles (position, radius, color are variable)

ideally the latent space should be of shape (number_of_circles_in_input, 3) (3 for position, radius, color)

## Perception space - latent space principle

Examples of perception spaces:
- 2d images (x, y, rgb)
- videos (x, y, time, rgb)
- 3d pointcloud (x, y, z, rgb)
- weather (x, y, z, time, temperature, humidity, wind, ...)
- text (natural language)

Examples ol latent spaces:
- real world (space, time, matter) space is 3d relative, time is relative, matter molecules or materials, or entities
  - for example a text can describe a real world scenario, so we can query it with text
- symbolic knwoledge about world

the network learns to create the smallest possible latent space that can still be used to reconstruct the input correctly

the network also learns how to translate between perception spaces

training and prediction examples:
- image description: (supervised learning) 2d image -> latent space -> text
- image generation: (supervised learning) text -> latent space -> 2d image
- forecast: (unsupervised learning) space, time, feature -> latent space -> feature

NOTES:

given known observed features these are encoded in disentangled representations, let's call it scenario
this is a generalization that enables to mix and match supervised and unsupervised learning
so we can leverage generalizations in latent space and disentangled representations of unsupervised learning in supervised learning

the model is going to learn functions that can predict a given data point from all the others in a given scenario

then we choose desired query functions, examples:
- next_word(text) what is the next word given some text?
- pixel_color(x, y, incomplete_image) what is the color of a pixel given its position? to complete 2d images
- pixel_color(x, y, time, incomplete_video) what is the color of a pixel given its position and time? to complete videos
- predict_temperature(position, time, weather) what is the temperature given position and time? to forecast weather

FINAL GOAL:
- we have partial observations of the real world
- the model sould find the rules that govern the real world
- so that given some incomplete observations it can predict the missing desired ones
- example:
  - given weather observations on earth, it can predict weather on mars

## Associative memory + feedback loops + categorization

Example:
there are plenty of trainign samples with cats from all angles
given a picture with a cat from an angle where the tail is not visible, and asking the model if there is a tail, it should answer yes

- the input layer is the image
- features layer tells what body parts are on the image
- categorization layer tells that there is a cat
- feedback layer tells all usual body parts of a cat

This mimics catamorphism and anamonorphism in category theory
Also the act of contracting or expanding mathematical expressions to find a solution

## Output refinement

what it is useful for?
- reconsturct a video
- generate a video
- control a robotic arm to achieve a goal

VIDEO example:
as training samples we have some videos.
given two frames of a video from the test samples, reconstruct the video
if a first stage predict all the missing frames based solely on the given frames
the ouput is united with the given frames [give_frame1, stage_1_prediceted_frame2, stage_1_predicted_frame3, given_frame4]
on the next stage predict all the frames based on the united frames from previous stage
this will be the output [give_frame1, stage_2_prediceted_frame2, stage_2_predicted_frame3, given_frame4]
and so on until the loss is minimized, the loss is calculated per frame using a loss function learned from training samples

Notes: this could be done listing the frames in latent space

ROBOTIC ARM example:
as training samples we have (actuator_value, sensor_value) pairs over time
the prediction is: actuator_value = predict(starting_sensor_value, desired_sensor_value)
as from previous example, it is like a video where all the frames are filled, extracting the actuator_value from the result gives what the robotic arm should do