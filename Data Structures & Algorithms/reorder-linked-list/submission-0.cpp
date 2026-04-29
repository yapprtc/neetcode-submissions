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
    void reorderList(ListNode* head) {
        if (head == nullptr) {
            return;
        }

        ListNode* middle = middleList(head);
        ListNode* tail = reverseList(middle);

        ListNode* p = head;
        ListNode* r = tail;
        while (r->next != nullptr) {
            ListNode* q = p->next;
            ListNode* s = r->next;
            p->next = r;
            r->next = q;
            p = q;
            r = s;
        }
    }

    ListNode* middleList(ListNode* head) {
        ListNode* fast = head;
        ListNode* slow = head;
        while (fast != nullptr && fast->next != nullptr) {
            fast = fast->next->next;
            slow = slow->next;
        }
        return slow;
    }

    ListNode* reverseList(ListNode* middle) {
        ListNode* r = middle;
        ListNode* p = middle->next;
        while (p != nullptr) {
            ListNode* q = p->next;
            p->next = r;
            r = p;
            p = q;
        }
        middle->next = nullptr;
        return r;
    }
};
