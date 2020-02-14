/*
 * Problem: Given a year, return the century it is in. The first century spans from the year 1 up to and including the year 100, the second - from the year 101 up to and including the year 200, etc.
 * Example:
 *  For `year = 1905`, the output should be `centuryFromYear(year) = 20`
 *  For `year = 1700`, the output should be `centuryFromYear(year) = 17`
 */

function centuryFromYear(year: number): number {
  let d: number = year % 100
  let c: number = Math.floor(year / 100)
  if (d == 0) return c
  else return c + 1
}

export { centuryFromYear }