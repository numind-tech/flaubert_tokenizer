This repo is meant to see how to create flaubert tokenizers properly using huggingface tokenizers.

It relies on the flaubert model that can be found there: https://huggingface.co/flaubert/flaubert_base_uncased


In order to create some "fresh" config files, use the following python sniplet:

```python
from transformers import AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("flaubert/flaubert_base_uncased")

tokenizer.save_pretrained("tokenizer")
```
