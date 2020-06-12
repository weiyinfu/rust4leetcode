class Solution {
public:
  ListNode *partition(ListNode *head, int x) {
    ListNode small_head(0);
    ListNode *small_end = &small_head;
    ListNode big_head(0);
    ListNode *big_end = &big_head;

    for (ListNode *i = head; i != NULL; i = i->next) {
      if (i->val < x) {
        small_end->next = i;
        small_end = i;
      } else {
        big_end->next = i;
        big_end = i;
      }
    }
    small_end->next = big_head.next;
    big_end->next = NULL;
    return small_head.next;
  }
};