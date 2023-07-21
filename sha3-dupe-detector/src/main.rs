/*
Generates random duplicate phrases from a list of phrases
and prints the number of unique phrases and the number of duplicate phrases.

Example output:

Total number of phrases: 24
131a931202f9f1e7821ece767d0a9041aeb0270a40def3583de149c849683cb2 - 3 times: the old man looked at him with his sun burned confident loving eyes
ad604cf092d30c844c8f1820de47771efef7a66763468cd2a68bbed8637579d2 - 3 times: his eyes were cheerful and undefeated
796599a1f14554fde9514bf41dca747548570d3a7a38a74cc19caa07ae55ca70 - 2 times: a man can be destroyed but not defeated
6931bbb80ba33090d9e7015551e7e21676a4813d50238b1e75c9a9ce38845b1e - 2 times: but man is not made for defeat
864fd2e3cac4fa99278594867c3accc16aacfb3f543e0ed53bf71cd8d23273e1 - 3 times: man can be destroyed but not defeated
0a9247da949c7ff1ff5b64bf3482a46cb90b87de9cade88ca26ff4bd101b1017 - 3 times: everything about him was old
7f7f417bb4ff8b62b19edc25c3e359c6a32f7d2f06883ce40062eae093a23ad1 - 3 times: the old man had taught the boy to fish
c33fc7b48db132dfdc5fc6aac5514ad867fc35b21c06e55725720057034c6a56 - 2 times: he was an old man who fished alone
d40a2c3a380fbb5f5b15db16a9acb87b14c91906aac595e6f920827ee6187ef6 - 2 times: the sail was patched with flour sacks
Total Unique Phrases: 10
Total Unique Duplicates: 9
Total Combined Duplicates: 14

*/
use sha3_dupe_detector::generate_random_phrases;

fn main() {
    let phrases = generate_random_phrases();
    sha3_dupe_detector::analyze_duplicates(&phrases);
}
