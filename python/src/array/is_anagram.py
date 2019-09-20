def is_anagram(str1, str2):

    str1 = str1.replace(' ', '').lower()
    str2 = str2.replace(' ', '').lower()
    return sorted(str1) == sorted(str2)


def is_anagram2(str1, str2):

    str1 = str1.replace(' ', '').lower()
    str2 = str2.replace(' ', '').lower()

    # Edge case check
    if len(str1) != len(str2):
        return False

    count = {}

    for c in str1:
        count[c] = count[c] + 1 if c in count else 1

    for c in str1:
        count[c] = count[c] - 1 if c in count else 1

    for k in count:
        if count[k] != 0:
            return False
    return True
