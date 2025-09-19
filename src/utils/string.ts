/**
 * String utility functions
 */

/**
 * Capitalize the first letter of a string
 */
export function capitalize(str: string): string {
  if (!str) return str;
  return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Convert string to camelCase
 */
export function camelCase(str: string): string {
  return str
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) => {
      return index === 0 ? word.toLowerCase() : word.toUpperCase();
    })
    .replace(/\s+/g, '');
}

/**
 * Convert string to kebab-case
 */
export function kebabCase(str: string): string {
  return str
    .replace(/([a-z])([A-Z])/g, '$1-$2')
    .replace(/[\s_]+/g, '-')
    .toLowerCase();
}

/**
 * Convert string to snake_case
 */
export function snakeCase(str: string): string {
  return str
    .replace(/([a-z])([A-Z])/g, '$1_$2')
    .replace(/[\s-]+/g, '_')
    .toLowerCase();
}

/**
 * Convert string to PascalCase
 */
export function pascalCase(str: string): string {
  return str
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word) => {
      return word.toUpperCase();
    })
    .replace(/\s+/g, '');
}

/**
 * Convert string to Title Case
 */
export function titleCase(str: string): string {
  return str
    .toLowerCase()
    .split(' ')
    .map(word => capitalize(word))
    .join(' ');
}

/**
 * Convert string to sentence case
 */
export function sentenceCase(str: string): string {
  return str
    .toLowerCase()
    .replace(/(^\w|\.\s+\w)/g, (match) => match.toUpperCase());
}

/**
 * Truncate string to specified length
 */
export function truncate(str: string, length: number, suffix: string = '...'): string {
  if (str.length <= length) return str;
  return str.substring(0, length - suffix.length) + suffix;
}

/**
 * Truncate string at word boundary
 */
export function truncateWords(str: string, wordCount: number, suffix: string = '...'): string {
  const words = str.split(' ');
  if (words.length <= wordCount) return str;
  return words.slice(0, wordCount).join(' ') + suffix;
}

/**
 * Create URL-friendly slug from string
 */
export function slugify(str: string): string {
  return str
    .toLowerCase()
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_-]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

/**
 * Escape HTML characters
 */
export function escapeHtml(str: string): string {
  const htmlEscapes: Record<string, string> = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;',
  };
  
  return str.replace(/[&<>"']/g, (match) => htmlEscapes[match]);
}

/**
 * Unescape HTML characters
 */
export function unescapeHtml(str: string): string {
  const htmlUnescapes: Record<string, string> = {
    '&amp;': '&',
    '&lt;': '<',
    '&gt;': '>',
    '&quot;': '"',
    '&#39;': "'",
  };
  
  return str.replace(/&amp;|&lt;|&gt;|&quot;|&#39;/g, (match) => htmlUnescapes[match]);
}

/**
 * Remove HTML tags from string
 */
export function stripHtml(str: string): string {
  return str.replace(/<[^>]*>/g, '');
}

/**
 * Extract text content from HTML
 */
export function extractTextFromHtml(str: string): string {
  return stripHtml(str).replace(/\s+/g, ' ').trim();
}

/**
 * Pad string to specified length
 */
export function pad(str: string, length: number, padString: string = ' '): string {
  if (str.length >= length) return str;
  
  const padLength = length - str.length;
  const padStart = Math.floor(padLength / 2);
  const padEnd = padLength - padStart;
  
  return padString.repeat(padStart) + str + padString.repeat(padEnd);
}

/**
 * Pad string to the left
 */
export function padStart(str: string, length: number, padString: string = ' '): string {
  if (str.length >= length) return str;
  return padString.repeat(length - str.length) + str;
}

/**
 * Pad string to the right
 */
export function padEnd(str: string, length: number, padString: string = ' '): string {
  if (str.length >= length) return str;
  return str + padString.repeat(length - str.length);
}

/**
 * Repeat string n times
 */
export function repeat(str: string, count: number): string {
  if (count < 0) throw new Error('Count must be non-negative');
  if (count === 0) return '';
  if (count === 1) return str;
  
  let result = '';
  while (count > 0) {
    if (count & 1) result += str;
    count >>= 1;
    if (count) str += str;
  }
  return result;
}

/**
 * Reverse string
 */
export function reverse(str: string): string {
  return str.split('').reverse().join('');
}

/**
 * Check if string is empty or whitespace
 */
export function isEmpty(str: string): boolean {
  return !str || str.trim().length === 0;
}

/**
 * Check if string is blank (empty or only whitespace)
 */
export function isBlank(str: string): boolean {
  return !str || /^\s*$/.test(str);
}

/**
 * Remove leading and trailing whitespace
 */
export function trim(str: string): string {
  return str.trim();
}

/**
 * Remove leading whitespace
 */
export function trimStart(str: string): string {
  return str.trimStart();
}

/**
 * Remove trailing whitespace
 */
export function trimEnd(str: string): string {
  return str.trimEnd();
}

/**
 * Remove all whitespace
 */
export function removeWhitespace(str: string): string {
  return str.replace(/\s+/g, '');
}

/**
 * Normalize whitespace (replace multiple spaces with single space)
 */
export function normalizeWhitespace(str: string): string {
  return str.replace(/\s+/g, ' ').trim();
}

/**
 * Split string into words
 */
export function words(str: string): string[] {
  return str.split(/\s+/).filter(word => word.length > 0);
}

/**
 * Count words in string
 */
export function wordCount(str: string): number {
  return words(str).length;
}

/**
 * Count characters in string
 */
export function charCount(str: string): number {
  return str.length;
}

/**
 * Count lines in string
 */
export function lineCount(str: string): number {
  return str.split('\n').length;
}

/**
 * Get first n characters
 */
export function first(str: string, n: number = 1): string {
  return str.substring(0, n);
}

/**
 * Get last n characters
 */
export function last(str: string, n: number = 1): string {
  return str.substring(str.length - n);
}

/**
 * Remove first n characters
 */
export function removeFirst(str: string, n: number = 1): string {
  return str.substring(n);
}

/**
 * Remove last n characters
 */
export function removeLast(str: string, n: number = 1): string {
  return str.substring(0, str.length - n);
}

/**
 * Insert string at position
 */
export function insert(str: string, index: number, insertStr: string): string {
  if (index < 0 || index > str.length) {
    throw new Error('Index out of bounds');
  }
  return str.substring(0, index) + insertStr + str.substring(index);
}

/**
 * Replace all occurrences of substring
 */
export function replaceAll(str: string, search: string, replace: string): string {
  return str.split(search).join(replace);
}

/**
 * Remove all occurrences of substring
 */
export function removeAll(str: string, search: string): string {
  return replaceAll(str, search, '');
}

/**
 * Check if string starts with substring
 */
export function startsWith(str: string, search: string): boolean {
  return str.startsWith(search);
}

/**
 * Check if string ends with substring
 */
export function endsWith(str: string, search: string): boolean {
  return str.endsWith(search);
}

/**
 * Check if string contains substring
 */
export function contains(str: string, search: string): boolean {
  return str.includes(search);
}

/**
 * Check if string contains any of the substrings
 */
export function containsAny(str: string, searches: string[]): boolean {
  return searches.some(search => str.includes(search));
}

/**
 * Check if string contains all of the substrings
 */
export function containsAll(str: string, searches: string[]): boolean {
  return searches.every(search => str.includes(search));
}

/**
 * Find all occurrences of substring
 */
export function findAll(str: string, search: string): number[] {
  const indices: number[] = [];
  let index = str.indexOf(search);
  
  while (index !== -1) {
    indices.push(index);
    index = str.indexOf(search, index + 1);
  }
  
  return indices;
}

/**
 * Count occurrences of substring
 */
export function countOccurrences(str: string, search: string): number {
  return findAll(str, search).length;
}

/**
 * Wrap text to specified width
 */
export function wrap(str: string, width: number, breakChar: string = '\n'): string {
  const words = str.split(' ');
  const lines: string[] = [];
  let currentLine = '';
  
  for (const word of words) {
    if (currentLine.length + word.length + 1 <= width) {
      currentLine += (currentLine ? ' ' : '') + word;
    } else {
      if (currentLine) lines.push(currentLine);
      currentLine = word;
    }
  }
  
  if (currentLine) lines.push(currentLine);
  return lines.join(breakChar);
}

/**
 * Indent each line with specified string
 */
export function indent(str: string, indentStr: string = '  '): string {
  return str.split('\n').map(line => indentStr + line).join('\n');
}

/**
 * Remove indentation from each line
 */
export function dedent(str: string): string {
  const lines = str.split('\n');
  const minIndent = Math.min(
    ...lines
      .filter(line => line.trim().length > 0)
      .map(line => line.length - line.trimStart().length)
  );
  
  return lines.map(line => line.substring(minIndent)).join('\n');
}

/**
 * Convert string to array of characters
 */
export function toCharArray(str: string): string[] {
  return str.split('');
}

/**
 * Convert array of characters to string
 */
export function fromCharArray(chars: string[]): string {
  return chars.join('');
}

/**
 * Shuffle characters in string
 */
export function shuffle(str: string): string {
  const chars = toCharArray(str);
  for (let i = chars.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [chars[i], chars[j]] = [chars[j], chars[i]];
  }
  return fromCharArray(chars);
}

/**
 * Generate random string
 */
export function randomString(length: number = 10, charset: string = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'): string {
  let result = '';
  for (let i = 0; i < length; i++) {
    result += charset.charAt(Math.floor(Math.random() * charset.length));
  }
  return result;
}

/**
 * Generate UUID v4
 */
export function uuid(): string {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
    const r = Math.random() * 16 | 0;
    const v = c === 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

/**
 * Generate short ID
 */
export function shortId(): string {
  return Math.random().toString(36).substr(2, 9);
}

/**
 * Check if string is numeric
 */
export function isNumeric(str: string): boolean {
  return !isNaN(parseFloat(str)) && isFinite(Number(str));
}

/**
 * Check if string is integer
 */
export function isInteger(str: string): boolean {
  return /^-?\d+$/.test(str);
}

/**
 * Check if string is float
 */
export function isFloat(str: string): boolean {
  return /^-?\d*\.?\d+$/.test(str);
}

/**
 * Check if string is boolean
 */
export function isBoolean(str: string): boolean {
  return /^(true|false)$/i.test(str);
}

/**
 * Check if string is JSON
 */
export function isJson(str: string): boolean {
  try {
    JSON.parse(str);
    return true;
  } catch {
    return false;
  }
}

/**
 * Check if string is URL
 */
export function isUrl(str: string): boolean {
  try {
    new URL(str);
    return true;
  } catch {
    return false;
  }
}

/**
 * Check if string is email
 */
export function isEmail(str: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(str);
}

/**
 * Check if string is phone number
 */
export function isPhone(str: string): boolean {
  const phoneRegex = /^\+?[\d\s\-\(\)]+$/;
  return phoneRegex.test(str) && str.replace(/\D/g, '').length >= 10;
}

/**
 * Check if string is IP address
 */
export function isIp(str: string): boolean {
  const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
  return ipRegex.test(str);
}

/**
 * Check if string is UUID
 */
export function isUuid(str: string): boolean {
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
  return uuidRegex.test(str);
}

/**
 * Check if string is base64
 */
export function isBase64(str: string): boolean {
  try {
    return btoa(atob(str)) === str;
  } catch {
    return false;
  }
}

/**
 * Check if string is strong password
 */
export function isStrongPassword(str: string): boolean {
  // At least 8 characters, 1 uppercase, 1 lowercase, 1 number, 1 special character
  const strongRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/;
  return strongRegex.test(str);
}

/**
 * Mask string (hide characters except first and last few)
 */
export function mask(str: string, visibleStart: number = 2, visibleEnd: number = 2, maskChar: string = '*'): string {
  if (str.length <= visibleStart + visibleEnd) return str;
  
  const start = str.substring(0, visibleStart);
  const end = str.substring(str.length - visibleEnd);
  const middle = maskChar.repeat(str.length - visibleStart - visibleEnd);
  
  return start + middle + end;
}

/**
 * Mask email address
 */
export function maskEmail(email: string, maskChar: string = '*'): string {
  const [localPart, domain] = email.split('@');
  if (!localPart || !domain) return email;
  
  const maskedLocal = mask(localPart, 1, 1, maskChar);
  return `${maskedLocal}@${domain}`;
}

/**
 * Mask phone number
 */
export function maskPhone(phone: string, maskChar: string = '*'): string {
  const digits = phone.replace(/\D/g, '');
  if (digits.length < 4) return phone;
  
  const visibleStart = Math.min(2, digits.length - 2);
  const visibleEnd = Math.min(2, digits.length - visibleStart);
  
  return mask(digits, visibleStart, visibleEnd, maskChar);
}

/**
 * Highlight search terms in text
 */
export function highlight(str: string, search: string, highlightClass: string = 'highlight'): string {
  if (!search) return str;
  
  const regex = new RegExp(`(${escapeRegex(search)})`, 'gi');
  return str.replace(regex, `<span class="${highlightClass}">$1</span>`);
}

/**
 * Escape regex special characters
 */
export function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

/**
 * Convert string to array of bytes
 */
export function toBytes(str: string): number[] {
  const bytes: number[] = [];
  for (let i = 0; i < str.length; i++) {
    bytes.push(str.charCodeAt(i));
  }
  return bytes;
}

/**
 * Convert array of bytes to string
 */
export function fromBytes(bytes: number[]): string {
  return String.fromCharCode(...bytes);
}

/**
 * Convert string to base64
 */
export function toBase64(str: string): string {
  return btoa(str);
}

/**
 * Convert base64 to string
 */
export function fromBase64(str: string): string {
  return atob(str);
}

/**
 * Convert string to hex
 */
export function toHex(str: string): string {
  return Array.from(str)
    .map(char => char.charCodeAt(0).toString(16).padStart(2, '0'))
    .join('');
}

/**
 * Convert hex to string
 */
export function fromHex(str: string): string {
  const hex = str.replace(/[^0-9a-fA-F]/g, '');
  const bytes: number[] = [];
  
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(parseInt(hex.substr(i, 2), 16));
  }
  
  return String.fromCharCode(...bytes);
}

/**
 * Convert string to binary
 */
export function toBinary(str: string): string {
  return Array.from(str)
    .map(char => char.charCodeAt(0).toString(2).padStart(8, '0'))
    .join(' ');
}

/**
 * Convert binary to string
 */
export function fromBinary(str: string): string {
  return str.split(' ')
    .map(binary => String.fromCharCode(parseInt(binary, 2)))
    .join('');
}

/**
 * Levenshtein distance between two strings
 */
export function levenshteinDistance(str1: string, str2: string): number {
  const matrix: number[][] = [];
  
  for (let i = 0; i <= str2.length; i++) {
    matrix[i] = [i];
  }
  
  for (let j = 0; j <= str1.length; j++) {
    matrix[0][j] = j;
  }
  
  for (let i = 1; i <= str2.length; i++) {
    for (let j = 1; j <= str1.length; j++) {
      if (str2.charAt(i - 1) === str1.charAt(j - 1)) {
        matrix[i][j] = matrix[i - 1][j - 1];
      } else {
        matrix[i][j] = Math.min(
          matrix[i - 1][j - 1] + 1, // substitution
          matrix[i][j - 1] + 1,     // insertion
          matrix[i - 1][j] + 1      // deletion
        );
      }
    }
  }
  
  return matrix[str2.length][str1.length];
}

/**
 * Calculate similarity between two strings (0-1)
 */
export function similarity(str1: string, str2: string): number {
  const maxLength = Math.max(str1.length, str2.length);
  if (maxLength === 0) return 1;
  
  const distance = levenshteinDistance(str1, str2);
  return 1 - distance / maxLength;
}

/**
 * Find fuzzy matches in array of strings
 */
export function fuzzyMatch(query: string, strings: string[], threshold: number = 0.6): Array<{ string: string; score: number }> {
  return strings
    .map(str => ({ string: str, score: similarity(query.toLowerCase(), str.toLowerCase()) }))
    .filter(match => match.score >= threshold)
    .sort((a, b) => b.score - a.score);
}

/**
 * Sort strings by similarity to query
 */
export function sortBySimilarity(query: string, strings: string[]): string[] {
  return strings
    .map(str => ({ string: str, score: similarity(query.toLowerCase(), str.toLowerCase()) }))
    .sort((a, b) => b.score - a.score)
    .map(item => item.string);
}