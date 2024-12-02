import time
from fasttok import split_string
from sklearn.feature_extraction.text import CountVectorizer

text = ["The large quad-legged vehicle fires lasers in a splash pattern well-suited to destroying swarms of weaker units. This unit can also traverse differences in terrain height due to its long legs, and will appear to step over ledges and other obstacles due to the inverse kinematics system."] * 100000

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