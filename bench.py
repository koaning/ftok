import time
from fasttok import split_string
from sklearn.feature_extraction.text import CountVectorizer

text = ["The large quad-legged vehicle. That's a normal sentence from the starcraft universe right there!"] * 100000

tic = time.time()
CountVectorizer().fit(text)
toc = time.time()
print(toc - tic)

tic = time.time()
CountVectorizer(tokenizer=lambda d: d.split(" ")).fit(text)
toc = time.time()
print(toc - tic)

tic = time.time()
CountVectorizer(tokenizer=split_string).fit(text)
toc = time.time()
print(toc - tic)
