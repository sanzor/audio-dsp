

import type {  STOP_COMMAND } from '@/Events';
import type { BaseCommandResultDto } from './BaseCommandResultDto';

export interface StopCommandResultDto extends BaseCommandResultDto{
    command:typeof STOP_COMMAND;

};