struct Node {
    bool value;
    Node* next[26];

    Node() {
        value = false;
        for (int i = 0; i < 26; ++i) {
            next[i] = nullptr;
        }
    }

    ~Node() {
        for (int i = 0; i < 26; ++i) {
            if (next[i] != nullptr) {
                delete next[i];
            }
        }
    }
};

class WordDictionary {
public:
    WordDictionary() {
        root = nullptr;
    }

    ~WordDictionary() {
        if (root != nullptr) {
            delete root;
        }
    }

    void addWord(string word) {
        root = addWord(root, word, 0);
    }
    
    bool search(string word) {
        auto x = search(root, word, 0);
        if (x == nullptr) { return false; }
        return x->value;
    }

private:
    Node* addWord(Node* x, string& word, size_t d) {
        if (x == nullptr) { x = new Node(); }
        if (d == word.size()) { x->value = true; return x; }
        auto c = word[d] - 'a';
        x->next[c] = addWord(x->next[c], word, d+1);
        return x;
    }

    Node* search(Node* x, string& word, size_t d) {
        if (x == nullptr) { return nullptr; }
        if (d == word.size()) { return x; }
        if (word[d] == '.') {
            for (int c = 0; c < 26; ++c) {
                auto n = search(x->next[c], word, d+1);
                if (n != nullptr && n->value) {
                    return n;
                }
            }
            return nullptr;
        } else {
            auto c = word[d] - 'a';
            return search(x->next[c], word, d+1);
        }
    }

    Node* root;
};
