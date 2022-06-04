import { AppJob } from '../../../typings/app';
import { formatDistance } from 'date-fns';
import React from 'react';

import s from './WorkerCard.module.css';

export interface IWorkerCardProps {
  worker_id: string;
  job_type: string;
  source: string;
  layers: string;
  last_seen: string;
  jobs: AppJob[] | undefined;
}

export default function WorkerCard(props: IWorkerCardProps) {
  const lastSeen = new Date(props.last_seen);
  const fiveMinuteAgo = new Date(Date.now() - 1000 * 60 * 5);
  return (
    <div className={s.tile}>
      <div className={s.tileWrapper}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width={24}
          height={24}
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth={2}
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <rect x={3} y={3} width={18} height={18} rx={2} ry={2} />
          <path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3" />
          <path d="M9 11.2h5.7" />
        </svg>

        <h4>Worker #{props.worker_id.slice(0, 8)}</h4>
        <span
          className={`${s.lastSeen} ${lastSeen > fiveMinuteAgo ? s.lastSeenActive : ''} ${
            props.jobs?.length ? s.lastSeenRunning : ''
          }`}
        ></span>
        <span className={s.lastSeenTime}>Last seen {formatDistance(lastSeen, new Date())} ago</span>
      </div>
    </div>
  );
}
