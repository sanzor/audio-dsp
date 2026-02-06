

import type { SEEK_COMMAND } from '@/Events';
import type { BaseCommandResultDto } from './BaseCommandResultDto';

export interface SeekCommandResultDto extends BaseCommandResultDto{
    command:typeof SEEK_COMMAND;

};