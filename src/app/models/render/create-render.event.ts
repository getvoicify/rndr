type CreateRenderStatus = 'idle' | 'loading' | 'error' | 'success';

export interface CreateRenderConfig {
  scene: string;
  samples: number;
  percentage: number;
  startframe: number;
  endframe: number;
  breakpoint?: number;
}

interface ICreateRenderEvent {
  status: CreateRenderStatus;
}

interface CreateRenderSuccessEvent extends ICreateRenderEvent {
  status: 'success';
}

interface CreateRenderErrorEvent extends ICreateRenderEvent {
  status: 'error';
  error: Error;
}

interface CreateRenderLoadingEvent extends ICreateRenderEvent {
  status: 'loading';
  payload: {
    file: File | string;
    config: CreateRenderConfig;
  }
}

interface CreateRenderIdleEvent extends ICreateRenderEvent {
  status: 'idle';
}

export type CreateRenderEvent = CreateRenderSuccessEvent | CreateRenderErrorEvent | CreateRenderLoadingEvent | CreateRenderIdleEvent;

export const isLoadRenderEvent = (event: CreateRenderEvent): event is CreateRenderLoadingEvent => event.status === 'loading';
export const isIdleRenderEvent = (event: CreateRenderEvent): event is CreateRenderIdleEvent => event.status === 'idle';
export const isErrorRenderEvent = (event: CreateRenderEvent): event is CreateRenderErrorEvent => event.status === 'error';
export const isSuccessRenderEvent = (event: CreateRenderEvent): event is CreateRenderSuccessEvent => event.status === 'success';