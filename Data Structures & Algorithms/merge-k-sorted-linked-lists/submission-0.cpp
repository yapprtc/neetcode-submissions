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
        auto cmp = [](const pair<int, ListNode*>&a , const pair<int, ListNode*>&b) {
            return a.first > b.first;
        };
        priority_queue<pair<int, ListNode*>, std::vector<pair<int, ListNode*>>, decltype(cmp)> heap(cmp);
        for (auto& p : lists) {
            if (p != nullptr) {
                heap.push({p->val, p});
            }
        }
        ListNode dummy;
        ListNode* p = &dummy;
        while (!heap.empty()) {
            auto q = heap.top(); heap.pop();
            p->next = q.second;
            p = p->next;
            if (p->next != nullptr) {
                heap.push({p->next->val, p->next});
            }
        }
        return dummy.next;
    }
};
