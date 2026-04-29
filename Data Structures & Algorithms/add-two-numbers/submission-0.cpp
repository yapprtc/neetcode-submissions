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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode dummy;
        ListNode* p = &dummy;
        int carry = 0;

        while (l1 != nullptr && l2 != nullptr) {
            int sum = l1->val + l2->val + carry;
            carry = sum / 10;
            sum = sum % 10;
            p->next = new ListNode(sum);
            l1 = l1->next;
            l2 = l2->next;
            p = p->next;
        }

        ListNode* l = l1 != nullptr ? l1 : l2;
        while (l != nullptr) {
            int sum = l->val + carry;
            carry = sum / 10;
            sum = sum % 10;
            p->next = new ListNode(sum);
            l = l->next;
            p = p->next;
        }

        if (carry > 0) {
            p->next = new ListNode(carry);
        }

        return dummy.next;
    }
};
