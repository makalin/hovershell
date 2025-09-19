/**
 * Async-related React hooks
 */

import { useState, useEffect, useCallback, useRef } from 'react';

/**
 * Hook for managing async operations with loading, error, and data states
 */
export function useAsync<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  immediate: boolean = false
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);

  const execute = useCallback(async (...args: any[]) => {
    try {
      setLoading(true);
      setError(null);
      const result = await asyncFunction(...args);
      setData(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunction]);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
  }, []);

  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return { data, loading, error, execute, reset };
}

/**
 * Hook for managing async operations with retry logic
 */
export function useAsyncWithRetry<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  options: {
    retries?: number;
    retryDelay?: number;
    immediate?: boolean;
  } = {}
) {
  const { retries = 3, retryDelay = 1000, immediate = false } = options;
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const [retryCount, setRetryCount] = useState(0);

  const execute = useCallback(async (...args: any[]) => {
    let currentRetry = 0;
    
    while (currentRetry <= retries) {
      try {
        setLoading(true);
        setError(null);
        setRetryCount(currentRetry);
        
        const result = await asyncFunction(...args);
        setData(result);
        setRetryCount(0);
        return result;
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'An error occurred';
        setError(errorMessage as E);
        
        if (currentRetry === retries) {
          setLoading(false);
          throw err;
        }
        
        currentRetry++;
        await new Promise(resolve => setTimeout(resolve, retryDelay));
      }
    }
  }, [asyncFunction, retries, retryDelay]);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    setRetryCount(0);
  }, []);

  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return { data, loading, error, retryCount, execute, reset };
}

/**
 * Hook for managing async operations with timeout
 */
export function useAsyncWithTimeout<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  timeout: number = 5000,
  immediate: boolean = false
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const [timedOut, setTimedOut] = useState<boolean>(false);

  const execute = useCallback(async (...args: any[]) => {
    try {
      setLoading(true);
      setError(null);
      setTimedOut(false);
      
      const timeoutPromise = new Promise<never>((_, reject) => {
        setTimeout(() => {
          reject(new Error(`Operation timed out after ${timeout}ms`));
        }, timeout);
      });
      
      const result = await Promise.race([
        asyncFunction(...args),
        timeoutPromise
      ]);
      
      setData(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      
      if (errorMessage.includes('timed out')) {
        setTimedOut(true);
      }
      
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunction, timeout]);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    setTimedOut(false);
  }, []);

  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return { data, loading, error, timedOut, execute, reset };
}

/**
 * Hook for managing multiple async operations
 */
export function useAsyncAll<T, E = string>(
  asyncFunctions: Array<(...args: any[]) => Promise<T>>,
  options: {
    immediate?: boolean;
    parallel?: boolean;
  } = {}
) {
  const { immediate = false, parallel = true } = options;
  const [data, setData] = useState<T[] | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const [results, setResults] = useState<Array<T | null>>(new Array(asyncFunctions.length).fill(null));
  const [completedCount, setCompletedCount] = useState(0);

  const execute = useCallback(async (...args: any[]) => {
    try {
      setLoading(true);
      setError(null);
      setCompletedCount(0);
      setResults(new Array(asyncFunctions.length).fill(null));
      
      let result: T[];
      
      if (parallel) {
        result = await Promise.all(asyncFunctions.map(fn => fn(...args)));
      } else {
        result = [];
        for (let i = 0; i < asyncFunctions.length; i++) {
          const res = await asyncFunctions[i](...args);
          result.push(res);
          setCompletedCount(i + 1);
          setResults(prev => {
            const newResults = [...prev];
            newResults[i] = res;
            return newResults;
          });
        }
      }
      
      setData(result);
      setResults(result.map(r => r));
      setCompletedCount(asyncFunctions.length);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunctions, parallel]);

  const executeWithProgress = useCallback(async (...args: any[]) => {
    try {
      setLoading(true);
      setError(null);
      setCompletedCount(0);
      setResults(new Array(asyncFunctions.length).fill(null));
      
      const result: T[] = [];
      
      for (let i = 0; i < asyncFunctions.length; i++) {
        try {
          const res = await asyncFunctions[i](...args);
          result.push(res);
          setCompletedCount(i + 1);
          setResults(prev => {
            const newResults = [...prev];
            newResults[i] = res;
            return newResults;
          });
        } catch (err) {
          // Continue with other functions even if one fails
          setCompletedCount(i + 1);
          setResults(prev => {
            const newResults = [...prev];
            newResults[i] = null;
            return newResults;
          });
        }
      }
      
      setData(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunctions]);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    setResults(new Array(asyncFunctions.length).fill(null));
    setCompletedCount(0);
  }, [asyncFunctions.length]);

  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return {
    data,
    loading,
    error,
    results,
    completedCount,
    totalCount: asyncFunctions.length,
    progress: asyncFunctions.length > 0 ? (completedCount / asyncFunctions.length) * 100 : 0,
    execute,
    executeWithProgress,
    reset,
  };
}

/**
 * Hook for managing async operations with caching
 */
export function useAsyncWithCache<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  options: {
    cacheKey?: string;
    ttl?: number;
    immediate?: boolean;
  } = {}
) {
  const { cacheKey, ttl = 5 * 60 * 1000, immediate = false } = options;
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const [fromCache, setFromCache] = useState<boolean>(false);
  
  const cacheRef = useRef<Map<string, { data: T; timestamp: number }>>(new Map());

  const execute = useCallback(async (...args: any[]) => {
    const key = cacheKey || JSON.stringify(args);
    const cached = cacheRef.current.get(key);
    
    // Check if cached data is still valid
    if (cached && Date.now() - cached.timestamp < ttl) {
      setData(cached.data);
      setFromCache(true);
      setLoading(false);
      return cached.data;
    }
    
    try {
      setLoading(true);
      setError(null);
      setFromCache(false);
      
      const result = await asyncFunction(...args);
      
      // Cache the result
      cacheRef.current.set(key, { data: result, timestamp: Date.now() });
      
      setData(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunction, cacheKey, ttl]);

  const invalidateCache = useCallback((key?: string) => {
    if (key) {
      cacheRef.current.delete(key);
    } else {
      cacheRef.current.clear();
    }
  }, []);

  const clearCache = useCallback(() => {
    cacheRef.current.clear();
  }, []);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    setFromCache(false);
  }, []);

  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return {
    data,
    loading,
    error,
    fromCache,
    execute,
    invalidateCache,
    clearCache,
    reset,
  };
}

/**
 * Hook for managing async operations with debouncing
 */
export function useAsyncDebounced<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  delay: number = 300,
  immediate: boolean = false
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  const execute = useCallback(async (...args: any[]) => {
    // Clear existing timeout
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
    
    return new Promise<T>((resolve, reject) => {
      timeoutRef.current = setTimeout(async () => {
        try {
          setLoading(true);
          setError(null);
          
          const result = await asyncFunction(...args);
          setData(result);
          resolve(result);
        } catch (err) {
          const errorMessage = err instanceof Error ? err.message : 'An error occurred';
          setError(errorMessage as E);
          reject(err);
        } finally {
          setLoading(false);
        }
      }, delay);
    });
  }, [asyncFunction, delay]);

  const cancel = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
      setLoading(false);
    }
  }, []);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    cancel();
  }, [cancel]);

  useEffect(() => {
    if (immediate) {
      execute();
    }
    
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [immediate, execute]);

  return { data, loading, error, execute, cancel, reset };
}

/**
 * Hook for managing async operations with throttling
 */
export function useAsyncThrottled<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  delay: number = 1000,
  immediate: boolean = false
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const lastCallRef = useRef<number>(0);

  const execute = useCallback(async (...args: any[]) => {
    const now = Date.now();
    
    if (now - lastCallRef.current < delay) {
      return data; // Return cached data if called too soon
    }
    
    try {
      setLoading(true);
      setError(null);
      lastCallRef.current = now;
      
      const result = await asyncFunction(...args);
      setData(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunction, delay, data]);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    lastCallRef.current = 0;
  }, []);

  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [immediate, execute]);

  return { data, loading, error, execute, reset };
}

/**
 * Hook for managing async operations with polling
 */
export function useAsyncPolling<T, E = string>(
  asyncFunction: (...args: any[]) => Promise<T>,
  interval: number = 5000,
  options: {
    immediate?: boolean;
    stopOnError?: boolean;
    maxAttempts?: number;
  } = {}
) {
  const { immediate = true, stopOnError = false, maxAttempts = Infinity } = options;
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<E | null>(null);
  const [isPolling, setIsPolling] = useState<boolean>(immediate);
  const [attempts, setAttempts] = useState<number>(0);
  
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const execute = useCallback(async (...args: any[]) => {
    try {
      setLoading(true);
      setError(null);
      
      const result = await asyncFunction(...args);
      setData(result);
      setAttempts(0);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An error occurred';
      setError(errorMessage as E);
      setAttempts(prev => prev + 1);
      
      if (stopOnError || attempts >= maxAttempts) {
        setIsPolling(false);
      }
      
      throw err;
    } finally {
      setLoading(false);
    }
  }, [asyncFunction, stopOnError, maxAttempts, attempts]);

  const startPolling = useCallback((...args: any[]) => {
    setIsPolling(true);
    setAttempts(0);
    
    // Execute immediately
    execute(...args);
    
    // Set up interval
    intervalRef.current = setInterval(() => {
      if (attempts < maxAttempts) {
        execute(...args);
      } else {
        setIsPolling(false);
      }
    }, interval);
  }, [execute, interval, attempts, maxAttempts]);

  const stopPolling = useCallback(() => {
    setIsPolling(false);
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }, []);

  const reset = useCallback(() => {
    setData(null);
    setError(null);
    setLoading(false);
    setAttempts(0);
    stopPolling();
  }, [stopPolling]);

  useEffect(() => {
    if (immediate) {
      startPolling();
    }
    
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [immediate, startPolling]);

  return {
    data,
    loading,
    error,
    isPolling,
    attempts,
    execute,
    startPolling,
    stopPolling,
    reset,
  };
}