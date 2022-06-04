import React from 'react';
import { NavLink, useRouteMatch } from 'react-router-dom';
import s from './StatusMenu.module.css';
import { AppQueue, JobCounts } from '../../../typings/app';
import { STATUS_LIST } from '../../constants/status-list';
import { Store } from '../../hooks/useStore';
import { QueueDropdownActions } from '../QueueDropdownActions/QueueDropdownActions';

export const StatusMenu = ({
  queue,
  counts,
  actions,
}: {
  queue: AppQueue;
  counts?: JobCounts | null;
  actions: Store['actions'];
}) => {
  const { url } = useRouteMatch();

  return (
    <div className={s.statusMenu}>
      <NavLink
        to={`${url}`}
        activeClassName={s.active}
        isActive={(_path, { search }) => {
          const query = new URLSearchParams(search);
          return query.get('status') === null;
        }}
      >
        <span title={'Home'}>🏠 Home</span>
      </NavLink>
      {STATUS_LIST.map((status) => {
        const displayStatus = status;
        return (
          <NavLink
            to={`${url}${`?status=${status}`}`}
            activeClassName={s.active}
            isActive={(_path, { search }) => {
              const query = new URLSearchParams(search);
              return query.get('status') === status;
            }}
            key={`${queue.name}-${status}`}
          >
            <span title={displayStatus}>{displayStatus}</span>
            {counts && counts[status] > 0 && <span className={s.badge}>{counts[status]}</span>}
          </NavLink>
        );
      })}
      {!queue.readOnlyMode && (
        <div>
          <QueueDropdownActions queue={queue} actions={actions} />
        </div>
      )}
    </div>
  );
};
