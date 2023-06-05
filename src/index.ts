const practice = (nums: number[], index: number) => {
  if (nums[index]) {
    return nums[index] * 5;
  }
  return index * 5;
};

console.log(practice([12, 12, 12], 2));
