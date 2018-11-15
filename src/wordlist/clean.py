from subprocess import run
files = {'final':'final.txt', 'white':'whitelist.txt', 'black':'blacklist.txt',
         'raw':'Moby/FILES/SINGLE.TXT', 'scrabble':'Moby/FILES/CROSSWD.TXT',
         'scrabble2':'Moby/FILES/CRSWD-D.TXT'}
alpha_set = set('abcdefghijklmnopqrstuvwxyz')

# TODO: test Set.issubset() vs regex
def is_valid(word):
    """
    Return True if word is 5 letters long, no letters repeat, and
    all letters are lowercase a-z. Ignore trailing newlines.
    """
    word = word.rstrip('\n')
    if len(word) != 5:
        return False
    elif len(set(word)) != 5:
        return False
    elif not set(word).issubset(alpha_set):
        return False
    else:
        return True

def filter_raw(file_location):
    """
    Return a list of the valid words from the raw file.
    """
    with open(file_location) as f:
        lines = f.readlines()
    words = [word.rstrip('\n') for word in lines if is_valid(word)]
    return words

def remove_nonsense(current_list):
    """
    Remove words without any definitions in WordNet. Requires WordNet.
    Still leaves a lot of nonsense.
    """
    for word in current_list:
        result = run(['wn', word, '-over'], capture_output=True)
        # returncode is number of WordNet senses
        if result.returncode == 0:
            current_list.remove(word)

def add_white(current_list):
    with open(files['white']) as f:
        lines = f.readlines()
    words = [word.rstrip('\n') for word in lines if word != '\n']
    current_list += words
    current_list.sort()

def remove_black(current_list):
    with open(files['black']) as f:
        lines = f.readlines()
    words = [word.rstrip('\n') for word in lines if word != '\n']
    for word in words:
        current_list.remove(word)
    
def validate_whiteblack():
    """
    Check consistency of white and black lists.

    Is a separate function so it can be done beforehand, rather than cause the
    program to fail after the raw list has been processed.
    """
    with open(files['white']) as listfile:
        for line in listfile.readlines():
            if not is_valid(line) and line != '\n':
                print("Whitelist contains non-valid word '{}'.".format(
                    line.rstrip('\n')))
                raise Exception
    with open(files['black']) as listfile:
        for line in listfile.readlines():
            if not is_valid(line) and line != '\n':
                print("Blacklist contains non-valid word '{}'.".format(
                    line.rstrip('\n')))
                raise Exception
    
def is_valid_test():
    test_list = {"board":True, "bola":False, "backing":False,
                 "butter":False, "Paste":False, "bo ta":False,
                 "bl0ck":False}
    failures = 0
    for word, expected in test_list.items():
        result = is_valid(word)
        print("{:.<8}: expected {!s:<5}, tested as {}".format(
            word, expected, result))
        if result != expected:
            failures += 1
    print("{} failures".format(failures))
        
if __name__ == "__main__":
    validate_whiteblack()
    wordlist = filter_raw(files['scrabble'])
    #wordlist += filter_raw(files['scrabble2'])
    #remove_nonsense(wordlist)
    remove_black(wordlist)
    add_white(wordlist)

    with open(files['final'], mode='w') as f:
        for word in wordlist:
            f.write(word + '\n')
    
