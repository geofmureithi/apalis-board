import React from 'react';
import { Redirect, Route, Switch } from 'react-router-dom';
import { ToastContainer } from 'react-toastify';
import { useScrollTopOnNav } from '../hooks/useScrollTopOnNav';
import { useStore } from '../hooks/useStore';
import { Api } from '../services/Api';
import { Header } from './Header/Header';
import { Menu } from './Menu/Menu';
import { QueuePage } from './QueuePage/QueuePage';
import { RedisStats } from './RedisStats/RedisStats';
import { ConfirmModal } from './ConfirmModal/ConfirmModal';

export const App = ({ api }: { api: Api }) => {
  useScrollTopOnNav();
  const store = useStore(api);
  // eslint-disable-next-line no-console
  const { state, actions, selectedStatuses, confirmProps } = store;
  return (
    <>
      <Header>{state.data?.stats && <RedisStats stats={state.data?.stats} />}</Header>
      <main>
        <div>
          {state.loading ? (
            'Loading...'
          ) : (
            <>
              <Switch>
                <Route
                  path="/queue/:name"
                  render={({ match: { params } }) => {
                    const currentQueueName = decodeURIComponent(params.name);
                    const queue = state.data?.queues?.find((q) => q.url === currentQueueName);

                    return (
                      <QueuePage
                        queue={queue}
                        actions={actions}
                        selectedStatus={selectedStatuses}
                      />
                    );
                  }}
                />

                <Route path="/" exact>
                  {!!state.data &&
                    Array.isArray(state.data?.queues) &&
                    state.data.queues.length > 0 && (
                      <Redirect
                        to={`/queue/${encodeURIComponent(state.data?.queues[0].url || '')}`}
                      />
                    )}
                </Route>
              </Switch>
              <ConfirmModal {...confirmProps} />
            </>
          )}
        </div>
      </main>
      <Menu queues={state.data?.queues} selectedStatuses={selectedStatuses} />
      <ToastContainer />
    </>
  );
};
