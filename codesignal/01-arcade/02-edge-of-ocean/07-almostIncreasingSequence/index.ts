function almostIncreasingSequence(sequence: number[]): boolean {
  let flag: number = 0
  for (let i = 1; i <= sequence.length; i++) {
    if (sequence[i] <= sequence[i - 1]) {
      flag++
      console.log(flag)
      if (flag > 1)
        return false
      if (sequence[i] <= sequence[i - 2] && sequence[i + 1] <= sequence[i - 1])
        return false
    }
  }
  return true
}

export { almostIncreasingSequence }