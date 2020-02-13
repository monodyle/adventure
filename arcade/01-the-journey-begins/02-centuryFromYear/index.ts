function centuryFromYear(year: number): number {
  let d: number = year % 100
  let c: number = Math.floor(year / 100)
  if (d == 0) return c
  else return c + 1
}

export { centuryFromYear }