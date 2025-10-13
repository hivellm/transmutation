Table 3: Variations on the Transformer architecture. Unlisted values are identical to those of the base model. All metrics are on the English-t o-German translation development set, newstest2013. Listed perplexities are per-wordpiece, according to our byte-pair encoding, and should not be compared to per-word perplexities. N d model df f hd k dv P drop  ls train PPL BLEU params steps (dev) (dev)  10 6 base


## 6 512 2048 8 64 64 0.1 0.1 100K


## 4.92 25.8 65

(A)


## 1 512 512


## 5.29 24.9


## 4 128 128


## 5.00 25.5


## 16 32 32


## 4.91 25.8


## 32 16 16


## 5.01 25.4

(B) 16


## 5.16 25.1 58

32


## 5.01 25.4 60

(C) 2


## 6.11 23.7 36

4


## 5.19 25.3 50

8


## 4.88 25.5 80


## 256 32 32


## 5.75 24.5 28


## 1024 128 128


## 4.66 26.0 168

1024


## 5.12 25.4 53

4096


## 4.75 26.2 90

(D) 0.0


## 5.77 24.6

0.2


## 4.95 25.5

0.0


## 4.67 25.3

0.2


## 5.47 25.7

(E) positional embedding instead of sinusoids


## 4.92 25.7

big


## 6 1024 4096 16 0.3 300K


## 4.33 26.4

213 development set, newstest2013. We used beam search as described in the previous section, but no checkpoint averaging. We present these results in Table 3. In Table 3 rows (A), we vary the number of attention heads and the attention key and value dimensions, keeping the amount of computation constant, as described in Section 3.2.2. While single-head attention is 0.9 BLEU worse than the best setting, quality also drops off with too many heads. In Table 3 rows (B), we observe that reducing the attention key size dk hurts model quality. This suggests that determining compatibility is not easy and that a more sophisticated compatibility function than dot product may be benecial. We further observe in rows (C) and (D) that, as expected, bigger models are better, and dropout is very helpful in avoiding over-tting. In row (E) we replace our sinusoidal positional encoding with learned positional embeddings [ 9 ], and observe nearly identical results to the base model.


## 6.3 English Constituency Parsing

To evaluate if the Transformer can generalize to other tasks we performed experiments on English constituency parsing. This task presents specic challenges: the output is subject to strong structural constraints and is signicantly longer than the input. Furthermore, RNN sequence-t o-sequence models have not been able to attain state-o f-the-art results in small-data regimes [37]. We trained a 4-layer transformer with d model = 1024 on the Wall Street Journal (WSJ) portion of the Penn Treebank [ 25 ], about 40K training sentences. We also trained it i na semi-supervised setting, using the larger high-condence and BerkleyParser corpora from with approximately 17M sentences [ 37 ]. We used a vocabulary of 16K tokens for the WSJ only setting and a vocabulary of 32K tokens for the semi-supervised setting. We performed only a small number of experiments to select the dropout, both attention and residual (section 5.4), learning rates and beam size on the Section 22 development set, all other parameters remained unchanged from the English-t o-German base translation model. During inference, we 9