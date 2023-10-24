# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if head == None:
            return
        l = []
        var = head
        while not var == None:
            l.append(var.val)
            var = var.next

        i = 0
        j = len(l) - 1
        even = True
        var = head
        while not i > j:
            if even:
                var.val = l[i]
                var = var.next
                i += 1
                even = False
            else:
                var.val = l[j]
                var = var.next
                j -= 1
                even = True