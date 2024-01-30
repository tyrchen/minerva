import { BaseAssistant } from './index';
import { OpenAI } from 'openai';

export class SqlSuggestionAssistant extends BaseAssistant {
  table = '';

  constructor(table: string) {
    super();
    this.table = table;
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
        content: `Given the following SQL table, please think of at least 10 different SQL statements that could help to best understand the data in the table.

        Table:

        ${this.table}

        Please response with a json containing a list of {"purpose: "the purpose of the query", "sql": "the actual query"}.
        `,
      },
    ] as OpenAI.ChatCompletionMessageParam[];
  }
}
