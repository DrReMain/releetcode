using namespace std;

struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution
{
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2)
    {
        ListNode *head = nullptr, *tail = nullptr;
        int c = 0;

        while (l1 || l2)
        {
            int sum = c + (l1 ? l1->val : 0) + (l2 ? l2->val : 0);

            if (!head)
            {
                head = tail = new ListNode(sum % 10);
            }
            else
            {
                tail->next = new ListNode(sum % 10);
                tail = tail->next;
            }

            c = sum / 10;
            l1 = l1 ? l1->next : nullptr;
            l2 = l2 ? l2->next : nullptr;
        }

        if (c > 0)
        {
            tail->next = new ListNode(c);
        }

        return head;
    }
};