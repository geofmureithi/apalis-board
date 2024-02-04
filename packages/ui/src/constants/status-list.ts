import { KeyOf } from '../../typings/utils';
import { STATUSES } from './statuses';

export const STATUS_LIST: Readonly<KeyOf<typeof STATUSES>> = [
  STATUSES.Running,
  STATUSES.Pending,
  STATUSES.Done,
  STATUSES.Retry,
  STATUSES.Failed,
  STATUSES.Killed,
] as const;
