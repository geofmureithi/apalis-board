import { KeyOf } from '@bull-board/api/typings/utils';
import { STATUSES } from '@bull-board/api/src/constants/statuses';

export const STATUS_LIST: Readonly<KeyOf<typeof STATUSES>> = [
  STATUSES.Latest as any,
  STATUSES.Running,
  STATUSES.Pending,
  STATUSES.Done,
  STATUSES.Failed,
  STATUSES.Scheduled,
  STATUSES.Killed,
] as const;
