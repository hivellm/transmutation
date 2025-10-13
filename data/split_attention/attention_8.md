Table 2: The Transformer achieves better BLEU scores than previous state-o f-the-art models on the English-t o-German and English-t o-French newstest2014 tests at a fraction of the training cost. Model BLEU Training Cost (FLOPs)


## EN-DE EN-FR EN-DE EN-FR

ByteNet [18] 23.75 Deep-Att + PosUnk [39] 39.2 1 : 0  10 20 GNMT + RL [38] 24.6 39.92 2 : 3  10 19 1 : 4  10 20 ConvS2S [9] 25.16 40.46 9 : 6  10 18 1 : 5  10 20 MoE [32] 26.03 40.56 2 : 0  10 19 1 : 2  10 20 Deep-Att + PosUnk Ensemble [39] 40.4 8 : 0  10 20 GNMT + RL Ensemble [38] 26.30 41.16 1 : 8  10 20 1 : 1  10 21 ConvS2S Ensemble [9] 26.36 41.29 7 : 7  10 19 1 : 2  10 21 Transformer (base model) 27.3 38.1 3 : 3  10 18 Transformer (big)


## 28.4 41.8

2 : 3  10 19 Residual Dropout We apply dropout [ 33 ] to the output of each sub-layer, before it is added to the sub-layer input and normalized. In addition, we apply dropout to the sums of the embeddings and the positional encodings in both the encoder and decoder stacks. For the base model, we use a rate of P drop = 0 : 1 . Label Smoothing During training, we employed label smoothing of value  ls = 0 : 1 [ 36 ]. This hurts perplexity, as the model learns to be more unsure, but improves accuracy and BLEU score.


## 6 Results


## 6.1 Machine Translation

On the WMT 2014 English-t o-German translation task, the big transformer model (Transformer (big) in Table 2) outperforms the best previously reported models (including ensembles) by more than 2 : 0 BLEU, establishing a new state-o f-the-art BLEU score of 28 : 4 . The conguration of this model is listed in the bottom line of Table 3. Training took 3 : 5 days on 8 P100 GPUs. Even our base model surpasses all previously published models and ensembles, at a fraction of the training cost of any of the competitive models. On the WMT 2014 English-t o-French translation task, our big model achieves a BLEU score of 41 : 0 , outperforming all of the previously published single models, at less than 1 = 4 the training cost of the previous state-o f-the-art model. The Transformer (big) model trained for English-t o-French used dropout rate P drop = 0 : 1 , instead of 0 : 3 . For the base models, we used a single model obtained by averaging the last 5 checkpoints, which were written at 10-minute intervals. For the big models, we averaged the last 20 checkpoints. We used beam search with a beam size of 4 and length penalty = 0 : 6 [ 38 ]. These hyperparameters were chosen after experimentation on the development set. We set the maximum output length during inference to input length + 50 , but terminate early when possible [38]. Table 2 summarizes our results and compares our translation quality and training costs to other model architectures from the literature. We estimate the number of oating point operations used to train a model by multiplying the training time, the number of GPUs used, and an estimate of the sustained single-precision oating-point capacity of each GPU 5 .


## 6.2 Model Variations

To evaluate the importance of different components of the Transformer, we varied our base model in different ways, measuring the change in performance on English-t o-German translation on the 5 We used values of 2.8, 3.7, 6.0 and 9.5 TFLOPS for K80, K40, M40 and P100, respectively. 8