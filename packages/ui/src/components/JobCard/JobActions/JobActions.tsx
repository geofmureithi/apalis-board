import React from 'react';
import { PromoteIcon } from '../../Icons/Promote';
import { RetryIcon } from '../../Icons/Retry';
import { TrashIcon } from '../../Icons/Trash';
import { Tooltip } from '../../Tooltip/Tooltip';
import { Button } from '../Button/Button';
import s from './JobActions.module.css';
import { Status } from '../../../../typings/app';
import { STATUSES } from '../../../constants/statuses';

interface JobActionsProps {
  status: Status;
  allowRetries: boolean;
  actions: {
    promoteJob: () => Promise<void>;
    retryJob: () => Promise<void>;
    cleanJob: () => Promise<void>;
  };
}

interface ButtonType {
  title: string;
  Icon: React.ElementType;
  actionKey: 'promoteJob' | 'cleanJob' | 'retryJob';
}

const buttonTypes: Record<string, ButtonType> = {
  promote: { title: 'Promote', Icon: PromoteIcon, actionKey: 'promoteJob' },
  clean: { title: 'Clean', Icon: TrashIcon, actionKey: 'cleanJob' },
  retry: { title: 'Retry', Icon: RetryIcon, actionKey: 'retryJob' },
};

const statusToButtonsMap: Record<string, ButtonType[]> = {
  [STATUSES.Retry]: [buttonTypes.promote, buttonTypes.clean],
  [STATUSES.Failed]: [buttonTypes.retry, buttonTypes.clean],
  [STATUSES.Done]: [buttonTypes.clean],
  [STATUSES.Pending]: [buttonTypes.clean],
};

export const JobActions = ({ actions, status, allowRetries }: JobActionsProps) => {
  let buttons = statusToButtonsMap[status];
  if (!buttons) {
    return null;
  }
  if (!allowRetries) {
    buttons = buttons.filter((btn) => btn.actionKey !== 'retryJob');
  }
  return (
    <ul className={s.jobActions}>
      {buttons.map((type) => (
        <li key={type.title}>
          <Tooltip title={type.title}>
            <Button onClick={actions[type.actionKey]} className={s.button}>
              <type.Icon />
            </Button>
          </Tooltip>
        </li>
      ))}
    </ul>
  );
};
