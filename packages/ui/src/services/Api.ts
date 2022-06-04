import { AppJob, Status } from '../../typings/app';
import { GetQueuesResponse } from '../../typings/responses';
import Axios, { AxiosInstance, AxiosResponse } from 'axios';
import { toast } from 'react-toastify';
import { ApalisWorker } from '../../typings/app';

export class Api {
  private axios: AxiosInstance;

  constructor({ basePath }: { basePath: string } = { basePath: '' }) {
    this.axios = Axios.create({ baseURL: `${basePath}/api` });
    this.axios.interceptors.response.use(this.handleResponse, this.handleError);
  }

  public getQueues(): Promise<GetQueuesResponse> {
    return this.axios.get(`/queues`);
  }

  public getQueueWorkers(queueName: string): Promise<ApalisWorker[]> {
    return this.axios.get(`/queues/${encodeURIComponent(queueName)}/workers`);
  }

  public getJobsByQueue(
    queueName: string,
    { page, status }: { page: string; status: Status }
  ): Promise<{ jobs: AppJob[]; counts: any }> {
    return this.axios.get(`/queues/${encodeURIComponent(queueName)}`, { params: { page, status } });
  }

  public retryAll(queueName: string): Promise<void> {
    return this.axios.put(`/queues/${encodeURIComponent(queueName)}/retry`);
  }

  public cleanAllDelayed(queueName: string): Promise<void> {
    return this.axios.put(`/queues/${encodeURIComponent(queueName)}/clean/delayed`);
  }

  public cleanAllFailed(queueName: string): Promise<void> {
    return this.axios.put(`/queues/${encodeURIComponent(queueName)}/clean/failed`);
  }

  public cleanAllCompleted(queueName: string): Promise<void> {
    return this.axios.put(`/queues/${encodeURIComponent(queueName)}/clean/completed`);
  }

  public cleanJob(queueName: string, jobId: AppJob['context']['id']): Promise<void> {
    return this.axios.put(
      `/queues/${encodeURIComponent(queueName)}/${encodeURIComponent(`${jobId}`)}/clean`
    );
  }

  public retryJob(queueName: string, jobId: AppJob['context']['id']): Promise<void> {
    return this.axios.put(
      `/queues/${encodeURIComponent(queueName)}/${encodeURIComponent(`${jobId}`)}/retry`
    );
  }

  public promoteJob(queueName: string, jobId: AppJob['context']['id']): Promise<void> {
    return this.axios.put(
      `/queues/${encodeURIComponent(queueName)}/${encodeURIComponent(`${jobId}`)}/promote`
    );
  }

  public getJobLogs(queueName: string, jobId: AppJob['context']['id']): Promise<string[]> {
    return this.axios.get(
      `/queues/${encodeURIComponent(queueName)}/${encodeURIComponent(`${jobId}`)}/logs`
    );
  }

  public pauseQueue(queueName: string) {
    return this.axios.put(`/queues/${encodeURIComponent(queueName)}/pause`);
  }

  public resumeQueue(queueName: string) {
    return this.axios.put(`/queues/${encodeURIComponent(queueName)}/resume`);
  }

  private handleResponse(response: AxiosResponse): any {
    return response.data;
  }

  private async handleError(error: { response: AxiosResponse }): Promise<any> {
    if (error.response.data?.error) {
      toast.error(error.response.data?.error, { autoClose: 5000 });
    }

    return Promise.resolve(error.response.data);
  }
}
