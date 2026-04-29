/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        unordered_map<Node*, Node*> random;
        Node dummy(0);
        Node* p = &dummy;
        Node* q = head;

        while (q != nullptr) {
            Node* cloned = new Node(q->val);
            random.insert({q, cloned});
            p->next = cloned;
            q = q->next;
            p = p->next;
        }

        p = dummy.next;
        q = head;
        while (q != nullptr) {
            if (q->random != nullptr) {
                p->random = random[q->random];
            }
            q = q->next;
            p = p->next;
        }
        return dummy.next;
    }
};
