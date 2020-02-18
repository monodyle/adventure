/*
 * Problem:
 *  An IP address is a numerical label assigned to each device (e.g., computer, printer) participating in a computer network that uses the Internet Protocol for communication. There are two versions of the Internet protocol, and thus two versions of addresses. One of them is the IPv4 address.
 *  Given a string, find out if it satisfies the IPv4 address naming rules.
 * Example:
 *  For "172.16.254.1" the output should be `true`
 *  For "172.316.254.1" the output should be `false`
 *    316 is not in range [0, 255]
 *  For ".254.255.0" the output should be `false`
 *    There is no first number
 */

const isIPv4Address = (input: string): boolean => {
  return /^(?:(?:^|\.)(?:2(?:5[0-5]|[0-4]\d)|1?\d?\d)){4}$/.test(input)
}

export { isIPv4Address }