#define MIN(a, b) ((a) < (b) ? (a) : (b))

int minOperations(int *nums, int numsSize, int x)
{
  int sum = 0;

  for (int i = 0; i < numsSize; i++)
  {
    sum += nums[i];
  }

  if (sum < x)
  {
    return -1;
  }

  int right = 0;
  int lsum = 0, rsum = sum;
  int ans = numsSize + 1;

  for (int left = -1; left < numsSize; ++left)
  {
    if (left != -1)
    {
      lsum += nums[left];
    }
    while (right < numsSize && lsum + rsum > x)
    {
      rsum -= nums[right];
      ++right;
    }

    if (lsum + rsum == x)
    {
      ans = MIN(ans, (left + 1) + (numsSize - right));
    }
  }

  return ans > numsSize ? -1 : ans;
}