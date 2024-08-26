#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode *result = (struct ListNode*)malloc(sizeof(struct ListNode));
    struct ListNode *curr = result;
    int num = 0;

    while (l1 || l2 || num) {
        if (l1) {
            num += l1->val;
            l1 = l1->next;
        }

        if (l2) {
            num += l2->val;
            l2 = l2->next;
        }

            curr->val = num % 10;
            num = num / 10;
            if (l1 || l2 || num) {
                curr->next = (struct ListNode *)malloc(sizeof(struct ListNode));
                curr = curr->next;
            }
                curr->next = NULL;

    }

    return result;
}

// Function to print the linked list
void printLinkedList(struct ListNode* head) {
    while (head != NULL) {
        printf("%d ", head->val);
        head = head->next;
    }
    printf("\n");
}

int main() {
    // Example usage
    struct ListNode* l1 = (struct ListNode*)malloc(sizeof(struct ListNode));
    struct ListNode* l2 = (struct ListNode*)malloc(sizeof(struct ListNode));
    
    l1->val = 2;
    l1->next = (struct ListNode*)malloc(sizeof(struct ListNode));
    l1->next->val = 4;
    l1->next->next = (struct ListNode*)malloc(sizeof(struct ListNode));
    l1->next->next->val = 3;
    
    l2->val = 5;
    l2->next = (struct ListNode*)malloc(sizeof(struct ListNode));
    l2->next->val = 6;
    l2->next->next = (struct ListNode*)malloc(sizeof(struct ListNode));
    l2->next->next->val= 4;
    
    struct ListNode* result = addTwoNumbers(l1, l2);
    
    printLinkedList(result);
    
    free(l1->next);
    free(l2->next);
    free(l1);
    free(l2);
    
    return 0;
}
