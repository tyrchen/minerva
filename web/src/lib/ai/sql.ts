import { BaseAssistant } from './index';
import { OpenAI } from 'openai';

export class SqlAssistant extends BaseAssistant {
  table = '';
  instruction = '';

  constructor(table: string, instruction: string) {
    super();
    this.table = table;
    this.instruction = instruction;
    this.format = 'json_object';
  }

  getInstructions() {
    return [
      {
        role: 'system',
        content:
          'You are an SQL and data analysis expert, you could write complex and efficient SQL to do data analysis',
      },
      {
        role: 'user',
        content: `Given the following SQL table, please write an SQL statement to query the data to fulfill the instructions. Please return a json contains a 'sql' field with the sql statement. Please take a deep breath and think step by step.

        Table creation statement:

        ${this.table}

        Instructions:

        ${this.instruction}
        `,
      },
    ] as OpenAI.ChatCompletionMessageParam[];
  }
}
