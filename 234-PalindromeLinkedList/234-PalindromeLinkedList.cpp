// Last updated: 6/7/2025, 2:41:57 AM
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
    bool isPalindrome(ListNode* head) {
        
        if (head == NULL) return true;

        // Find the end of first half 
        ListNode *fast = head;
        ListNode *slow = head;
        while (fast->next != NULL && (fast->next)->next != NULL) {
            fast = (fast->next)->next;
            slow = slow->next;
        }
        ListNode *firstHalfEnd = slow;
        
        // Reverse second half
        ListNode *next = NULL;
        ListNode *prev = NULL;
        ListNode *curr = firstHalfEnd->next;
        while (curr != NULL) {
            next = curr->next;
            curr->next = prev;
            prev = curr;
            curr = next;
        }
        ListNode *secondHalfStart = prev;

        // Compare two pointers
        ListNode *p1 = head;
        ListNode *p2 = secondHalfStart;
        bool result = true;
        
        while (result && p2 != NULL) {
            if (p1->val != p2->val) result = false;
            p1 = p1->next;
            p2 = p2->next;
        }        

        return result;       
    }
};