import React from 'react';
import { Store } from '../../hooks/useStore';
import { RetryIcon } from '../Icons/Retry';
import { TrashIcon } from '../Icons/Trash';
import { Button } from '../JobCard/Button/Button';
import s from './QueueActions.module.css';
import { AppQueue, Status } from '../../../typings/app';
import { STATUSES } from '../../constants/statuses';

interface QueueActionProps {
  queue: AppQueue;
  actions: Store['actions'];
  status: Status;
  allowRetries: boolean;
}

const ACTIONABLE_STATUSES = [STATUSES.Failed, STATUSES.Scheduled, STATUSES.Done] as const;

const isStatusActionable = (status: Status): boolean => ACTIONABLE_STATUSES.includes(status as any);

const CleanAllButton = ({ onClick }: any) => (
  <Button onClick={onClick} className={s.button}>
    <TrashIcon />
    Clean all
  </Button>
);

export const QueueActions = ({ status, actions, queue, allowRetries }: QueueActionProps) => {
  if (!isStatusActionable(status)) {
    return null;
  }

  return (
    <ul className={s.queueActions}>
      {status === STATUSES.Failed && (
        <>
          {allowRetries && (
            <li>
              <Button onClick={actions.retryAll(queue.name)} className={s.button}>
                <RetryIcon />
                Retry all
              </Button>
            </li>
          )}
          <li>
            <CleanAllButton onClick={actions.cleanAllFailed(queue.name)} />
          </li>
        </>
      )}
      {status === STATUSES.Scheduled && (
        <li>
          <CleanAllButton onClick={actions.cleanAllDelayed(queue.name)} />
        </li>
      )}
      {status === STATUSES.Done && (
        <li>
          <CleanAllButton onClick={actions.cleanAllCompleted(queue.name)} />
        </li>
      )}
    </ul>
  );
};
