/*
 * Problem: Given a rectangular matrix of characters, add a border of asterisks(`*`) to it.
 * Example:
 *  For ["abc",
 *       "ded"],
 *  the output should be ["*****",
 *                        "*abc*",
 *                        "*ded*",
 *                        "*****"]
 */

function addBorder(picture: string[]): string[] {
  let wrap: string = "*".repeat(picture[0].length + 2)
  return [wrap, ...picture.map(i => `*${i}*`), wrap]
}

export { addBorder }