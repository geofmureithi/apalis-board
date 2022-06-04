import { KeyOf } from '../../typings/utils';
import { STATUSES } from './statuses';

export const STATUS_LIST: Readonly<KeyOf<typeof STATUSES>> = [
  STATUSES.Running,
  STATUSES.Pending,
  STATUSES.Done,
  STATUSES.Failed,
  STATUSES.Scheduled,
  STATUSES.Killed,
] as const;
