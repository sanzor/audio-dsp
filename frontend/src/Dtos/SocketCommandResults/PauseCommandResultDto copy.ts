

import type { PAUSE_COMMAND } from '@/Events';
import type { BaseCommandResultDto } from './BaseCommandResultDto';

export interface PauseCommandResultDto extends BaseCommandResultDto{
    command:typeof PAUSE_COMMAND;

};