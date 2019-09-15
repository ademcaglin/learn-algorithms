import crypto from "crypto";

export function remove(array, element) {
    const index = array.indexOf(element);

    if (index !== -1) {
        array.splice(index, 1);
    }
}

export function find_diff(first, last) {
    let firstArr = [...first];
    let lastArr = [...last];
    for (let f of first) {
        if (lastArr.includes(f)) {
            remove(firstArr, f);
            remove(lastArr, f);
        }
    }
    return firstArr.join("");
}

export function find_anagrams(phrase, candidate, words, secrets, result) {
    let filtered = words.filter(w => !find_diff(w, phrase));
    for (let w of filtered) {
        let new_candidate = [...candidate, w];
        let new_words = [...words];
        remove(new_words, w);
        let new_phrase = find_diff(phrase, w);
        if (new_phrase === "") {
            let new_candidate_str = new_candidate.join(" ");
            let digest = crypto.createHash('md5').update(new_candidate_str).digest("hex");
            let r = secrets.find(x => x === digest);
            if (r) {
                result.push({ phrase: new_candidate_str, secret: digest });
            }
        }
        else {
            find_anagrams(new_phrase, new_candidate, new_words, secrets, result);
        }
    }
}

/*import fs from 'fs';
import { find_diff } from './find_diff';
import find_anagrams from './find_anagrams';
const phrase = "poultryoutwitsants";
const wordlist = [];

var lines = fs.readFileSync('words').toString().split("\n");
for(let line of lines){
    let word = line.replace("\r", "");
    if(word.length > 4 && !wordlist.includes(word) && !find_diff(word, phrase)){
        wordlist.push(word);
    }
}
console.time("dbsave");
let secrets = ["e4820b45d2277f3844eac66c903e84be", "23170acc097c24edb98fc5488ab033fe", ""]
let result = [];
find_anagrams(phrase, [], wordlist, secrets, result);
console.timeEnd("dbsave");*/