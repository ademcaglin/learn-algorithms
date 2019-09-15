import hashlib 

def diff(first, last):
  first_arr = list(first)
  last_arr = list(last)
  for c in first:
    if c in last_arr:
      first_arr.remove(c)
      last_arr.remove(c)
  return "".join(first_arr)

def find_anagrams(phrase, candidate, words, secrets, result):
  filtered = [w for w in words if len(diff(w, phrase)) == 0]
  for w in filtered:
    new_candidate = [*candidate, w]
    new_words = [*filtered]
    new_words.remove(w)
    new_phrase = diff(phrase, w)
    if new_phrase == "":
      new_candidate_str = " ".join(new_candidate)
      digest = hashlib.md5(new_candidate_str.encode()).hexdigest()
      r = next(s for s in secrets if s == digest)
      if r:
        result.append(new_candidate_str)
    else:
      find_anagrams(new_phrase, new_candidate , new_words, secrets, result)


# import time
# import diff
# import find_anagrams

# phrase = "poultryoutwitsants"
# fo = open("words", "r")

# wordlist = [line.rstrip('\n') for line in fo]  # get words from file
# fo.close()
# # remove words include different chars
# wordlist = [w for w in wordlist if len(diff(w, phrase)) == 0]
# wordlist = list(dict.fromkeys(wordlist))  # remove dublicates
# wordlist = [w for w in wordlist if len(w) > 3]  # take words whose length

# start_time = time.time()
# result = []
# secrets = ["e4820b45d2277f3844eac66c903e84be", "23170acc097c24edb98fc5488ab033fe", ""]
# find_anagrams(phrase, [], wordlist, secrets, result)
# elapsed_time = time.time() - start_time
# print("Time Elapsed :{0:2f}". format(elapsed_time))