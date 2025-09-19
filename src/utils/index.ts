/**
 * Utility functions for HoverShell
 */

// Re-export string utilities
export * from './string';

// Main utility functions
export const utils = {
  // String utilities
  string: {
    capitalize: (str: string) => str.charAt(0).toUpperCase() + str.slice(1),
    camelCase: (str: string) => str.replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) => index === 0 ? word.toLowerCase() : word.toUpperCase()).replace(/\s+/g, ''),
    kebabCase: (str: string) => str.replace(/([a-z])([A-Z])/g, '$1-$2').replace(/[\s_]+/g, '-').toLowerCase(),
    snakeCase: (str: string) => str.replace(/([a-z])([A-Z])/g, '$1_$2').replace(/[\s-]+/g, '_').toLowerCase(),
    pascalCase: (str: string) => str.replace(/(?:^\w|[A-Z]|\b\w)/g, word => word.toUpperCase()).replace(/\s+/g, ''),
    truncate: (str: string, length: number, suffix = '...') => str.length <= length ? str : str.substring(0, length - suffix.length) + suffix,
    slugify: (str: string) => str.toLowerCase().replace(/[^\w\s-]/g, '').replace(/[\s_-]+/g, '-').replace(/^-+|-+$/g, ''),
    escapeHtml: (str: string) => str.replace(/[&<>"']/g, (match) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[match] || match)),
    unescapeHtml: (str: string) => str.replace(/&amp;|&lt;|&gt;|&quot;|&#39;/g, (match) => ({ '&amp;': '&', '&lt;': '<', '&gt;': '>', '&quot;': '"', '&#39;': "'" }[match] || match)),
  },

  // Array utilities
  array: {
    unique: <T>(arr: T[]) => [...new Set(arr)],
    chunk: <T>(arr: T[], size: number) => Array.from({ length: Math.ceil(arr.length / size) }, (_, i) => arr.slice(i * size, i * size + size)),
    flatten: <T>(arr: (T | T[])[]): T[] => arr.reduce<T[]>((flat, item) => flat.concat(Array.isArray(item) ? item : [item]), []),
    groupBy: <T, K extends keyof T>(arr: T[], key: K) => arr.reduce((groups, item) => {
      const group = String(item[key]);
      groups[group] = groups[group] || [];
      groups[group].push(item);
      return groups;
    }, {} as Record<string, T[]>),
    sortBy: <T>(arr: T[], key: keyof T, direction: 'asc' | 'desc' = 'asc') => [...arr].sort((a, b) => {
      const aVal = a[key];
      const bVal = b[key];
      if (aVal < bVal) return direction === 'asc' ? -1 : 1;
      if (aVal > bVal) return direction === 'asc' ? 1 : -1;
      return 0;
    }),
    shuffle: <T>(arr: T[]) => {
      const shuffled = [...arr];
      for (let i = shuffled.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
      }
      return shuffled;
    },
    sample: <T>(arr: T[], count: number = 1) => {
      const shuffled = utils.array.shuffle(arr);
      return count === 1 ? shuffled[0] : shuffled.slice(0, count);
    },
  },

  // Object utilities
  object: {
    deepClone: <T>(obj: T): T => {
      if (obj === null || typeof obj !== 'object') return obj;
      if (obj instanceof Date) return new Date(obj.getTime()) as T;
      if (obj instanceof Array) return obj.map(item => utils.object.deepClone(item)) as T;
      if (typeof obj === 'object') {
        const cloned = {} as T;
        for (const key in obj) {
          if (obj.hasOwnProperty(key)) {
            cloned[key] = utils.object.deepClone(obj[key]);
          }
        }
        return cloned;
      }
      return obj;
    },
    deepMerge: <T extends Record<string, any>>(target: T, ...sources: Partial<T>[]): T => {
      if (!sources.length) return target;
      const source = sources.shift();
      
      if (utils.object.isObject(target) && utils.object.isObject(source)) {
        for (const key in source) {
          if (utils.object.isObject(source[key])) {
            if (!target[key]) Object.assign(target, { [key]: {} });
            utils.object.deepMerge(target[key], source[key] || {});
          } else {
            Object.assign(target, { [key]: source[key] });
          }
        }
      }
      
      return utils.object.deepMerge(target, ...sources);
    },
    isObject: (item: any): boolean => item && typeof item === 'object' && !Array.isArray(item),
    pick: <T extends Record<string, any>, K extends keyof T>(obj: T, keys: K[]): Pick<T, K> => {
      const result = {} as Pick<T, K>;
      keys.forEach(key => {
        if (key in obj) {
          result[key] = obj[key];
        }
      });
      return result;
    },
    omit: <T extends Record<string, any>, K extends keyof T>(obj: T, keys: K[]): Omit<T, K> => {
      const result = { ...obj };
      keys.forEach(key => {
        delete result[key];
      });
      return result;
    },
    isEmpty: (obj: any): boolean => {
      if (obj == null) return true;
      if (Array.isArray(obj) || typeof obj === 'string') return obj.length === 0;
      if (typeof obj === 'object') return Object.keys(obj).length === 0;
      return false;
    },
  },

  // Date utilities
  date: {
    format: (date: Date, format: string = 'YYYY-MM-DD HH:mm:ss'): string => {
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1).padStart(2, '0');
      const day = String(date.getDate()).padStart(2, '0');
      const hours = String(date.getHours()).padStart(2, '0');
      const minutes = String(date.getMinutes()).padStart(2, '0');
      const seconds = String(date.getSeconds()).padStart(2, '0');
      
      return format
        .replace('YYYY', String(year))
        .replace('MM', month)
        .replace('DD', day)
        .replace('HH', hours)
        .replace('mm', minutes)
        .replace('ss', seconds);
    },
    parse: (dateString: string): Date => {
      return new Date(dateString);
    },
    isToday: (date: Date): boolean => {
      const today = new Date();
      return date.getDate() === today.getDate() &&
             date.getMonth() === today.getMonth() &&
             date.getFullYear() === today.getFullYear();
    },
    addDays: (date: Date, days: number): Date => {
      const result = new Date(date);
      result.setDate(result.getDate() + days);
      return result;
    },
    humanize: (date: Date): string => {
      const now = new Date();
      const diff = now.getTime() - date.getTime();
      const seconds = Math.floor(diff / 1000);
      const minutes = Math.floor(seconds / 60);
      const hours = Math.floor(minutes / 60);
      const days = Math.floor(hours / 24);
      
      if (days > 0) return `${days} day${days > 1 ? 's' : ''} ago`;
      if (hours > 0) return `${hours} hour${hours > 1 ? 's' : ''} ago`;
      if (minutes > 0) return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
      return 'just now';
    },
  },

  // Number utilities
  number: {
    format: (num: number, options?: Intl.NumberFormatOptions): string => {
      return new Intl.NumberFormat('en-US', options).format(num);
    },
    formatBytes: (bytes: number): string => {
      if (bytes === 0) return '0 Bytes';
      const k = 1024;
      const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    },
    clamp: (num: number, min: number, max: number): number => {
      return Math.min(Math.max(num, min), max);
    },
    random: (min: number, max: number): number => {
      return Math.floor(Math.random() * (max - min + 1)) + min;
    },
    round: (num: number, decimals: number = 0): number => {
      return Math.round(num * Math.pow(10, decimals)) / Math.pow(10, decimals);
    },
  },

  // Async utilities
  async: {
    sleep: (ms: number): Promise<void> => new Promise(resolve => setTimeout(resolve, ms)),
    timeout: <T>(promise: Promise<T>, ms: number): Promise<T> => {
      const timeoutPromise = new Promise<never>((_, reject) => {
        setTimeout(() => reject(new Error(`Timeout after ${ms}ms`)), ms);
      });
      return Promise.race([promise, timeoutPromise]);
    },
    debounce: <T extends (...args: any[]) => any>(fn: T, delay: number): T => {
      let timeoutId: NodeJS.Timeout;
      
      return ((...args: any[]) => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => fn(...args), delay);
      }) as T;
    },
    throttle: <T extends (...args: any[]) => any>(fn: T, delay: number): T => {
      let lastCall = 0;
      
      return ((...args: any[]) => {
        const now = Date.now();
        if (now - lastCall >= delay) {
          lastCall = now;
          return fn(...args);
        }
      }) as T;
    },
  },

  // Validation utilities
  validation: {
    isEmail: (email: string): boolean => {
      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
      return emailRegex.test(email);
    },
    isUrl: (url: string): boolean => {
      try {
        new URL(url);
        return true;
      } catch {
        return false;
      }
    },
    isJson: (json: string): boolean => {
      try {
        JSON.parse(json);
        return true;
      } catch {
        return false;
      }
    },
  },

  // Generate utilities
  generate: {
    id: (): string => `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    uuid: (): string => {
      return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
        const r = Math.random() * 16 | 0;
        const v = c === 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
      });
    },
    randomString: (length: number = 10): string => {
      const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
      let result = '';
      for (let i = 0; i < length; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length));
      }
      return result;
    },
  },
};

// Default export
export default utils;