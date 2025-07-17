import { invoke } from '@tauri-apps/api/core';

export enum LogLevel {
  DEBUG = 'DEBUG',
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR',
}

class Logger {
  private static instance: Logger;
  private logPath: string | null = null;
  private initialized = false;

  private constructor() {}

  static getInstance(): Logger {
    if (!Logger.instance) {
      Logger.instance = new Logger();
    }
    return Logger.instance;
  }

  async init(): Promise<void> {
    if (this.initialized) return;
    
    try {
      this.logPath = await invoke<string>('get_frontend_log_path');
      this.initialized = true;
      this.info('Frontend logger initialized');
    } catch (error) {
      console.error('Failed to initialize frontend logger:', error);
    }
  }

  private async writeLog(level: LogLevel, message: string, ...args: any[]): Promise<void> {
    const timestamp = new Date().toISOString();
    const fullMessage = args.length > 0 ? `${message} ${args.map(arg => 
      typeof arg === 'object' ? JSON.stringify(arg, null, 2) : String(arg)
    ).join(' ')}` : message;

    // Always log to console in development
    if (process.env.NODE_ENV === 'development') {
      const consoleMethod = level === LogLevel.ERROR ? 'error' : 
                          level === LogLevel.WARN ? 'warn' : 
                          level === LogLevel.INFO ? 'info' : 'debug';
      console[consoleMethod](`[${timestamp}] [${level}]`, message, ...args);
    }

    // Write to file if initialized
    if (this.initialized && this.logPath) {
      try {
        await invoke('write_frontend_log', {
          level: level.toString(),
          message: fullMessage,
          timestamp
        });
      } catch (error) {
        console.error('Failed to write to log file:', error);
      }
    }
  }

  debug(message: string, ...args: any[]): void {
    this.writeLog(LogLevel.DEBUG, message, ...args);
  }

  info(message: string, ...args: any[]): void {
    this.writeLog(LogLevel.INFO, message, ...args);
  }

  warn(message: string, ...args: any[]): void {
    this.writeLog(LogLevel.WARN, message, ...args);
  }

  error(message: string, ...args: any[]): void {
    this.writeLog(LogLevel.ERROR, message, ...args);
  }

  // Convenience method for logging objects
  logObject(level: LogLevel, label: string, obj: any): void {
    this.writeLog(level, `${label}:`, obj);
  }
}

// Export singleton instance
export const logger = Logger.getInstance();

// Auto-initialize when module is imported
logger.init().catch(console.error);

// Export convenience functions
export const log = {
  debug: (message: string, ...args: any[]) => logger.debug(message, ...args),
  info: (message: string, ...args: any[]) => logger.info(message, ...args),
  warn: (message: string, ...args: any[]) => logger.warn(message, ...args),
  error: (message: string, ...args: any[]) => logger.error(message, ...args),
  object: (level: LogLevel, label: string, obj: any) => logger.logObject(level, label, obj),
};