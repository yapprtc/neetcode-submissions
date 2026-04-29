/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        auto cmp = [](const ListNode* a, const ListNode* b) {
            return a->val > b->val;
        };
        priority_queue<ListNode*, std::vector<ListNode*>, decltype(cmp)> heap(cmp);
        for (auto& p : lists) {
            if (p != nullptr) {
                heap.push(p);
            }
        }
        ListNode dummy;
        ListNode* p = &dummy;
        while (!heap.empty()) {
            auto q = heap.top(); heap.pop();
            p->next = q;
            p = p->next;
            if (p->next != nullptr) {
                heap.push(p->next);
            }
        }
        return dummy.next;
    }
};
