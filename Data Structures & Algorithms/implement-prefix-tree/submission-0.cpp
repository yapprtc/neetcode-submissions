struct Node {
    bool value;
    Node* next[26];

    Node() {
        value = false;
        for (int i = 0 ; i < 26; i++) {
            next[i] = nullptr;
        }
    }

    ~Node() {
        for (int i = 0 ; i < 26; i++) {
            if (next[i] != nullptr) {
                delete next[i];
            }
        }
    }
};

class PrefixTree {
public:
    PrefixTree() {
        root = nullptr;
    }

    ~PrefixTree() {
        if (root != nullptr) {
            delete root;
        }
    }
    
    void insert(string word) {
        root = insert(root, word, 0);
    }
    
    bool search(string word) {
        auto x = search(root, word, 0);
        if (x == nullptr) { return false; }
        return x->value;
    }
    
    bool startsWith(string prefix) {
        auto x = search(root, prefix, 0);
        if (x == nullptr) { return false; };
        return true;
    }

private:
    Node* insert(Node* x, string& word, size_t d) {
        if (x == nullptr) { x = new Node(); }
        if (d == word.size()) { x->value = true; return x; }
        auto c = word[d] - 'a';
        x->next[c] = insert(x->next[c], word, d+1);
        return x;
    }

    Node* search(Node* x, string& word, size_t d) {
        if (x == nullptr) { return nullptr; }
        if (d == word.size()) { return x; }
        auto c = word[d] - 'a';
        return search(x->next[c], word, d+1);
    }

    Node* root;
};
