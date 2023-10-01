# Concepts

- associative memory (recalling a knowledge links to other)
- correlation (an event happens in concert with others)
- induction (find a pattern from facts)
- deduction (create hypothetical fact from pattern)
- forecast (given some facts, what happens next?)
- generation (generate information from patterns)
- by analogy (apply a pattern on similar information)
- causality (classify if a pattern is correlation or causality)
- scientific method (given some facts, find patterns, ask for information needed to perfect patterns)
- classification (find out similar sets of events has same consequence)

# Tasks

- translation (english language -> agnostic language -> italian language)
- complete text
- summarize text
- query (get knowledge -> select and rearrange it based on a query)
- forecast event (given some events -> what happens next?)

# Training Data

https://dmf.unicatt.it/~della/pythoncourse18/commedia.txt
https://download.feedbooks.net/book/7384.epub?t=1548245072&filename=il-piccolo-principe

# Other

lazily sinthetize all possible predictions with no duplication

make it a lazy graph width predictions as nodes and edges that indicates fluctuations in complexity, accuracy, occurrences

sinthetize the less complex predictions with highest accuracy and occurrence

given some facts predict what happens next, with backtracking

- given current facts, predict most plausible one, add it to headings
- rank every heading with average accuracy based on all apredictions and predicted facts
- extend heading with highest
- present the sequence wiht highest accuracy as result

---

conseguenza: A causa sicuramente B, B può accadere anche in assenza di A
se A allora B per forza
se B allora A non è detto

causa: B è sicuramente causato da A, non da altro
se A allora B per forza
se B allora A per forza

generalizzazione: un insieme di eventi AN causano sempre la stessa conseguenza B, AN può essere generalizzato come AN segue B
se A1 allora B per forza
se B allora A1 non è detto
se A2 allora B per forza
se B allora A2 non è detto

---

- per costruire una nuova relazione prendere in esame il termine che più influise sul risultato
- predizione del testo per ogni carattere la probabilità che sia il successivo è quante volte la sua funzione booleana ritorna vero sul totale

---

pattern.utility = pattern.accuracy \* pattern.condition.occurrence
pattern.accuracy = (count every scenario where pattern.condition happens and pattern.conseguence are true) / pattern.condition.occurence
pattern.condition.occurence = count how many times happens
pattern.conseguence

---

there are until now created variables (start with input variables)
compute or odds table (https://en.wikipedia.org/wiki/Odds_ratio) save it ordered
combine variables (https://en.wikipedia.org/wiki/Combination) starting with highest odd ratio pairs and save in ordered structure to form new variables not/and/or
the ordered structure can be kept small by retaining only input variables and synthetized variables by utility treshold (see above)
(in the creation deduplicate synthetized variables by truth table)
character prediction is implemented by finding in the odd table the highest probability next character

----

associative memory: let be input values be neurons, output value neurons, input/output pairs, optionally layers, leran by back propagation

generative model: given some data, generate more data with same probability distributions

translate from one language to another:
train on translation pairs
learn mapping from language A to language B
or learn mappiong from language A to intermediate alnguage I, maaping from I to B

summarize: given pairs of source and summary learn mapping

question/answer: given source, question, answer, leanr mapping source + question -> answer

complete: given incomplete data, try to complete it

forecast: observe events in training data, then given some events forecast possibility of others (ex: causes -> effects or vice versa)

complete knowledge: given some concept and relations, infere more concepts and relations that hold

idea! given some training data, create patterns, check patterns probability in training data, check pattern probability in test data

idea! optimize by
- implementing high level concepts in classical programming to reduce learning costs (first per task, then generalize)
- optimize source data analysis to minimize training cost (ex: convolution and weight sharing for images, embeddings for sequences)

idea! theory -> proof
- begin with limited set of training data, make a theory, validate it by searching proofs in the rest of training data, or sythetize proofs along the way
- apply classical inferences ex: humans are mortal, socrates is human, socrates is mortal (then validate by other factors too) 
