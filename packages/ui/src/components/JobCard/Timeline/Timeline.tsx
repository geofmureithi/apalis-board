import { format, formatDistance, getYear, isToday } from 'date-fns';
import React from 'react';
import s from './Timeline.module.css';
import { AppJob, Status } from '@bull-board/api/typings/app';
import { STATUSES } from '@bull-board/api/src/constants/statuses';

//type TimeStamp = number | Date;

const formatDate = (date: any) => {
  const ts = new Date(date);
  if (isToday(ts)) {
    return format(ts, 'HH:mm:ss');
  }

  return getYear(ts) === getYear(new Date())
    ? format(ts, 'MM/dd HH:mm:ss')
    : format(ts, 'MM/dd/yyyy HH:mm:ss');
};

export const Timeline = function Timeline({ job, status }: { job: AppJob; status: Status }) {
  return (
    <div className={s.timelineWrapper}>
      <ul className={s.timeline}>
        <li>
          <small>Added at</small>
          <time>{formatDate(job.run_at || 0)}</time>
        </li>
        {!!job.delay && job.delay > 0 && status === STATUSES.Scheduled && (
          <li>
            <small>Will run at</small>
            <time>{formatDate((job.run_at || 0) + job.delay)}</time>
          </li>
        )}
        {!!job.lock_at && (
          <li>
            <small>
              {job.delay && job.delay > 0 ? 'delayed for ' : ''}
              {formatDistance(new Date(job.lock_at), new Date(job.run_at || 0), {
                includeSeconds: true,
              })}
            </small>
            <small>Process started at</small>
            <time>{formatDate(job.lock_at)}</time>
          </li>
        )}
        {!!job.done_at && (
          <li>
            <small>
              {formatDistance(new Date(job.done_at), new Date(job.lock_at || 0), {
                includeSeconds: true,
              })}
            </small>
            <small>
              {job.last_error && status !== STATUSES.Running ? 'Failed' : 'Finished'} at
            </small>
            <time>{formatDate(job.done_at)}</time>
          </li>
        )}
      </ul>
    </div>
  );
};
