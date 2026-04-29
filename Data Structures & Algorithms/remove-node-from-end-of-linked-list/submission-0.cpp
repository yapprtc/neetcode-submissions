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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode dummy;
        dummy.next = head;
        ListNode* prev = &dummy;
        ListNode* slow = head;
        ListNode* fast = head;
        while (n > 0 && fast != nullptr) {
            fast = fast->next;
            n -= 1;
        }

        if (n == 0) {
            while (fast != nullptr) {
                prev = slow;
                slow = slow->next;
                fast = fast->next;
            }
            prev->next = slow->next;
        }
        return dummy.next;
    }
};
