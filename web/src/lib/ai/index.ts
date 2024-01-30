import { OpenAI } from 'openai';
import yaml from 'js-yaml';

// TODO: API key
const openai = new OpenAI.OpenAI({ dangerouslyAllowBrowser: true });
export class BaseAssistant {
  name = 'My assistant';
  temperature = 0.0;
  model = 'gpt-3.5-turbo-1106';
  modelMaxTokens = 4096;
  format = 'text' as 'text' | 'json_object';

  constructor() {}

  getInstructions(): OpenAI.ChatCompletionMessageParam[] {
    return [];
  }

  getAvailableFunctions() {
    return [];
  }

  callFunction(_fnName: string, _args: object) {
    return null;
  }

  async getCompletion(msgs?: OpenAI.ChatCompletionMessageParam[]): Promise<string | object | null> {
    const messages = this.getInstructions().concat(msgs || []);
    console.log('messages:', messages);
    const completion = await openai.chat.completions.create({
      messages,
      max_tokens: this.modelMaxTokens,
      temperature: this.temperature,
      model: this.model,
      n: 1,
      response_format: { type: this.format },
    });

    const text = completion.choices[0].message.content;

    if (this.format === 'json_object') {
      return JSON.parse(text || '{}');
    } else {
      return text;
    }
  }

  parseYaml(data: string): object {
    return yaml.load(data.replace(/```+[^\n]*\n?/g, ''));
  }
}
