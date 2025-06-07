// Last updated: 6/7/2025, 2:42:06 AM
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
    ListNode* removeElements(ListNode* head, int val) {
        ListNode* prev = NULL;
        ListNode* curr = head;
        while(curr != NULL){
            if(curr->val == val){
                if(prev != NULL){
                    prev->next = curr->next;
                }
                else{
                    head = head->next;
                }
            }
            else{
                prev = curr;
            }
            curr = curr->next;
        }
        return head;        
    }
};