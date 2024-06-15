/*
 * Problem:
 *  Given an array of words and a length l, format the text such that each line has exactly l characters and is fully justified on both the left and the right. Words should be packed in a greedy approach; that is, pack as many words as possible in each line. Add extra spaces when necessary so that each line has exactly l characters.
 *  Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right. For the last line of text and lines with one word only, the words should be left justified with no extra space inserted between them.
 * Example:
 *   For ["This", "is", "an", "example", "of", "text", "justification."]
 *   and l = 16, the output should be ["This    is    an",
 *                                     "example  of text",
 *                                     "justification.  "]
 *
 */

function textJustification(words: string[], l: number): string[] {
  let result: string[] = []
  let currentLine: string[] = [],
      currentLength: number = 0

  const addWord = (word: string): boolean => {
    if (word.length + currentLength + currentLine.length > l) return false
    currentLine.push(word)
    currentLength += word.length
    return true
  }

  const parseLine = (lastest: boolean = false): string => {
    if (lastest) return currentLine.join(" ") + " ".repeat(l - currentLength - currentLine.length + 1)
    if (currentLine.length === 1) return currentLine[0] + " ".repeat(l - currentLength)

    const spaces: number = l - currentLength
    const baseSpaces: number = Math.floor(spaces / (currentLine.length - 1))
    const extraSpaces: number = spaces % (currentLine.length - 1)

    let string: string = ""

    for (let i: number = 0; i < currentLine.length - 1; ++i) {
      string += currentLine[i]
      string += ' '.repeat(i < extraSpaces ? baseSpaces + 1 : baseSpaces)
    }

    string += currentLine[currentLine.length - 1]
    return string
  }

  for (const word of words) {
    if (addWord(word)) continue

    result.push(parseLine())
    currentLine.length = 0
    currentLength = 0
    addWord(word)
  }

  // Last line wasn't parsed, sad :(
  result.push(parseLine(true))

  return result
}

export { textJustification }