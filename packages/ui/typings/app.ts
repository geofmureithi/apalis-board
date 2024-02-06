import { STATUSES } from '../src/constants/statuses';

export type SelectedStatuses = Record<AppQueue['name'], Status>;

export interface QueueActions {
  promoteJob: (queueName: string) => (job: AppJob) => () => Promise<void>;
  retryJob: (queueName: string) => (job: AppJob) => () => Promise<void>;
  cleanJob: (queueName: string) => (job: AppJob) => () => Promise<void>;
  getJobLogs: (queueName: string) => (job: AppJob) => () => Promise<string[]>;
  retryAll: (queueName: string) => () => Promise<void>;
  cleanAllDelayed: (queueName: string) => () => Promise<void>;
  cleanAllFailed: (queueName: string) => () => Promise<void>;
  cleanAllCompleted: (queueName: string) => () => Promise<void>;
  pauseQueue: (queueName: string) => () => Promise<void>;
  resumeQueue: (queueName: string) => () => Promise<void>;
}


export type Status = keyof typeof STATUSES;

export type JobStatus = keyof Omit<typeof STATUSES, 'latest'>;

export type JobCounts = Record<Status, number>;

export interface QueueAdapterOptions {
  readOnlyMode: boolean;
  allowRetries: boolean;
  prefix: string;
}

export interface QueueJob {
  opts: {
    delay?: number | undefined;
  };

  promote(): Promise<void>;

  remove(): Promise<void>;

  retry(): Promise<void>;

  toJSON(): QueueJobJson;
}

export interface QueueJobJson {
  context: {
    id?: string | undefined | number | null;
    name: string;
    // eslint-disable-next-line @typescript-eslint/ban-types
    progress: number | object;
    attempts: number;
    done_at?: number | null;
    lock_at?: number | null;
    run_at: number;
    last_error: string;
    lock_by?: string | null;
  };
  stacktrace: string[] | null;
  job: any;
  returnvalue: any;
  opts: any;
  parentKey?: string;
}

export interface ValidMetrics {
  total_system_memory: string;
  redis_version: string;
  used_memory: string;
  mem_fragmentation_ratio: string;
  connected_clients: string;
  blocked_clients: string;
}

export interface AppJob {
  context: {
    id: QueueJobJson['context']['id'];
    name: QueueJobJson['context']['name'];
    run_at: QueueJobJson['context']['run_at'];
    lock_at?: QueueJobJson['context']['lock_at'];
    done_at?: QueueJobJson['context']['done_at'];
    progress: QueueJobJson['context']['progress'];
    attempts: QueueJobJson['context']['attempts'];
    last_error: QueueJobJson['context']['last_error'];
    lock_by: QueueJobJson['context']['lock_by'];
  };
  delay: number | undefined;
  opts: QueueJobJson['opts'];
  job: QueueJobJson['job'];
  returnValue: QueueJobJson['returnvalue'];
}

export interface AppQueue {
  name: string;
  readOnlyMode: boolean;
  allowRetries: boolean;
  isPaused: boolean;
}

export type HTTPMethod = 'get' | 'post' | 'put';
export type HTTPStatus = 200 | 204 | 404 | 405 | 500;

export type Promisify<T> = T | Promise<T>;

export interface Pagination {
  pageCount: number;
}

export interface WorkerId {
  name: string;
}

export interface ApalisWorker {
  worker_id: WorkerId;
  job_type: string;
  source: string;
  layers: string;
  last_seen: string;
}

export type FormatterField = 'data' | 'returnValue' | 'name';
