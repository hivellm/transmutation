| 31st Conference on Neural Information Processing Systems (NIPS 2017), Long Beach, CA, USA. |
| --- |

| Work performed while at Google Research.++ Work performed while at Google Brain.+ our research. implementing tensor2tensor, replacing our earlier codebase, greatly improving results and massively accelerating efficient inference and visualizations. Lukasz and Aidan spent countless long days designing various parts of and tensor2tensor. Llion also experimented with novel model variants, was responsible for our initial codebase, and detail. Niki designed, implemented, tuned and evaluated countless model variants in our original codebase and attention and the parameter-free position representation and became the other person involved in nearly every has been crucially involved in every aspect of this work. Noam proposed scaled dot-product attention, multi-head the effort to evaluate this idea. Ashish, with Illia, designed and implemented the first Transformer models and Equal contribution. Listing order is random. Jakob proposed replacing RNNs with self-attention and started∗ |
| --- |

| large and limited training data. other tasks by applying it successfully to English constituency parsing both with a best models from the literature. We show that the Transformer generalizes well to r training for 3.5 days on eight GPUs, a small fraction of the training costs of the X our model establishes a new single-model state-of-the-art BLEU score of 41.8 after i ensembles, by over 2 BLEU. On theWMT 2014 English-to-French translation task, v to-German translation task, improving over the existing best results, including : less time to train. Our model achieves 28.4 BLEU on the WMT 2014 English-1 be superior in quality while being more parallelizable and requiring significantly7 entirely. Experiments on two machine translation tasks show these models to0 based solely on attentionmechanisms, dispensing with recurrence and convolutions6 mechanism. We propose a new simple network architecture, the Transformer,. 0 performing models also connect the encoder and decoder through an attention 3 convolutional neural networks that include an encoder and a decoder. The best 7 The dominant sequence transduction models are based on complex recurrent or |
| --- |

6

| 2 Abstract |
| --- |

v 7

| illia.polosukhin@gmail.com |
| --- |

\[

| c Illia Polosukhin∗ ++ |
| --- |

s

| . llion@google.com aidan@cs.toronto.edu lukaszkaiser@google.comC Google Research University of Toronto Google Brain |
| --- |

## L

| ] Llion Jones Aidan N. Gomez Łukasz Kaiser∗ ∗ + ∗ |
| --- |

| avaswani@google.com noam@google.com nikip@google.com usz@google.com2 Google Brain Google Brain Google Research Google Research |
| --- |

| A Ashish Vaswani Noam Shazeer Niki Parmar Jakob Uszkoreit∗ ∗ ∗ ∗ |
| --- |

u g 2
0 2

| 3 Attention Is All You Need |
| --- |

| scholarly works. reproduce the tables and figures in this paper solely for use in journalistic or Provided proper attribution is provided, Google hereby grants permission to |
| --- |

2

| [10], consuming the previously generated symbols as additional input when generating the next. 1 msequence of symbols one element at a time. At each step the model is auto-regressive(y , ..., y ) 1 nof continuous representations . Given , the decoder then generates an outputz z= (z , ..., z ) 1 nHere, the encoder maps an input sequence of symbol representations to a sequence(x , ...,x ) Most competitive neural sequence transduction models have an encoder-decoder structure [5, 2, 35]. 3 Model Architecture |
| --- |

| self-attention and discuss its advantages over models such as [17, 18] and [9]. aligned RNNs or convolution. In the following sections, we will describe the Transformer, motivate entirely on self-attention to compute representations of its input and output without using sequenceTo the best of our knowledge, however, the Transformer is the first transduction model relying language modeling tasks [34]. aligned recurrence and have been shown to perform well on simple-language question answering and End-to-end memory networks are based on a recurrent attention mechanism instead of sequencetextual entailment and learning task-independent sentence representations [4, 27, 28, 22]. used successfully in a variety of tasks including reading comprehension, abstractive summarization, of a single sequence in order to compute a representation of the sequence. Self-attention has been Self-attention, sometimes called intra-attention is an attention mechanism relating different positions described in section 3.2. to averaging attention-weighted positions, an effect we counteract with Multi-Head Attention as reduced to a constant number of operations, albeit at the cost of reduced effective resolution due it more difficult to learn dependencies between distant positions [12]. In the Transformer this is in the distance between positions, linearly for ConvS2S and logarithmically for ByteNet. This makes the number of operations required to relate signals from two arbitrary input or output positions grows block, computing hidden representations in parallel for all input and output positions. In thesemodels, [16], ByteNet [18] and ConvS2S [9], all of which use convolutional neural networks as basic building The goal of reducing sequential computation also forms the foundation of the Extended Neural GPU 2 Background |
| --- |

| translation quality after being trained for as little as twelve hours on eight P100 GPUs. The Transformer allows for significantly more parallelization and can reach a new state of the art in relying entirely on an attention mechanism to draw global dependencies between input and output. In this work we propose the Transformer, a model architecture eschewing recurrence and instead are used in conjunction with a recurrent network. the input or output sequences [2, 19]. In all but a few cases [27], however, such attention mechanisms tion models in various tasks, allowing modeling of dependencies without regard to their distance in Attention mechanisms have become an integral part of compelling sequence modeling and transducconstraint of sequential computation, however, remains. computation [32], while also improving model performance in case of the latter. The fundamental significant improvements in computational efficiency through factorization tricks [21] and conditional sequence lengths, as memory constraints limit batching across examples. Recent work has achieved sequential nature precludes parallelization within training examples, which becomes critical at longer t t 1-states , as a function of the previous hidden state and the input for position . This inherentlyh h t sequences. Aligning the positions to steps in computation time, they generate a sequence of hidden Recurrent models typically factor computation along the symbol positions of the input and output architectures [38, 24, 15]. efforts have since continued to push the boundaries of recurrent languagemodels and encoder-decoder transduction problems such as language modeling and machine translation [35, 2, 5]. Numerous in particular, have been firmly established as state of the art approaches in sequence modeling and Recurrent neural networks, long short-term memory [13] and gated recurrent [7] neural networks 1 Introduction |
| --- |

3

| where the query, keys, values, and output are all vectors. The output is computed as a weighted sum An attention function can be described as mapping a query and a set of key-value pairs to an output, 3.2 Attention predictions for position can depend only on the known outputs at positions less than .i i masking, combined with fact that the output embeddings are offset by one position, ensures that the sub-layer in the decoder stack to prevent positions from attending to subsequent positions. This around each of the sub-layers, followed by layer normalization. We also modify the self-attention attention over the output of the encoder stack. Similar to the encoder, we employ residual connections sub-layers in each encoder layer, the decoder inserts a third sub-layer, which performs multi-head Decoder: The decoder is also composed of a stack of identical layers. In addition to the twoN = 6 modellayers, produce outputs of dimension .d = 512 itself. To facilitate these residual connections, all sub-layers in the model, as well as the embedding , where is the function implemented by the sub-layerLayerNorm(x + Sublayer(x)) Sublayer(x) the two sub-layers, followed by layer normalization [1]. That is, the output of each sub-layer is wise fully connected feed-forward network. We employ a residual connection [11] around each of sub-layers. The first is a multi-head self-attention mechanism, and the second is a simple, positionEncoder: The encoder is composed of a stack of identical layers. Each layer has twoN = 6 3.1 Encoder and Decoder Stacks respectively. connected layers for both the encoder and decoder, shown in the left and right halves of Figure 1, The Transformer follows this overall architecture using stacked self-attention and point-wise, fully |
| --- |

| Figure 1: The Transformer - model architecture. |
| --- |

4

| i=1 ki ivariables with mean and variance . Then their dot product, , has mean and variance .0 1 q k = q k 0 d· sum kd To illustrate why the dot products get large, assume that the components of and are independent randomq k4 vqueries, keys and values we then perform the attention function in parallel, yielding -dimensionald k k vlinear projections to , and dimensions, respectively. On each of these projected versions ofd d d we found it beneficial to linearly project the queries, keys and values times with different, learnedh modelInstead of performing a single attention function with -dimensional keys, values and queries,d 3.2.2 Multi-Head Attention kdextremely small gradients . To counteract this effect, we scale the dot products by .sqrt4 1 k, the dot products grow large in magnitude, pushing the softmax function into regions where it hasd kdot product attention without scaling for larger values of [3]. We suspect that for large values ofd kWhile for small values of the two mechanisms perform similarly, additive attention outperformsd matrix multiplication code. much faster and more space-efficient in practice, since it can be implemented using highly optimized a single hidden layer. While the two are similar in theoretical complexity, dot-product attention is kdof . Additive attention computes the compatibility function using a feed-forward network withsqrt1 plicative) attention. Dot-product attention is identical to our algorithm, except for the scaling factor The two most commonly used attention functions are additive attention [2], and dot-product (multikd (1)Attention(Q,K,V ) = softmax( )Vsqrt |
| --- |

### QKT

| the matrix of outputs as: into a matrix . The keys and values are also packed together into matrices and . We computeQ K V In practice, we compute the attention function on a set of queries simultaneously, packed together values. kquery with all keys, divide each by , and apply a softmax function to obtain the weights on thed sqrtk vqueries and keys of dimension , and values of dimension . We compute the dot products of thed d We call our particular attention "Scaled Dot-Product Attention" (Figure 2). The input consists of 3.2.1 Scaled Dot-Product Attention query with the corresponding key. of the values, where the weight assigned to each value is computed by a compatibility function of the |
| --- |

| attention layers running in parallel. Figure 2: (left) Scaled Dot-Product Attention. (right) Multi-Head Attention consists of several |
| --- |

| Scaled Dot-Product Attention Multi-Head Attention |
| --- |

5

| modellinear transformation, similar to [30]. In the embedding layers, we multiply those weights by .d sqrtour model, we share the same weight matrix between the two embedding layers and the pre-softmax mation and softmax function to convert the decoder output to predicted next-token probabilities. In modeltokens and output tokens to vectors of dimension . We also use the usual learned linear transfor-d Similarly to other sequence transduction models, we use learned embeddings to convert the input 3.4 Embeddings and Softmax ff .d = 2048 modelThe dimensionality of input and output is , and the inner-layer has dimensionalityd = 512 from layer to layer. Another way of describing this is as two convolutions with kernel size 1. While the linear transformations are the same across different positions, they use different parameters 1 1 2 2 (2)FFN(x) = max(0,xW + b )W + b |
| --- |

| consists of two linear transformations with a ReLU activation in between. connected feed-forward network, which is applied to each position separately and identically. This In addition to attention sub-layers, each of the layers in our encoder and decoder contains a fully 3.3 Position-wise Feed-Forward Networks of the softmax which correspond to illegal connections. See Figure 2. inside of scaled dot-product attention by masking out (setting to ) all values in the input-infinity information flow in the decoder to preserve the auto-regressive property. We implement this all positions in the decoder up to and including that position. We need to prevent leftward |
| --- |

· Similarly, self-attention layers in the decoder allow each position in the decoder to attend to

| encoder. encoder. Each position in the encoder can attend to all positions in the previous layer of the and queries come from the same place, in this case, the output of the previous layer in the |
| --- |

· The encoder contains self-attention layers. In a self-attention layer all of the keys, values

| [38, 2, 9]. typical encoder-decoder attention mechanisms in sequence-to-sequence models such as position in the decoder to attend over all positions in the input sequence. This mimics the and the memory keys and values come from the output of the encoder. This allows every |
| --- |

· In "encoder-decoder attention" layers, the queries come from the previous decoder layer,

| The Transformer uses multi-head attention in three different ways: 3.2.3 Applications of Attention in ourModel is similar to that of single-head attention with full dimensionality. modelk v . Due to the reduced dimension of each head, the total computational costd = d = d /h = 64 In this work we employ parallel attention layers, or heads. For each of these we useh = 8 and .W ∈ R modelvO hd dx i i iWhere the projections are parametermatrices , ,W W W∈ ∈ ∈R R Rmodel model modelk k vd d K d d V d dx x xQ |
| --- |

| i i iiwhere head = Attention(QW ,KW ,VW )K VQ 1 hMultiHead(Q,K,V ) = Concat(head , ...,head )WO |
| --- |

| subspaces at different positions. With a single attention head, averaging inhibits this. Multi-head attention allows the model to jointly attend to information from different representation depicted in Figure 2. output values. These are concatenated and once again projected, resulting in the final values, as |
| --- |

6

| computational complexity, self-attention layers are faster than recurrent layers when the sequence executed operations, whereas a recurrent layer requires sequential operations. In terms ofO(n) As noted in Table 1, a self-attention layer connects all positions with a constant number of sequentially different layer types. the maximum path length between any two input and output positions in networks composed of the and output sequences, the easier it is to learn long-range dependencies [12]. Hence we also compare traverse in the network. The shorter these paths between any combination of positions in the input ability to learn such dependencies is the length of the paths forward and backward signals have to dependencies is a key challenge in many sequence transduction tasks. One key factor affecting the The third is the path length between long-range dependencies in the network. Learning long-range be parallelized, as measured by the minimum number of sequential operations required. One is the total computational complexity per layer. Another is the amount of computation that can consider three desiderata. layer in a typical sequence transduction encoder or decoder. Motivating our use of self-attention we 1 n 1 n i ito another sequence of equal length , with , such as a hidden(x , ...,x ) (z , ..., z ) x , z ∈ Rd tional layers commonly used for mapping one variable-length sequence of symbol representations In this section we compare various aspects of self-attention layers to the recurrent and convolu4 Why Self-Attention during training. because it may allow the model to extrapolate to sequence lengths longer than the ones encountered versions produced nearly identical results (see Table 3 row (E)).We chose the sinusoidal version We also experimented with using learned positional embeddings [9] instead, and found that the two pos.PE pos+krelative positions, since for any fixed offset , can be represented as a linear function ofk PE chose this function because we hypothesized it would allow the model to easily learn to attend by corresponds to a sinusoid. The wavelengths form a geometric progression from to . We2pi 10000 2pi· where is the position and is the dimension. That is, each dimension of the positional encodingpos i (pos,2i+1)PE = cos(pos/10000 )model2i/d (pos,2i)PE = sin(pos/10000 )model2i/d |
| --- |

| In this work, we use sine and cosine functions of different frequencies: learned and fixed [9]. as the embeddings, so that the two can be summed. There are many choices of positional encodings, modelbottoms of the encoder and decoder stacks. The positional encodings have the same dimension d tokens in the sequence. To this end, we add "positional encodings" to the input embeddings at the order of the sequence, we must inject some information about the relative or absolute position of the Since our model contains no recurrence and no convolution, in order for the model to make use of the 3.5 Positional Encoding |
| --- |

| Self-Attention (restricted) O(r n d) O(1) O(n/r)· · kConvolutional O(k n d ) O(1) O(log (n))· · 2 Recurrent O(n d ) O(n) O(n)· 2 Self-Attention O(n d) O(1) O(1)·2 Operations Layer Type Complexity per Layer Sequential Maximum Path Length size of convolutions and the size of the neighborhood in restricted self-attention.r for different layer types. is the sequence length, is the representation dimension, is the kerneln d k Table 1: Maximum path lengths, per-layer complexity andminimum number of sequential operations |
| --- |

7

| We employ three types of regularization during training: 5.4 Regularization _ .warmup steps = 4000 and decreasing it thereafter proportionally to the inverse square root of the step number. We used This corresponds to increasing the learning rate linearly for the first _ training steps,warmup steps model _ _ _ (3)lrate = d min(step num , step num warmup steps )· ·0.5 1.5- -0.5- |
| --- |

| rate over the course of training, according to the formula: 1 2We used the Adam optimizer [20] with , and . We varied the learningbeta = 0.9 beta = 0.98 ϵ = 10 95.3 Optimizer (3.5 days). bottom line of table 3), step time was 1.0 seconds. The big models were trained for 300,000 steps trained the base models for a total of 100,000 steps or 12 hours. For our big models,(described on the the hyperparameters described throughout the paper, each training step took about 0.4 seconds. We We trained our models on one machine with 8 NVIDIA P100 GPUs. For our base models using 5.2 Hardware and Schedule target tokens. batch contained a set of sentence pairs containing approximately 25000 source tokens and 25000 vocabulary [38]. Sentence pairs were batched together by approximate sequence length. Each training 2014 English-French dataset consisting of 36M sentences and split tokens into a 32000 word-piece target vocabulary of about 37000 tokens. For English-French, we used the significantly largerWMT sentence pairs. Sentences were encoded using byte-pair encoding [3], which has a shared sourceWe trained on the standard WMT 2014 English-German dataset consisting of about 4.5 million 5.1 Training Data and Batching This section describes the training regime for our models. 5 Training |
| --- |

| and semantic structure of the sentences. heads clearly learn to perform different tasks, many appear to exhibit behavior related to the syntactic from our models and present and discuss examples in the appendix. Not only do individual attention As side benefit, self-attention could yieldmore interpretablemodels. We inspect attention distributions the approach we take in our model. convolution is equal to the combination of a self-attention layer and a point-wise feed-forward layer, considerably, to . Even with , however, the complexity of a separableO(k n d + n d ) k = n· · · 2 recurrent layers, by a factor of . Separable convolutions [6], however, decrease the complexityk between any two positions in the network. Convolutional layers are generally more expensive than kor in the case of dilated convolutions [18], increasing the length of the longest pathsO(log (n)) positions. Doing so requires a stack of convolutional layers in the case of contiguous kernels,O(n/k) A single convolutional layer with kernel width does not connect all pairs of input and outputk < n path length to . We plan to investigate this approach further in future work.O(n/r) the input sequence centered around the respective output position. This would increase the maximum very long sequences, self-attention could be restricted to considering only a neighborhood of size inr [38] and byte-pair [31] representations. To improve computational performance for tasks involving sentence representations used by state-of-the-art models in machine translations, such as word-piece length is smaller than the representation dimensionality , which is most often the case withn d |
| --- |

8

| We used values of 2.8, 3.7, 6.0 and 9.5 TFLOPS for K80, K40,M40 and P100, respectively.5 in different ways, measuring the change in performance on English-to-German translation on the To evaluate the importance of different components of the Transformer, we varied our base model 6.2 Model Variations single-precision floating-point capacity of each GPU .5 model by multiplying the training time, the number of GPUs used, and an estimate of the sustained architectures from the literature. We estimate the number of floating point operations used to train a Table 2 summarizes our results and compares our translation quality and training costs to other model inference to input length + , but terminate early when possible [38].50 were chosen after experimentation on the development set. We set the maximum output length during used beam search with a beam size of and length penalty [38]. These hyperparameters4 alpha = 0.6 were written at 10-minute intervals. For the big models, we averaged the last 20 checkpoints. We For the base models, we used a single model obtained by averaging the last 5 checkpoints, which dropdropout rate , instead of .P = 0.1 0.3 previous state-of-the-art model. The Transformer (big) model trained for English-to-French used outperforming all of the previously published single models, at less than the training cost of the1/4 On theWMT 2014 English-to-French translation task, our big model achieves a BLEU score of ,41.0 the competitive models. surpasses all previously published models and ensembles, at a fraction of the training cost of any of listed in the bottom line of Table 3. Training took days on P100 GPUs. Even our base model3.5 8 BLEU, establishing a new state-of-the-art BLEU score of . The configuration of this model is28.4 in Table 2) outperforms the best previously reported models (including ensembles) by more than 2.0 On theWMT 2014 English-to-German translation task, the big transformer model (Transformer (big) 6.1 Machine Translation 6 Results |
| --- |

| hurts perplexity, as the model learns to be more unsure, but improves accuracy and BLEU score. lsLabel Smoothing During training, we employed label smoothing of value [36]. Thisϵ = 0.1 drop .P = 0.1 positional encodings in both the encoder and decoder stacks. For the base model, we use a rate of sub-layer input and normalized. In addition, we apply dropout to the sums of the embeddings and the Residual Dropout We apply dropout [33] to the output of each sub-layer, before it is added to the |
| --- |

| 28.4 41.8Transformer (big) 2.3 10· 19 Transformer (base model) 27.3 38.1 3.3 10· 18 41.29ConvS2S Ensemble [9] 26.36 7.7 10 1.2 10· ·19 21 GNMT + RL Ensemble [38] 26.30 41.16 1.8 10 1.1 10· ·20 21 Deep-Att + PosUnk Ensemble [39] 40.4 8.0 10· 20 MoE [32] 26.03 40.56 2.0 10 1.2 10· ·19 20 ConvS2S [9] 25.16 40.46 9.6 10 1.5 10· ·18 20 GNMT + RL [38] 24.6 39.92 2.3 10 1.4 10· ·19 20 Deep-Att + PosUnk [39] 39.2 1.0 10· 20 ByteNet [18] 23.75 EN-DE EN-FR EN-DE EN-FR Model BLEU Training Cost (FLOPs) English-to-German and English-to-French newstest2014 tests at a fraction of the training cost. Table 2: The Transformer achieves better BLEU scores than previous state-of-the-art models on the |
| --- |

9

| remained unchanged from the English-to-German base translation model. During inference, we (section 5.4), learning rates and beam size on the Section 22 development set, all other parameters We performed only a small number of experiments to select the dropout, both attention and residual for the semi-supervised setting. [37]. We used a vocabulary of 16K tokens for theWSJ only setting and a vocabulary of 32K tokens using the larger high-confidence and BerkleyParser corpora from with approximately 17M sentences Penn Treebank [25], about 40K training sentences. We also trained it in a semi-supervised setting, modelWe trained a 4-layer transformer with on theWall Street Journal (WSJ) portion of thed = 1024 models have not been able to attain state-of-the-art results in small-data regimes [37]. constraints and is significantly longer than the input. Furthermore, RNN sequence-to-sequence constituency parsing. This task presents specific challenges: the output is subject to strong structural To evaluate if the Transformer can generalize to other tasks we performed experiments on English 6.3 English Constituency Parsing results to the base model. sinusoidal positional encoding with learned positional embeddings [9], and observe nearly identical biggermodels are better, and dropout is very helpful in avoiding over-fitting. In row (E)we replace our function than dot product may be beneficial. We further observe in rows (C) and (D) that, as expected, suggests that determining compatibility is not easy and that a more sophisticated compatibility kIn Table 3 rows (B), we observe that reducing the attention key size hurts model quality. Thisd attention is 0.9 BLEU worse than the best setting, quality also drops off with too many heads. keeping the amount of computation constant, as described in Section 3.2.2. While single-head In Table 3 rows (A),we vary the number of attention heads and the attention key and value dimensions, checkpoint averaging. We present these results in Table 3. development set, newstest2013. We used beam search as described in the previous section, but no |
| --- |

| 4.33 26.4big 6 1024 4096 16 0.3 300K 213 (E) positional embedding instead of sinusoids 4.92 25.7 0.2 5.47 25.7 0.0 4.67 25.3 (D) 0.2 4.95 25.5 0.0 5.77 24.6 4096 4.75 26.2 90 1024 5.12 25.4 53 1024 128 128 4.66 26.0 168 (C) 256 32 32 5.75 24.5 28 8 4.88 25.5 80 4 5.19 25.3 50 2 6.11 23.7 36 32 5.01 25.4 60 (B) 16 5.16 25.1 58 32 16 16 5.01 25.4 16 32 32 4.91 25.8 (A) 4 128 128 5.00 25.5 1 512 512 5.29 24.9 base 6 512 2048 8 64 64 0.1 0.1 100K 4.92 25.8 65 steps (dev) (dev) 10x 6model ff k v drop lsN d d h d d P ϵ train PPL BLEU params per-word perplexities. perplexities are per-wordpiece, according to our byte-pair encoding, and should not be compared to model. All metrics are on the English-to-German translation development set, newstest2013. Listed Table 3: Variations on the Transformer architecture. Unlisted values are identical to those of the base |
| --- |

10

| reading. , 2016.arXiv preprint arXiv:1601.06733 [4] Jianpeng Cheng, Li Dong, andMirella Lapata. Long short-term memory-networks for machine machine translation architectures. , abs/1703.03906, 2017.CoRR [3] Denny Britz, Anna Goldie,Minh-Thang Luong, and Quoc V. Le. Massive exploration of neural learning to align and translate. , abs/1409.0473, 2014.CoRR [2] Dzmitry Bahdanau, Kyunghyun Cho, and Yoshua Bengio. Neural machine translation by jointly , 2016.arXiv:1607.06450 [1] Jimmy Lei Ba, Jamie Ryan Kiros, and Geoffrey E Hinton. Layer normalization. arXiv preprint References comments, corrections and inspiration. Acknowledgements We are grateful to Nal Kalchbrenner and Stephan Gouws for their fruitful tensorflow/tensor2tensor. https://github.com/The code we used to train and evaluate our models is available at such as images, audio and video. Making generation less sequential is another research goals of ours. to investigate local, restricted attention mechanisms to efficiently handle large inputs and outputs plan to extend the Transformer to problems involving input and output modalities other than text and We are excited about the future of attention-based models and plan to apply them to other tasks. We model outperforms even all previously reported ensembles. English-to-French translation tasks, we achieve a new state of the art. In the former task our best on recurrent or convolutional layers. On both WMT 2014 English-to-German and WMT 2014 For translation tasks, the Transformer can be trained significantly faster than architectures based multi-headed self-attention. attention, replacing the recurrent layers most commonly used in encoder-decoder architectures with In this work, we presented the Transformer, the first sequence transduction model based entirely on 7 Conclusion Parser [29] even when training only on theWSJ training set of 40K sentences. In contrast to RNN sequence-to-sequence models [37], the Transformer outperforms the BerkeleyRecurrent Neural Network Grammar [8]. prisingly well, yielding better results than all previously reported models with the exception of the Our results in Table 4 show that despite the lack of task-specific tuning our model performs surfor bothWSJ only and the semi-supervised setting. increased the maximum output length to input length + . We used a beam size of and300 21 alpha = 0.3 |
| --- |

| Dyer et al. (2016) [8] generative 93.3 Luong et al. (2015) [23] multi-task 93.0 Transformer (4 layers) semi-supervised 92.7 Vinyals & Kaiser el al. (2014) [37] semi-supervised 92.1 McClosky et al. (2006) [26] semi-supervised 92.1 Huang & Harper (2009) [14] semi-supervised 91.3 Zhu et al. (2013) [40] semi-supervised 91.3 Transformer (4 layers) WSJ only, discriminative 91.3 Dyer et al. (2016) [8] WSJ only, discriminative 91.7 Zhu et al. (2013) [40] WSJ only, discriminative 90.4 Petrov et al. (2006) [29] WSJ only, discriminative 90.4 Vinyals & Kaiser el al. (2014) [37] WSJ only, discriminative 88.3 Parser Training WSJ 23 F1 ofWSJ) Table 4: The Transformer generalizes well to English constituency parsing (Results are on Section 23 |
| --- |

11

| based neural machine translation. , 2015.arXiv preprint arXiv:1508.04025 [24] Minh-Thang Luong, Hieu Pham, and Christopher DManning. Effective approaches to attentionsequence to sequence learning. , 2015.arXiv preprint arXiv:1511.06114 [23] Minh-Thang Luong, Quoc V. Le, Ilya Sutskever, Oriol Vinyals, and Lukasz Kaiser. Multi-task , 2017.arXiv:1703.03130 Zhou, and Yoshua Bengio. A structured self-attentive sentence embedding. arXiv preprint [22] Zhouhan Lin, Minwei Feng, Cicero Nogueira dos Santos, Mo Yu, Bing Xiang, Bowen , 2017.arXiv:1703.10722 [21] Oleksii Kuchaiev and Boris Ginsburg. Factorization tricks for LSTM networks. arXiv preprint Diederik Kingma and Jimmy Ba. Adam: A method for stochastic optimization. In , 2015.[20] ICLR In , 2017.International Conference on Learning Representations [19] Yoon Kim, Carl Denton, Luong Hoang, and AlexanderM. Rush. Structured attention networks. 2017. rayKavukcuoglu. Neuralmachine translation in linear time. ,arXiv preprint arXiv:1610.10099v2 [18] Nal Kalchbrenner, Lasse Espeholt, Karen Simonyan, Aaron van den Oord, Alex Graves, and Ko- , 2016.on Learning Representations (ICLR) [17] Łukasz Kaiser and Ilya Sutskever. Neural GPUs learn algorithms. In International Conference , 2016.Information Processing Systems, (NIPS) [16] Łukasz Kaiser and Samy Bengio. Can active memory replace attention? In Advances in Neural the limits of language modeling. , 2016.arXiv preprint arXiv:1602.02410 [15] Rafal Jozefowicz, Oriol Vinyals,Mike Schuster, Noam Shazeer, and YonghuiWu. Exploring , pages 832-841. ACL, August 2009.Language Processing across languages. In Proceedings of the 2009 Conference on Empirical Methods in Natural [14] Zhongqiang Huang andMary Harper. Self-training PCFG grammars with latent annotations 9(8):1735-1780, 1997. [13] Sepp Hochreiter and Jürgen Schmidhuber. Long short-term memory. ,Neural computation recurrent nets: the difficulty of learning long-term dependencies, 2001. [12] Sepp Hochreiter, Yoshua Bengio, Paolo Frasconi, and Jürgen Schmidhuber. Gradient flow in , pages 770-778, 2016.Recognition age recognition. In Proceedings of the IEEE Conference on Computer Vision and Pattern [11] Kaiming He, Xiangyu Zhang, Shaoqing Ren, and Jian Sun. Deep residual learning for im- , 2013.arXiv:1308.0850 [10] Alex Graves. Generating sequences with recurrent neural networks. arXiv preprint tional sequence to sequence learning. , 2017.arXiv preprint arXiv:1705.03122v2 [9] Jonas Gehring,Michael Auli, David Grangier, Denis Yarats, and Yann N. Dauphin. Convolunetwork grammars. In , 2016.Proc. of NAACL [8] Chris Dyer, Adhiguna Kuncoro, Miguel Ballesteros, and Noah A. Smith. Recurrent neural of gated recurrent neural networks on sequence modeling. , abs/1412.3555, 2014.CoRR [7] Junyoung Chung, Çaglar Gülçehre, Kyunghyun Cho, and Yoshua Bengio. Empirical evaluation , 2016.preprint arXiv:1610.02357 [6] Francois Chollet. Xception: Deep learning with depthwise separable convolutions. arXiv machine translation. , abs/1406.1078, 2014.CoRR and Yoshua Bengio. Learning phrase representations using rnn encoder-decoder for statistical [5] Kyunghyun Cho, Bart vanMerrienboer, Caglar Gulcehre, Fethi Bougares, Holger Schwenk, |
| --- |

12

| , pages 434-443. ACL, August 2013.1: Long Papers) shift-reduce constituent parsing. In Proceedings of the 51st AnnualMeeting of the ACL (Volume [40] Muhua Zhu, Yue Zhang, Wenliang Chen, Min Zhang, and Jingbo Zhu. Fast and accurate fast-forward connections for neural machine translation. , abs/1606.04199, 2016.CoRR [39] Jie Zhou, Ying Cao, Xuguang Wang, Peng Li, and Wei Xu. Deep recurrent models with , 2016.arXiv:1609.08144 translation system: Bridging the gap between human and machine translation. arXiv preprint Macherey,Maxim Krikun, Yuan Cao, Qin Gao, KlausMacherey, et al. Google's neural machine [38] Yonghui Wu, Mike Schuster, Zhifeng Chen, Quoc V Le, Mohammad Norouzi, Wolfgang , 2015.Advances in Neural Information Processing Systems [37] Vinyals & Kaiser, Koo, Petrov, Sutskever, and Hinton. Grammar as a foreign language. In Rethinking the inception architecture for computer vision. , abs/1512.00567, 2015.CoRR [36] Christian Szegedy, Vincent Vanhoucke, Sergey Ioffe, Jonathon Shlens, and ZbigniewWojna. networks. In , pages 3104-3112, 2014.Advances in Neural Information Processing Systems [35] Ilya Sutskever, Oriol Vinyals, and Quoc VV Le. Sequence to sequence learning with neural Inc., 2015. , pages 2440-2448. Curran Associates,Advances in Neural Information Processing Systems 28 networks. In C. Cortes, N. D. Lawrence, D. D. Lee, M. Sugiyama, and R. Garnett, editors, [34] Sainbayar Sukhbaatar, Arthur Szlam, Jason Weston, and Rob Fergus. End-to-end memory , 15(1):1929-1958, 2014.Learning Research nov. Dropout: a simple way to prevent neural networks from overfitting. Journal ofMachine [33] Nitish Srivastava, Geoffrey E Hinton, Alex Krizhevsky, Ilya Sutskever, and Ruslan Salakhutdilayer. , 2017.arXiv preprint arXiv:1701.06538 and Jeff Dean. Outrageously large neural networks: The sparsely-gated mixture-of-experts [32] Noam Shazeer, AzaliaMirhoseini, KrzysztofMaziarz, Andy Davis, Quoc Le, Geoffrey Hinton, with subword units. , 2015.arXiv preprint arXiv:1508.07909 [31] Rico Sennrich, Barry Haddow, and Alexandra Birch. Neural machine translation of rare words , 2016.preprint arXiv:1608.05859 [30] Ofir Press and LiorWolf. Using the output embedding to improve language models. arXiv 2006. , pages 433-440. ACL, JulyComputational Linguistics and 44th Annual Meeting of the ACL and interpretable tree annotation. In Proceedings of the 21st International Conference on [29] Slav Petrov, Leon Barrett, Romain Thibaux, and Dan Klein. Learning accurate, compact, summarization. , 2017.arXiv preprint arXiv:1705.04304 [28] Romain Paulus, Caiming Xiong, and Richard Socher. A deep reinforced model for abstractive model. In , 2016.Empirical Methods in Natural Language Processing [27] Ankur Parikh, Oscar Täckström, Dipanjan Das, and Jakob Uszkoreit. A decomposable attention pages 152-159. ACL, June 2006. ,Proceedings of the Human Language Technology Conference of the NAACL, Main Conference [26] DavidMcClosky, Eugene Charniak, andMark Johnson. Effective self-training for parsing. In corpus of english: The penn treebank. , 19(2):313-330, 1993.Computational linguistics [25] Mitchell PMarcus,MaryAnnMarcinkiewicz, and Beatrice Santorini. Building a large annotated |
| --- |

13

| the word 'making'. Different colors represent different heads. Best viewed in color. the verb 'making', completing the phrase 'making...more difficult'. Attentions here shown only for encoder self-attention in layer 5 of 6. Many of the attention heads attend to a distant dependency of Figure 3: An example of the attention mechanism following long-distance dependencies in the |
| --- |

| go rv eA e grm i pm n <sp m dr < < < < <<tme Ea va o iras fm pp p p p p2 ah or sjs c f Oleo ki n ia i a a a a a a0t s tc a n tp et coth n ii ir eh Sw d d d d d dn oe 0a nhv c ui i s roa or tti wi i > > > > > >>nd 9 gn e gn e ea eI sy si s ls s rt tt f .t |
| --- |

| I i s a o A h n l 2 t r m d . < < <i t t m g p s m o v p < < < <s ant h h h ep i of a e 0 io a r r pp pE p p pm nw fa a oi a e g o fs i tv w 0v s a a aa a aOr c iij k rt i ce s cni o e e s s9 d d dd d deet i e Sur ne gtr r > > >> > >i ri n s lc gd >t a ty sma tin oe |
| --- |

nnts

| Attention VisualizationsInput-Input Layer5 |
| --- |

14

| and 6. Note that the attentions are very sharp for this word. Full attentions for head 5. Bottom: Isolated attentions from just the word 'its' for attention heads 5 Figure 4: Two attention heads, also in layer 5 of 6, apparently involved in anaphora resolution. Top: |
| --- |

a p

| p m <olp si <Ec pin e h s pa iw Oe r no sLT atf j t ahbvw i iu iue ma wh S do onb bh ie rau l isc tw iii >>n nd gee e e ne s yssl r - tt tt , , .l |
| --- |

| T L b , i a b - t w a m o . <w n p b s j i w , i m <t u s nhs ha e p e r pe e u E ph hi es i yi el ssow v r t p i ale Oat nf se ul de ti i Sic or l >nc da n >t gt |
| --- |

io n a p

| p m <olp si <Ec pInput-Input Layer5 in e h s pa iw Oe r no sLT atf j t ahbvw i iu iue ma wh S do onb bh ie rau l isc tw iii >>n nd gee e e ne ss ysl r - tt tt , , .l |
| --- |

| T L b , i a b - t w a m o . <w n p b s j i w , i m <t u s nhs ha e e e u p e r p E ph i h es i yi el so sw p iv r t ale Oat nf se ul de ti i Sic or l >nc da n >t gt |
| --- |

io n

| Input-Input Layer5 |
| --- |

15

| at layer 5 of 6. The heads clearly learned to perform different tasks. sentence. We give two such examples above, from two different heads from the encoder self-attention Figure 5: Many of the attention heads exhibit behaviour that seems related to the structure of the |
| --- |

a p

| p m <olp si <Ec pin e h s pa iw Oe r no sLT atf j t ahbvw i iu iue ma wh S do onb bh ie rau l isc tw iii >>n nd gee e e ne ss ysl r - tttt , , .l |
| --- |

| T m .L w n b p , b i a s b j - t i w w a , m o <i <t u s nhs ha e p e r pe e u E ph hi es i yi el ssow p iv r t ale Oat nf se ul de ti i Sic or l >nc da n >t gt |
| --- |

io n a p

| p m <olp si <Input-Input Layer5 Ec pin e h s pa iw Oe r no sLT atf j t ahbvw i iu iue ma wh S do onb bh ie rau l s ic tw iii >>n nd ge ee e ne ss ysl r - tttt , , .l |
| --- |

| T m .L w n b p , b i a s b j - t i w w a , m o <i <t u s nhs ha e p e r pe e u E ph hi es i yi el ssow p iv r t ale Oat nf se ul de ti i Sic or l >nc da n >t gt |
| --- |

io n

| Input-Input Layer5 |
| --- |
