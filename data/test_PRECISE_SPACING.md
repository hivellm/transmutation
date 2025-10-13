Provided proper attribution is provided Google hereby grants permission to

reproduce the tables and figures in this paper solely for use in journalistic or

scholarly works

Attention Is All You Need

3

2

0

2

g

u

∗ ∗ ∗ ∗Ashish Vaswani Noam Shazeer Niki Parmar Jakob UszkoreitA

Google Brain Google Brain Google Research Google Research

2 avaswani google com noam google com nikip google com usz google com

∗ ∗ † ∗Llion Jones Aidan N Gomez Łukasz Kaiser

L Google Research University of Toronto Google Brain

C lukaszkaiser google comllion google com aidan cs toronto edu

s

∗ ‡Illia Polosukhinc

illia polosukhin gmail com

7

v Abstract

2

6

The dominant sequence transduction models are based on complex recurrent or7

convolutional neural networks that include an encoder and a decoder The best3

performing models also connect the encoder and decoder through an attention0

mechanism We propose a new simple network architecture the Transformer

6 based solely on attention mechanisms dispensing with recurrence and convolutions

0 entirely Experiments on two machine translation tasks show these models to

7 be superior in quality while being more parallelizable and requiring significantly

1 less time to train Our model achieves 28 4 BLEU on the WMT 2014 English

to German translation task improving over the existing best results includingv

ensembles by over 2 BLEU On the WMT 2014 English to French translation taski

our model establishes a new single model state of the art BLEU score of 41 8 afterX

training for 3 5 days on eight GPUs a small fraction of the training costs of ther

best models from the literature We show that the Transformer generalizes well toa

other tasks by applying it successfully to English constituency parsing both with

large and limited training data

∗Equal contribution Listing order is random Jakob proposed replacing RNNs with self attention and started

the effort to evaluate this idea Ashish with Illia designed and implemented the first Transformer models and

has been crucially involved in every aspect of this work Noam proposed scaled dot product attention multi head

attention and the parameter free position representation and became the other person involved in nearly every

detail Niki designed implemented tuned and evaluated countless model variants in our original codebase and

tensor2tensor Llion also experimented with novel model variants was responsible for our initial codebase and

efficient inference and visualizations Lukasz and Aidan spent countless long days designing various parts of and

implementing tensor2tensor replacing our earlier codebase greatly improving results and massively accelerating

our research

†Work performed while at Google Brain

‡Work performed while at Google Research

31st Conference on Neural Information Processing Systems NIPS 2017 Long Beach CA USA

1 Introduction

Recurrent neural networks long short term memory 13 and gated recurrent 7 neural networks

in particular have been firmly established as state of the art approaches in sequence modeling and

transduction problems such as language modeling and machine translation 35 2 5 Numerous

efforts have since continued to push the boundaries of recurrent language models and encoder decoder

architectures 38 24 15

Recurrent models typically factor computation along the symbol positions of the input and output

sequences Aligning the positions to steps in computation time they generate a sequence of hidden

h h tstates as a function of the previous hidden state and the input for position This inherentlyt t 1

sequential nature precludes parallelization within training examples which becomes critical at longer

sequence lengths as memory constraints limit batching across examples Recent work has achieved

significant improvements in computational efficiency through factorization tricks 21 and conditional

computation 32 while also improving model performance in case of the latter The fundamental

constraint of sequential computation however remains

Attention mechanisms have become an integral part of compelling sequence modeling and transduc

tion models in various tasks allowing modeling of dependencies without regard to their distance in

the input or output sequences 2 19 In all but a few cases 27 however such attention mechanisms

are used in conjunction with a recurrent network

In this work we propose the Transformer a model architecture eschewing recurrence and instead

relying entirely on an attention mechanism to draw global dependencies between input and output

The Transformer allows for significantly more parallelization and can reach a new state of the art in

translation quality after being trained for as little as twelve hours on eight P100 GPUs

2 Background

The goal of reducing sequential computation also forms the foundation of the Extended Neural GPU

16 ByteNet 18 and ConvS2S 9 all of which use convolutional neural networks as basic building

block computing hidden representations in parallel for all input and output positions In these models

the number of operations required to relate signals from two arbitrary input or output positions grows

in the distance between positions linearly for ConvS2S and logarithmically for ByteNet This makes

it more difficult to learn dependencies between distant positions 12 In the Transformer this is

reduced to a constant number of operations albeit at the cost of reduced effective resolution due

to averaging attention weighted positions an effect we counteract with Multi Head Attention as

described in section 3 2

Self attention sometimes called intra attention is an attention mechanism relating different positions

of a single sequence in order to compute a representation of the sequence Self attention has been

used successfully in a variety of tasks including reading comprehension abstractive summarization

textual entailment and learning task independent sentence representations 4 27 28 22

End to end memory networks are based on a recurrent attention mechanism instead of sequence

aligned recurrence and have been shown to perform well on simple language question answering and

language modeling tasks 34

To the best of our knowledge however the Transformer is the first transduction model relying

entirely on self attention to compute representations of its input and output without using sequence

aligned RNNs or convolution In the following sections we will describe the Transformer motivate

self attention and discuss its advantages over models such as 17 18 and 9

3 Model Architecture

Most competitive neural sequence transduction models have an encoder decoder structure 5 2 35

x xHere the encoder maps an input sequence of symbol representations to a sequence1 n

z zz zof continuous representations Given the decoder then generates an output1 n

y ysequence of symbols one element at a time At each step the model is auto regressive1 m

10 consuming the previously generated symbols as additional input when generating the next

2

Figure 1 The Transformer model architecture

The Transformer follows this overall architecture using stacked self attention and point wise fully

connected layers for both the encoder and decoder shown in the left and right halves of Figure 1

respectively

3 1 Encoder and Decoder Stacks

N 6The encoder is composed of a stack of identical layers Each layer has twoEncoder

sub layers The first is a multi head self attention mechanism and the second is a simple position

wise fully connected feed forward network We employ a residual connection 11 around each of

the two sub layers followed by layer normalization 1 That is the output of each sub layer is

LayerNorm x Sublayer x Sublayer xwhere is the function implemented by the sub layer

itself To facilitate these residual connections all sub layers in the model as well as the embedding

d 512layers produce outputs of dimension model

N 6The decoder is also composed of a stack of identical layers In addition to the twoDecoder

sub layers in each encoder layer the decoder inserts a third sub layer which performs multi head

attention over the output of the encoder stack Similar to the encoder we employ residual connections

around each of the sub layers followed by layer normalization We also modify the self attention

sub layer in the decoder stack to prevent positions from attending to subsequent positions This

masking combined with fact that the output embeddings are offset by one position ensures that the

i ipredictions for position can depend only on the known outputs at positions less than

3 2 Attention

An attention function can be described as mapping a query and a set of key value pairs to an output

where the query keys values and output are all vectors The output is computed as a weighted sum

3

Scaled Dot Product Attention Multi Head Attention

Figure 2 left Scaled Dot Product Attention right Multi Head Attention consists of several

attention layers running in parallel

of the values where the weight assigned to each value is computed by a compatibility function of the

query with the corresponding key

3 2 1 Scaled Dot Product Attention

We call our particular attention Scaled Dot Product Attention Figure 2 The input consists of

d dqueries and keys of dimension and values of dimension We compute the dot products of thek v√

dquery with all keys divide each by and apply a softmax function to obtain the weights on thek

values

In practice we compute the attention function on a set of queries simultaneously packed together

Q K Vinto a matrix The keys and values are also packed together into matrices and We compute

the matrix of outputs as

TQK

√Attention Q K V softmax V 1

dk

The two most commonly used attention functions are additive attention 2 and dot product multi

plicative attention Dot product attention is identical to our algorithm except for the scaling factor

1√of Additive attention computes the compatibility function using a feed forward network with

dk

a single hidden layer While the two are similar in theoretical complexity dot product attention is

much faster and more space efficient in practice since it can be implemented using highly optimized

matrix multiplication code

dWhile for small values of the two mechanisms perform similarly additive attention outperformsk

ddot product attention without scaling for larger values of 3 We suspect that for large values ofk

d the dot products grow large in magnitude pushing the softmax function into regions where it hask

14 √extremely small gradients To counteract this effect we scale the dot products by

dk

3 2 2 Multi Head Attention

dInstead of performing a single attention function with dimensional keys values and queriesmodel

hwe found it beneficial to linearly project the queries keys and values times with different learned

d d dlinear projections to and dimensions respectively On each of these projected versions ofk k v

dqueries keys and values we then perform the attention function in parallel yielding dimensionalv

4 q kTo illustrate why the dot products get large assume that the components of and are independent random

dk∑·0 1 q k q k 0 dvariables with mean and variance Then their dot product has mean and variance

i i ki 1

4

output values These are concatenated and once again projected resulting in the final values as

depicted in Figure 2

Multi head attention allows the model to jointly attend to information from different representation

subspaces at different positions With a single attention head averaging inhibits this

OMultiHead Q K V Concat head head W

1 h

Q K V

head Attention QW KW V Wwhere i i ii

Q × × ×d d K d d V d dvk kmodel model modelR R R

∈ ∈ ∈W W WWhere the projections are parameter matrices i ii

×O hd dv modelR∈Wand

h 8In this work we employ parallel attention layers or heads For each of these we use

d d d h 64 Due to the reduced dimension of each head the total computational costk v model

is similar to that of single head attention with full dimensionality

3 2 3 Applications of Attention in our Model

The Transformer uses multi head attention in three different ways

• In encoder decoder attention layers the queries come from the previous decoder layer

and the memory keys and values come from the output of the encoder This allows every

position in the decoder to attend over all positions in the input sequence This mimics the

typical encoder decoder attention mechanisms in sequence to sequence models such as

38 2 9

• The encoder contains self attention layers In a self attention layer all of the keys values

and queries come from the same place in this case the output of the previous layer in the

encoder Each position in the encoder can attend to all positions in the previous layer of the

encoder

• Similarly self attention layers in the decoder allow each position in the decoder to attend to

all positions in the decoder up to and including that position We need to prevent leftward

information flow in the decoder to preserve the auto regressive property We implement this

∞inside of scaled dot product attention by masking out setting to all values in the input

of the softmax which correspond to illegal connections See Figure 2

3 3 Position wise Feed Forward Networks

In addition to attention sub layers each of the layers in our encoder and decoder contains a fully

connected feed forward network which is applied to each position separately and identically This

consists of two linear transformations with a ReLU activation in between

FFN x max 0 xW b W b 21 1 2 2

While the linear transformations are the same across different positions they use different parameters

from layer to layer Another way of describing this is as two convolutions with kernel size 1

d 512The dimensionality of input and output is and the inner layer has dimensionalitymodel

d 2048ff

3 4 Embeddings and Softmax

Similarly to other sequence transduction models we use learned embeddings to convert the input

dtokens and output tokens to vectors of dimension We also use the usual learned linear transformodel

mation and softmax function to convert the decoder output to predicted next token probabilities In

our model we share the same weight matrix between the two embedding layers and the pre softmax√

dlinear transformation similar to 30 In the embedding layers we multiply those weights by model

5

Table 1 Maximum path lengths per layer complexity and minimum number of sequential operations

n d kfor different layer types is the sequence length is the representation dimension is the kernel

rsize of convolutions and the size of the neighborhood in restricted self attention

Layer Type Complexity per Layer Sequential Maximum Path Length

Operations

2 ·O n d O 1 O 1Self Attention

2·O n d O n O nRecurrent

2· ·O k n d O 1 O log nConvolutional

k

· ·O r n d O 1 O n rSelf Attention restricted

3 5 Positional Encoding

Since our model contains no recurrence and no convolution in order for the model to make use of the

order of the sequence we must inject some information about the relative or absolute position of the

tokens in the sequence To this end we add positional encodings to the input embeddings at the

dbottoms of the encoder and decoder stacks The positional encodings have the same dimension model

as the embeddings so that the two can be summed There are many choices of positional encodings

learned and fixed 9

In this work we use sine and cosine functions of different frequencies

2i dmodelPE sin pos 10000

pos 2i

2i dmodelPE cos pos 10000

pos 2i 1

pos iwhere is the position and is the dimension That is each dimension of the positional encoding

·2π 10000 2πcorresponds to a sinusoid The wavelengths form a geometric progression from to We

chose this function because we hypothesized it would allow the model to easily learn to attend by

k PErelative positions since for any fixed offset can be represented as a linear function ofpos k

PEpos

We also experimented with using learned positional embeddings 9 instead and found that the two

versions produced nearly identical results see Table 3 row E We chose the sinusoidal version

because it may allow the model to extrapolate to sequence lengths longer than the ones encountered

during training

4 Why Self Attention

In this section we compare various aspects of self attention layers to the recurrent and convolu

tional layers commonly used for mapping one variable length sequence of symbol representations

dR∈x x z z x zto another sequence of equal length with such as a hidden

1 n 1 n i i

layer in a typical sequence transduction encoder or decoder Motivating our use of self attention we

consider three desiderata

One is the total computational complexity per layer Another is the amount of computation that can

be parallelized as measured by the minimum number of sequential operations required

The third is the path length between long range dependencies in the network Learning long range

dependencies is a key challenge in many sequence transduction tasks One key factor affecting the

ability to learn such dependencies is the length of the paths forward and backward signals have to

traverse in the network The shorter these paths between any combination of positions in the input

and output sequences the easier it is to learn long range dependencies 12 Hence we also compare

the maximum path length between any two input and output positions in networks composed of the

different layer types

As noted in Table 1 a self attention layer connects all positions with a constant number of sequentially

O nexecuted operations whereas a recurrent layer requires sequential operations In terms of

computational complexity self attention layers are faster than recurrent layers when the sequence

6

n dlength is smaller than the representation dimensionality which is most often the case with

sentence representations used by state of the art models in machine translations such as word piece

38 and byte pair 31 representations To improve computational performance for tasks involving

rvery long sequences self attention could be restricted to considering only a neighborhood of size in

the input sequence centered around the respective output position This would increase the maximum

O n rpath length to We plan to investigate this approach further in future work

k nA single convolutional layer with kernel width does not connect all pairs of input and output

O n kpositions Doing so requires a stack of convolutional layers in the case of contiguous kernels

O log nor in the case of dilated convolutions 18 increasing the length of the longest pathsk

between any two positions in the network Convolutional layers are generally more expensive than

krecurrent layers by a factor of Separable convolutions 6 however decrease the complexity

2· · ·O k n d n d k nconsiderably to Even with however the complexity of a separable

convolution is equal to the combination of a self attention layer and a point wise feed forward layer

the approach we take in our model

Assidebenefit self attentioncouldyieldmoreinterpretablemodels Weinspectattentiondistributions

from our models and present and discuss examples in the appendix Not only do individual attention

heads clearly learn to perform different tasks many appear to exhibit behavior related to the syntactic

and semantic structure of the sentences

5 Training

This section describes the training regime for our models

5 1 Training Data and Batching

We trained on the standard WMT 2014 English German dataset consisting of about 4 5 million

sentence pairs Sentences were encoded using byte pair encoding 3 which has a shared source

target vocabulary of about 37000 tokens For English French we used the significantly larger WMT

2014 English French dataset consisting of 36M sentences and split tokens into a 32000 word piece

vocabulary 38 Sentence pairs were batched together by approximate sequence length Each training

batch contained a set of sentence pairs containing approximately 25000 source tokens and 25000

target tokens

5 2 Hardware and Schedule

We trained our models on one machine with 8 NVIDIA P100 GPUs For our base models using

the hyperparameters described throughout the paper each training step took about 0 4 seconds We

trained the base models for a total of 100 000 steps or 12 hours For our big models described on the

bottom line of table 3 step time was 1 0 seconds The big models were trained for 300 000 steps

3 5 days

5 3 Optimizer

9β 0 9 β 0 98 ϵ 10We used the Adam optimizer 20 with and We varied the learning

1 2

rate over the course of training according to the formula

0 5 0 5 1 5· ·lrate d min step num step num warmup steps 3

model

warmup stepsThis corresponds to increasing the learning rate linearly for the first training steps

and decreasing it thereafter proportionally to the inverse square root of the step number We used

warmup steps 4000

5 4 Regularization

We employ three types of regularization during training

7

Table 2 The Transformer achieves better BLEU scores than previous state of the art models on the

English to German and English to French newstest2014 tests at a fraction of the training cost

BLEU Training Cost FLOPs

Model

EN DE EN FR EN DE EN FR

ByteNet 18 23 75

20·1 0 10Deep Att PosUnk 39 39 2

19 20· ·2 3 10 1 4 10GNMT RL 38 24 6 39 92

18 20· ·9 6 10 1 5 10ConvS2S 9 25 16 40 46

19 20· ·2 0 10 1 2 10MoE 32 26 03 40 56

20·8 0 10Deep Att PosUnk Ensemble 39 40 4

20 21· ·1 8 10 1 1 10GNMT RL Ensemble 38 26 30 41 16

19 21· ·7 7 10 1 2 10ConvS2S Ensemble 9 26 36 41 29

18·3 3 10Transformer base model 27 3 38 1

19·2 3 10Transformer big 28 4 41 8

We apply dropout 33 to the output of each sub layer before it is added to theResidual Dropout

sub layer input and normalized In addition we apply dropout to the sums of the embeddings and the

positional encodings in both the encoder and decoder stacks For the base model we use a rate of

P 0 1drop

ϵ 0 1During training we employed label smoothing of value 36 ThisLabel Smoothing ls

hurts perplexity as the model learns to be more unsure but improves accuracy and BLEU score

6 Results

6 1 Machine Translation

On the WMT 2014 English to German translation task the big transformer model Transformer big

2 0in Table 2 outperforms the best previously reported models including ensembles by more than

28 4BLEU establishing a new state of the art BLEU score of The configuration of this model is

3 5 8listed in the bottom line of Table 3 Training took days on P100 GPUs Even our base model

surpasses all previously published models and ensembles at a fraction of the training cost of any of

the competitive models

41 0On the WMT 2014 English to French translation task our big model achieves a BLEU score of

1 4outperforming all of the previously published single models at less than the training cost of the

previous state of the art model The Transformer big model trained for English to French used

P 0 1 0 3dropout rate instead ofdrop

For the base models we used a single model obtained by averaging the last 5 checkpoints which

were written at 10 minute intervals For the big models we averaged the last 20 checkpoints We

4 α 0 6used beam search with a beam size of and length penalty 38 These hyperparameters

were chosen after experimentation on the development set We set the maximum output length during

50inference to input length but terminate early when possible 38

Table 2 summarizes our results and compares our translation quality and training costs to other model

architectures from the literature We estimate the number of floating point operations used to train a

model by multiplying the training time the number of GPUs used and an estimate of the sustained

5single precision floating point capacity of each GPU

6 2 Model Variations

To evaluate the importance of different components of the Transformer we varied our base model

in different ways measuring the change in performance on English to German translation on the

5We used values of 2 8 3 7 6 0 and 9 5 TFLOPS for K80 K40 M40 and P100 respectively

8

Table 3 Variations on the Transformer architecture Unlisted values are identical to those of the base

model All metrics are on the English to German translation development set newstest2013 Listed

perplexities are per wordpiece according to our byte pair encoding and should not be compared to

per word perplexities

train PPL BLEU params

N d d h d d P ϵk v drop lsmodel ff 6

×10steps dev dev

base 6 512 2048 8 64 64 0 1 0 1 100K 4 92 25 8 65

1 512 512 5 29 24 9

4 128 128 5 00 25 5

A

16 32 32 4 91 25 8

32 16 16 5 01 25 4

16 5 16 25 1 58

B

32 5 01 25 4 60

2 6 11 23 7 36

4 5 19 25 3 50

8 4 88 25 5 80

C 256 32 32 5 75 24 5 28

1024 128 128 4 66 26 0 168

1024 5 12 25 4 53

4096 4 75 26 2 90

0 0 5 77 24 6

0 2 4 95 25 5

D

0 0 4 67 25 3

0 2 5 47 25 7

E positional embedding instead of sinusoids 4 92 25 7

big 6 1024 4096 16 0 3 300K 2134 33 26 4

development set newstest2013 We used beam search as described in the previous section but no

checkpoint averaging We present these results in Table 3

In Table 3 rows A we vary the number of attention heads and the attention key and value dimensions

keeping the amount of computation constant as described in Section 3 2 2 While single head

attention is 0 9 BLEU worse than the best setting quality also drops off with too many heads

dIn Table 3 rows B we observe that reducing the attention key size hurts model quality Thisk

suggests that determining compatibility is not easy and that a more sophisticated compatibility

function than dot product may be beneficial We further observe in rows C and D that as expected

bigger models are better and dropout is very helpful in avoiding over fitting In row E we replace our

sinusoidal positional encoding with learned positional embeddings 9 and observe nearly identical

results to the base model

6 3 English Constituency Parsing

To evaluate if the Transformer can generalize to other tasks we performed experiments on English

constituency parsing This task presents specific challenges the output is subject to strong structural

constraints and is significantly longer than the input Furthermore RNN sequence to sequence

models have not been able to attain state of the art results in small data regimes 37

d 1024We trained a 4 layer transformer with on the Wall Street Journal WSJ portion of themodel

Penn Treebank 25 about 40K training sentences We also trained it in a semi supervised setting

using the larger high confidence and BerkleyParser corpora from with approximately 17M sentences

37 We used a vocabulary of 16K tokens for the WSJ only setting and a vocabulary of 32K tokens

for the semi supervised setting

We performed only a small number of experiments to select the dropout both attention and residual

section 5 4 learning rates and beam size on the Section 22 development set all other parameters

remained unchanged from the English to German base translation model During inference we

9

Table 4 The Transformer generalizes well to English constituency parsing Results are on Section 23

of WSJ

Parser Training WSJ 23 F1

Vinyals Kaiser el al 2014 37 WSJ only discriminative 88 3

Petrov et al 2006 29 WSJ only discriminative 90 4

Zhu et al 2013 40 WSJ only discriminative 90 4

Dyer et al 2016 8 WSJ only discriminative 91 7

Transformer 4 layers WSJ only discriminative 91 3

Zhu et al 2013 40 semi supervised 91 3

Huang Harper 2009 14 semi supervised 91 3

McClosky et al 2006 26 semi supervised 92 1

Vinyals Kaiser el al 2014 37 semi supervised 92 1

Transformer 4 layers semi supervised 92 7

Luong et al 2015 23 multi task 93 0

Dyer et al 2016 8 generative 93 3

300 21 α 0 3increased the maximum output length to input length We used a beam size of and

for both WSJ only and the semi supervised setting

Our results in Table 4 show that despite the lack of task specific tuning our model performs sur

prisingly well yielding better results than all previously reported models with the exception of the

Recurrent Neural Network Grammar 8

In contrast to RNN sequence to sequence models 37 the Transformer outperforms the Berkeley

Parser 29 even when training only on the WSJ training set of 40K sentences

7 Conclusion

In this work we presented the Transformer the first sequence transduction model based entirely on

attention replacing the recurrent layers most commonly used in encoder decoder architectures with

multi headed self attention

For translation tasks the Transformer can be trained significantly faster than architectures based

on recurrent or convolutional layers On both WMT 2014 English to German and WMT 2014

English to French translation tasks we achieve a new state of the art In the former task our best

model outperforms even all previously reported ensembles

We are excited about the future of attention based models and plan to apply them to other tasks We

plan to extend the Transformer to problems involving input and output modalities other than text and

to investigate local restricted attention mechanisms to efficiently handle large inputs and outputs

such as images audio and video Making generation less sequential is another research goals of ours

The code we used to train and evaluate our models is available at https github com

tensorflow tensor2tensor

We are grateful to Nal Kalchbrenner and Stephan Gouws for their fruitfulAcknowledgements

comments corrections and inspiration

References

arXiv preprint1 Jimmy Lei Ba Jamie Ryan Kiros and Geoffrey E Hinton Layer normalization

arXiv 1607 06450 2016

2 Dzmitry Bahdanau Kyunghyun Cho and Yoshua Bengio Neural machine translation by jointly

CoRRlearning to align and translate abs 1409 0473 2014

3 Denny Britz Anna Goldie Minh Thang Luong and Quoc V Le Massive exploration of neural

CoRRmachine translation architectures abs 1703 03906 2017

4 Jianpeng Cheng Li Dong and Mirella Lapata Long short term memory networks for machine

arXiv preprint arXiv 1601 06733reading 2016

10

5 Kyunghyun Cho Bart van Merrienboer Caglar Gulcehre Fethi Bougares Holger Schwenk

and Yoshua Bengio Learning phrase representations using rnn encoder decoder for statistical

CoRRmachine translation abs 1406 1078 2014

arXiv6 Francois Chollet Xception Deep learning with depthwise separable convolutions

preprint arXiv 1610 02357 2016

7 Junyoung Chung Çaglar Gülçehre Kyunghyun Cho and Yoshua Bengio Empirical evaluation

CoRRof gated recurrent neural networks on sequence modeling abs 1412 3555 2014

8 Chris Dyer Adhiguna Kuncoro Miguel Ballesteros and Noah A Smith Recurrent neural

Proc of NAACLnetwork grammars In 2016

9 Jonas Gehring Michael Auli David Grangier Denis Yarats and Yann N Dauphin Convolu

arXiv preprint arXiv 1705 03122v2tional sequence to sequence learning 2017

arXiv preprint10 Alex Graves Generating sequences with recurrent neural networks

arXiv 1308 0850 2013

11 Kaiming He Xiangyu Zhang Shaoqing Ren and Jian Sun Deep residual learning for im

Proceedings of the IEEE Conference on Computer Vision and Patternage recognition In

Recognition pages 770 778 2016

12 Sepp Hochreiter Yoshua Bengio Paolo Frasconi and Jürgen Schmidhuber Gradient flow in

recurrent nets the difficulty of learning long term dependencies 2001

Neural computation13 Sepp Hochreiter and Jürgen Schmidhuber Long short term memory

9 8 1735 1780 1997

14 Zhongqiang Huang and Mary Harper Self training PCFG grammars with latent annotations

Proceedings of the 2009 Conference on Empirical Methods in Naturalacross languages In

Language Processing pages 832 841 ACL August 2009

15 Rafal Jozefowicz Oriol Vinyals Mike Schuster Noam Shazeer and Yonghui Wu Exploring

arXiv preprint arXiv 1602 02410the limits of language modeling 2016

Advances in Neural16 Łukasz Kaiser and Samy Bengio Can active memory replace attention In

Information Processing Systems NIPS 2016

International Conference17 Łukasz Kaiser and Ilya Sutskever Neural GPUs learn algorithms In

on Learning Representations ICLR 2016

18 Nal Kalchbrenner Lasse Espeholt Karen Simonyan Aaron van den Oord Alex Graves and Ko

arXiv preprint arXiv 1610 10099v2ray Kavukcuoglu Neural machine translation in linear time

2017

19 Yoon Kim Carl Denton Luong Hoang and Alexander M Rush Structured attention networks

International Conference on Learning RepresentationsIn 2017

ICLR20 Diederik Kingma and Jimmy Ba Adam A method for stochastic optimization In 2015

arXiv preprint21 Oleksii Kuchaiev and Boris Ginsburg Factorization tricks for LSTM networks

arXiv 1703 10722 2017

22 Zhouhan Lin Minwei Feng Cicero Nogueira dos Santos Mo Yu Bing Xiang Bowen

arXiv preprintZhou and Yoshua Bengio A structured self attentive sentence embedding

arXiv 1703 03130 2017

23 Minh Thang Luong Quoc V Le Ilya Sutskever Oriol Vinyals and Lukasz Kaiser Multi task

arXiv preprint arXiv 1511 06114sequence to sequence learning 2015

24 Minh Thang Luong Hieu Pham and Christopher D Manning Effective approaches to attention

arXiv preprint arXiv 1508 04025based neural machine translation 2015

11

25 Mitchell P Marcus Mary Ann Marcinkiewicz and Beatrice Santorini Building a large annotated

Computational linguisticscorpus of english The penn treebank 19 2 313 330 1993

26 David McClosky Eugene Charniak and Mark Johnson Effective self training for parsing In

Proceedings of the Human Language Technology Conference of the NAACL Main Conference

pages 152 159 ACL June 2006

27 Ankur Parikh Oscar Täckström Dipanjan Das and Jakob Uszkoreit A decomposable attention

Empirical Methods in Natural Language Processingmodel In 2016

28 Romain Paulus Caiming Xiong and Richard Socher A deep reinforced model for abstractive

arXiv preprint arXiv 1705 04304summarization 2017

29 Slav Petrov Leon Barrett Romain Thibaux and Dan Klein Learning accurate compact

Proceedings of the 21st International Conference onand interpretable tree annotation In

Computational Linguistics and 44th Annual Meeting of the ACL pages 433 440 ACL July

2006

arXiv30 Ofir Press and Lior Wolf Using the output embedding to improve language models

preprint arXiv 1608 05859 2016

31 Rico Sennrich Barry Haddow and Alexandra Birch Neural machine translation of rare words

arXiv preprint arXiv 1508 07909with subword units 2015

32 Noam Shazeer Azalia Mirhoseini Krzysztof Maziarz Andy Davis Quoc Le Geoffrey Hinton

and Jeff Dean Outrageously large neural networks The sparsely gated mixture of experts

arXiv preprint arXiv 1701 06538layer 2017

33 Nitish Srivastava Geoffrey E Hinton Alex Krizhevsky Ilya Sutskever and Ruslan Salakhutdi

Journal of Machinenov Dropout a simple way to prevent neural networks from overfitting

Learning Research 15 1 1929 1958 2014

34 Sainbayar Sukhbaatar Arthur Szlam Jason Weston and Rob Fergus End to end memory

networks In C Cortes N D Lawrence D D Lee M Sugiyama and R Garnett editors

Advances in Neural Information Processing Systems 28 pages 2440 2448 Curran Associates

Inc 2015

35 Ilya Sutskever Oriol Vinyals and Quoc VV Le Sequence to sequence learning with neural

Advances in Neural Information Processing Systemsnetworks In pages 3104 3112 2014

36 Christian Szegedy Vincent Vanhoucke Sergey Ioffe Jonathon Shlens and Zbigniew Wojna

CoRRRethinking the inception architecture for computer vision abs 1512 00567 2015

37 Vinyals Kaiser Koo Petrov Sutskever and Hinton Grammar as a foreign language In

Advances in Neural Information Processing Systems 2015

38 Yonghui Wu Mike Schuster Zhifeng Chen Quoc V Le Mohammad Norouzi Wolfgang

Macherey Maxim Krikun Yuan Cao Qin Gao Klaus Macherey et al Google s neural machine

arXiv preprinttranslation system Bridging the gap between human and machine translation

arXiv 1609 08144 2016

39 Jie Zhou Ying Cao Xuguang Wang Peng Li and Wei Xu Deep recurrent models with

CoRRfast forward connections for neural machine translation abs 1606 04199 2016

40 Muhua Zhu Yue Zhang Wenliang Chen Min Zhang and Jingbo Zhu Fast and accurate

Proceedings of the 51st Annual Meeting of the ACL Volumeshift reduce constituent parsing In

1 Long Papers pages 434 443 ACL August 2013

12

Input Input Layer5Attention Visualizations

st

n ne o

n ita m

sy tat d gc ln si ri

rr te gnr u Seit e e d d dd d d9 se e si o n c

e s cit j k ri icr O a a aa a av sv w 0 ts i fog

i a ea a ow fnm E p p pp ppo a r rf a e 0 iop i eh h ht ns a

i t t m g p s m o v pI i s a o A h n l 2 t r m d

t t f tt rs s li ssy sI a e en en e gd 9 g ni i wi t tr a o o r

sii uv c ha ne 0 n o d d d d d dw Sh er iin ih t ot cep a n tc s t

t 0 a a a a a aiai n i ko e l Ofcs j sr h oa2 p p p p ppm

fs a r ioa va Ee m t r

dmp snm pim r

geA ev

ro

g

Figure 3 An example of the attention mechanism following long distance dependencies in the

encoder self attention in layer 5 of 6 Many of the attention heads attend to a distant dependency of

the verb making completing the phrase making more difficult Attentions here shown only for

the word making Different colors represent different heads Best viewed in color

13

Input Input Layer5

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi h e

h pEa e e e u p e r phs h nt u s

w n p b s j i w i mT L b i a b t w a m o

l t t t trl s s yse e e e ne gd nni i iw

tc s ilu a re ih b b n oo dSh wa

me u iui iw v b h atjf t aT L s

o nre Ow ia ps

hen iInput Input Layer5 pc Ei s

p l om

p

p

a

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l aw v r t p io s sl ei yisi eh

h pEe e ua e p e r phs hu s nt

w n p b s j i w i mT L b i a b t w a m o

l t t t trl s s s ye e e e ne gdn ni i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc Ei s

p l om

p

p

a

Figure 4 Two attention heads also in layer 5 of 6 apparently involved in anaphora resolution Top

Full attentions for head 5 Bottom Isolated attentions from just the word its for attention heads 5

and 6 Note that the attentions are very sharp for this word

14

Input Input Layer5

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi eh

h pEe e ua e p e r phs h nt u s

iL w n b p b i a s b j t i w w a m oT m

l t t t trl s s yse e e nee gd nni i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc EInput Input Layer5i s

p l om

p

p

a

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi eh

h pEe e ua e p e r phs h nt u s

iL w n b p b i a s b j t i w w a m oT m

l t t t trl s s yse e e e ne gd nni i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc Ei s

p l om

p

p

a

Figure 5 Many of the attention heads exhibit behaviour that seems related to the structure of the

sentence We give two such examples above from two different heads from the encoder self attention

at layer 5 of 6 The heads clearly learned to perform different tasks

15