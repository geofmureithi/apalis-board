import React, { useState } from 'react';
import { Store } from '../../hooks/useStore';
import { JobCard } from '../JobCard/JobCard';
import { QueueActions } from '../QueueActions/QueueActions';
import { StatusMenu } from '../StatusMenu/StatusMenu';
import s from './QueuePage.module.css';
import { AppJob, AppQueue, JobCounts, Status } from '../../../typings/app';
import { Api } from '../../services/Api';
import { useInterval } from '../../hooks/useInterval';
import { useSelectedStatuses } from '../../hooks/useSelectedStatuses';
import { useLocation } from 'react-router-dom';
import WorkerCard from '../WorkerCard/WorkerCard';
import { ApalisWorker } from '../../../typings/app';
import { Pagination } from '../Pagination/Pagination';

const interval = 5000;
export const QueuePage = ({
  selectedStatus,
  actions,
  queue,
  api,
}: {
  queue: AppQueue | undefined;
  actions: Store['actions'];
  selectedStatus: Store['selectedStatuses'];
  api: Api;
}) => {
  if (!queue) {
    return <section>Queue Not found</section>;
  }

  const { search } = useLocation();
  const query = new URLSearchParams(search);
  const isDetailsTab = !query.get('status');

  const [jobRes, setJobRes] = useState<{
    data: { jobs: AppJob[]; counts: JobCounts } | null;
    loading: boolean;
  }>({
    data: null,
    loading: true,
  });

  const [workers, setWorkers] = useState<{ data: ApalisWorker[] | null; loading: boolean }>({
    data: null,
    loading: true,
  });

  const selectedStatuses = useSelectedStatuses();

  const updateJobs = () =>
    api
      .getJobsByQueue(queue.name, {
        page: query.get('page') || '1',
        status: selectedStatus[queue.name],
      })
      .then((data) => {
        setJobRes({ data, loading: false });
      })
      // eslint-disable-next-line no-console
      .catch((error) => console.error('Failed to poll', error));
  const updateWorkers = () =>
    api
      .getQueueWorkers(queue.name)
      .then((data) => {
        setWorkers({ data, loading: false });
      })
      // eslint-disable-next-line no-console
      .catch((error) => console.error('Failed to poll', error));
  function update() {
    if (isDetailsTab) updateWorkers();
    updateJobs();
  }

  useInterval(update, interval, [selectedStatuses]);
  const currentStatus: Status | null = query.get('status') as any;
  const pages = currentStatus ? (jobRes.data?.counts[currentStatus] || 0) / 10 : 0;

  return (
    <section className={s.wrapper}>
      <div className={s.stickyHeader}>
        <StatusMenu queue={queue} actions={actions} counts={jobRes.data?.counts} />
        <div className={s.actionContainer}>
          <div>
            {jobRes.data?.jobs && jobRes.data?.jobs.length > 0 && !queue.readOnlyMode && (
              <QueueActions
                queue={queue}
                actions={actions}
                status={selectedStatus[queue.name]}
                allowRetries={queue.allowRetries}
              />
            )}
          </div>
          {!isDetailsTab && <Pagination pageCount={pages} />}
        </div>
      </div>
      {isDetailsTab &&
        workers.data?.map((w) => (
          <WorkerCard
            key={w.worker_id}
            {...w}
            jobs={jobRes.data?.jobs?.filter((j) => j.context.lock_by == w.worker_id)}
          />
        ))}
      {!isDetailsTab &&
        jobRes.data?.jobs.map((job) => (
          <JobCard
            key={job.context.id}
            job={job}
            status={selectedStatus[queue.name]}
            actions={{
              cleanJob: actions.cleanJob(queue?.name)(job),
              promoteJob: actions.promoteJob(queue?.name)(job),
              retryJob: actions.retryJob(queue?.name)(job),
              getJobLogs: actions.getJobLogs(queue?.name)(job),
            }}
            readOnlyMode={queue?.readOnlyMode || false}
            allowRetries={true}
          />
        ))}
    </section>
  );
};
