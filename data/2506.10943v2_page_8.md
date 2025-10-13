## Method Single Passage

(n = 1; LoRA) Continued Pretraining (n = 200; full-FT) Continued Pretraining (n = 2067; full-FT) Base model 32.7 32.7 29.0 Train on Passage 33.5 36.0 31.2 Train on Passage + Synthetic 39.7 50.6 43.4 Train on Passage + GPT-4.1 Synthetic 46.3

## 59.4 49.2

SEAL 47.0

## 58.2 46.4

Table 2: Knowledge Incorporation Performance Across Passage Settings. Figure 5: Example Knowledge Incorporation Self-Edits Across RL Iterations. In this example, we see how RL leads to the generation of more detailed self-edits, which in turn results in better performance. While the progression is clear in this case, the differences across iterations are sometimes more subtle in other examples. We show in �B.11 that prompting for longer self-edits is effective, and that RL training further improves performance by a similar margin.

## 5 Limitations

Figure 6: Catastrophic forgetting from continual self-edits. We sequentially update the model on new passages and track degradation on prior tasks. Entrywise standard errors are reported in �B.6. Catastrophic forgetting. One key motivation we had for enabling language models to self-edit is to move towards the ultimate goal of continual learning�allowing models to incorporate new information over time, whether through agentically interacting with an environment or through standard training. While our earlier experiments assess how well SEAL adapts to individual edits in isolation, a more ambitious goal is to support sequences of edits: can the model adapt to new information repeatedly while preserving prior knowledge? This question relates directly to the challenge of catastrophic forgetting [ 73 , 74 ], where new updates interfere destructively with past learning. We do not explicitly optimize for retention in our current training setup, but we aim to establish a baseline for how well SEAL handles sequential self-edits without dedicated mechanisms for handling catastrophic forgetting. To test this, we simulate a continual learning setting in the knowledge incorporation domain. The model receives a stream of test passages, each triggering a new self-edit. After each update, we 8