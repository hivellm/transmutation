Table 5: Entrywise standard errors of the mean (SEM) across continual self-edits experiment.


## 1 2 3 4 5 6 7 8

0


## 0.0306 0.0315 0.0263 0.0318 0.0297 0.0370 0.0310 0.0284

1


## 0.0273 0.0000 0.0000 0.0000 0.0000 0.0000 0.0000 0.0000

2


## 0.0305 0.0277 0.0000 0.0000 0.0000 0.0000 0.0000 0.0000

3


## 0.0277 0.0358 0.0406 0.0000 0.0000 0.0000 0.0000 0.0000

4


## 0.0272 0.0303 0.0337 0.0320 0.0000 0.0000 0.0000 0.0000

5


## 0.0296 0.0342 0.0290 0.0298 0.0319 0.0000 0.0000 0.0000

6


## 0.0289 0.0334 0.0271 0.0258 0.0320 0.0337 0.0000 0.0000

7


## 0.0255 0.0313 0.0264 0.0253 0.0309 0.0331 0.0363 0.0000

8


## 0.0237 0.0307 0.0211 0.0267 0.0273 0.0271 0.0358 0.0263

Table 6: Model Size Scaling Performance (%). Model Base Model (No Training) Base Model Self-Edit SEAL Qwen2.5-3B 25.1 31.9 37.0 Qwen2.5-7B 32.7 39.7 47.0 benet sa s model capacity increases. We acknowledge that it is hard to draw conclusions though without actually scaling up further. B.8 Comparison to Generative Adapter We additionally compared with Generative Adapter [ 54 ], a hypernetwork approach that generates LoRA weights from context, using our evaluation setup. Table 7 reports results for both singlepassage ( n =1 ) and continued pretraining ( n = 200 ). We use the Mistral-7B-based model [ 85 ] for Generative Adapter, since that was the closest model for comparison. All values are on the same evaluation set, but CPT batches updates over all documents while single-passage trains and evaluates an adapter separately for each document. Generative Adapter achieves strong performance in the n =1 case, but underperforms SEAL in the CPT setting. SEAL's parameterization of weight updates through synthetic data generation allows reuse of generated data for CPT, application to arbitrary base models, and exibility to learn updates from diverse interaction types beyond LoRA netuning. Table 7: SEAL v s. Generative Adapter Performance (%). Model Base Single-passage ( n =1 ) CPT ( n = 200 ) SEAL 32.0 47.0 58.2 Generative Adapter 24.4 66.8 28.0 We note that parameterizing weight updates via synthetic data generation rather than directly predicting LoRA weights has several advantages: (1) generated data can be reused for CPT or applied to arbitrary base models, (2) models can leverage reasoning and restructuring as document scale and complexity grow, and (3) the framework is not restricted to LoRA netuning, allowing for many different update types, including those arising from environment or user interactions. By contrast, it is unclear how hypernetwork-based approaches would scale to such settings, while next-token prediction on generated data naturally exploits a model's i n-context learning capabilities. B.9 Comparison to Entigraph We additionally compare SEAL to Entigraph [ 25 ] in the Synthetic Continued Pretraining (SCPT) setting on SQuAD. Results for both 200 and 2067 passages are shown in Table 8. SEAL uses the same 5 synthetic data generations per document. For Entigraph, we sample 5 synthetic data generations involving pairs and 5 triplets of entities per document. Entigraph with all 10 synthetic data generations sampling is competitive with SEAL, especially at the larger scale. These results 22