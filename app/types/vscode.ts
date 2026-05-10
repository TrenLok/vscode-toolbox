export type VSCodeVersionChannelType = 'insider' | 'stable' | 'vscodium';

export interface VSCodeVersion {
  version?: string;
  commit?: string;
  architecture?: string;
  channel?: VSCodeVersionChannelType;
}
