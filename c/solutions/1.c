#include <stdio.h>
#include <stdlib.h>


int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    int *a = (int*)malloc(2 * sizeof(int));

    int l[numsSize];

    for (int i = 0; i < numsSize; i++) {
        l[i] = target - nums[i];

        for (int j = 0; j < i; j++) {
            if (l[j] - nums[i] == 0) {
                a[0] = j;
                a[1] = i;
            }
        }
    }

    *returnSize = 2;
    return a;
}

int main() {
    int nums[] = {2,7,11,15};
    int numsSize = sizeof(nums) / sizeof(nums[0]);
    int target = 9;
    int returnSize = 2;

    int *result = twoSum(nums, numsSize, target, &returnSize);

    printf("[%d, %d]", result[0], result[1]);

    return 0;
}
