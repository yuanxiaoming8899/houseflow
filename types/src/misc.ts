export interface DateTime {
  hour: number;
  minute: number;
  second: number;
}

export type State = {
  state: boolean;
};

export enum CloudTopics {
  DEVICE_DATA = 'device_data',
  DEVICE_DISCONNECT = 'device_disconnect',
  DEVICE_REQUEST = 'device_request',
}
