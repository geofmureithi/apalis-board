import React from 'react';
import { Details } from './Details/Details';
import { JobActions } from './JobActions/JobActions';
import s from './JobCard.module.css';
import { Progress } from './Progress/Progress';
import { Timeline } from './Timeline/Timeline';
import { AppJob, Status } from '../../../typings/app';
import { STATUSES } from '../../constants/statuses';

interface JobCardProps {
  job: AppJob;
  status: Status;
  readOnlyMode: boolean;
  allowRetries: boolean;
  actions: {
    promoteJob: () => Promise<void>;
    retryJob: () => Promise<void>;
    cleanJob: () => Promise<void>;
    getJobLogs: () => Promise<string[]>;
  };
}

const greenStatuses = [STATUSES.Done, STATUSES.Running];

export const JobCard = ({ job, status, actions, readOnlyMode, allowRetries }: JobCardProps) => (
  <div className={s.card}>
    <div className={s.sideInfo}>
      <span title={`#${job.context.id}`}>#{job.context.id}</span>
      <Timeline job={job} status={status} />
    </div>
    <div className={s.contentWrapper}>
      <div className={s.title}>
        <h4>
          {job.context.name}
          {job.context.attempts > 1 && <span>attempt #{job.context.attempts}</span>}
          {!!job.opts?.repeat?.count && (
            <span>
              repeat {job.opts?.repeat?.count}
              {!!job.opts?.repeat?.limit && ` / ${job.opts?.repeat?.limit}`}
            </span>
          )}
        </h4>
        {!readOnlyMode && (
          <JobActions status={status} actions={actions} allowRetries={allowRetries} />
        )}
      </div>
      <div className={s.content}>
        <Details status={status} job={job} actions={actions} />
        {typeof job.context.progress === 'number' && (
          <Progress
            percentage={job.context.progress}
            status={
              job.context.last_error && !greenStatuses.includes(status as any)
                ? STATUSES.Failed
                : status
            }
            className={s.progress}
          />
        )}
      </div>
    </div>
  </div>
);
