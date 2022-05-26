import { BaseAdapter } from '../src/queueAdapters/base';
import { STATUSES } from '../src/constants/statuses';

export type JobCleanStatus = 'Done' | 'Scheduled' | 'Running' | 'Killed' | 'Failed';

export type Status = keyof typeof STATUSES;

export type JobStatus = keyof Omit<typeof STATUSES, 'latest'>;

export type JobCounts = Record<Status, number>;

export interface QueueAdapterOptions {
  readOnlyMode: boolean;
  allowRetries: boolean;
  prefix: string;
}

export type BullBoardQueues = Map<string, BaseAdapter>;

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
  // add properties as needed from real Bull/BullMQ jobs
  id?: string | undefined | number | null;
  name: string;
  // eslint-disable-next-line @typescript-eslint/ban-types
  progress: number | object;
  attempts: number;
  done_at?: number | null;
  lock_at?: number | null;
  run_at: number;
  last_error: string;
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
  id: QueueJobJson['id'];
  name: QueueJobJson['name'];
  run_at: QueueJobJson['run_at'];
  lock_at?: QueueJobJson['lock_at'];
  done_at?: QueueJobJson['done_at'];
  progress: QueueJobJson['progress'];
  attempts: QueueJobJson['attempts'];
  last_error: QueueJobJson['last_error'];
  delay: number | undefined;
  opts: QueueJobJson['opts'];
  job: QueueJobJson['job'];
  returnValue: QueueJobJson['returnvalue'];
}

export interface AppQueue {
  name: string;
  url?: string;
  counts: Record<Status, number>;
  jobs: AppJob[];
  pagination: Pagination;
  readOnlyMode: boolean;
  allowRetries: boolean;
  isPaused: boolean;
}

export type HTTPMethod = 'get' | 'post' | 'put';
export type HTTPStatus = 200 | 204 | 404 | 405 | 500;

export interface BullBoardRequest {
  queues: BullBoardQueues;
  query: Record<string, any>;
  params: Record<string, any>;
}

export type ControllerHandlerReturnType = {
  status?: HTTPStatus;
  body: string | Record<string, any>;
};

export type ViewHandlerReturnType = {
  name: string;
};

export type Promisify<T> = T | Promise<T>;

export interface AppControllerRoute {
  method: HTTPMethod | HTTPMethod[];
  route: string | string[];

  handler(request?: BullBoardRequest): Promisify<ControllerHandlerReturnType>;
}

export interface AppViewRoute {
  method: HTTPMethod;
  route: string | string[];

  handler(request?: BullBoardRequest): ViewHandlerReturnType;
}

export type AppRouteDefs = {
  entryPoint: AppViewRoute;
  api: AppControllerRoute[];
};

export interface IServerAdapter {
  setQueues(bullBoardQueues: BullBoardQueues): IServerAdapter;

  setViewsPath(viewPath: string): IServerAdapter;

  setStaticPath(staticsRoute: string, staticsPath: string): IServerAdapter;

  setEntryRoute(route: AppViewRoute): IServerAdapter;

  setErrorHandler(handler: (error: Error) => ControllerHandlerReturnType): IServerAdapter;

  setApiRoutes(routes: AppControllerRoute[]): IServerAdapter;
}

export interface Pagination {
  pageCount: number;
  range: {
    start: number;
    end: number;
  };
}

export type FormatterField = 'data' | 'returnValue' | 'name';
