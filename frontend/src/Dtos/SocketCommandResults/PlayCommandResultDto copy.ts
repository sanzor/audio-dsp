

import type { PLAY_COMMAND } from '@/Events';
import type { BaseCommandResultDto } from './BaseCommandResultDto';

export interface PlayCommandResultDto extends BaseCommandResultDto{
    command:typeof PLAY_COMMAND;

};